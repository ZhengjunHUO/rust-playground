use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
//use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::web::{self, Data, Query};
use actix_web::{cookie, get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use derive_more::Display;
use openidconnect::core::{CoreAuthenticationFlow, CoreProviderMetadata};
use openidconnect::{
    reqwest, AccessToken, AccessTokenHash, AdditionalClaims, AuthorizationCode, Client, ClientId,
    ClientSecret, CsrfToken, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, Pool, Postgres, Row};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use time::Duration;

type OIDCClient = openidconnect::Client<
    //openidconnect::EmptyAdditionalClaims,
    OIDCAdditionClaims,
    openidconnect::core::CoreAuthDisplay,
    openidconnect::core::CoreGenderClaim,
    openidconnect::core::CoreJweContentEncryptionAlgorithm,
    openidconnect::core::CoreJsonWebKey,
    openidconnect::core::CoreAuthPrompt,
    openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
    openidconnect::StandardTokenResponse<
        openidconnect::IdTokenFields<
            //openidconnect::EmptyAdditionalClaims,
            OIDCAdditionClaims,
            openidconnect::EmptyExtraTokenFields,
            openidconnect::core::CoreGenderClaim,
            openidconnect::core::CoreJweContentEncryptionAlgorithm,
            openidconnect::core::CoreJwsSigningAlgorithm,
        >,
        openidconnect::core::CoreTokenType,
    >,
    openidconnect::StandardTokenIntrospectionResponse<
        openidconnect::EmptyExtraTokenFields,
        openidconnect::core::CoreTokenType,
    >,
    openidconnect::core::CoreRevocableToken,
    openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>,
    openidconnect::EndpointSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointMaybeSet,
    openidconnect::EndpointMaybeSet,
>;

#[derive(Serialize, Deserialize, Debug)]
struct OIDCAdditionClaims {
    oidc_id: Option<String>,
    oidc_role: Option<String>,
}

impl AdditionalClaims for OIDCAdditionClaims {}

struct AppState {
    oidc_client: Arc<OIDCClient>,
    //pkce: Mutex<Option<Pkce>>,
}

#[derive(Serialize, Deserialize)]
struct Pkce {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
    nonce: Nonce,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionData {
    user_id: String,
    access_token: AccessToken,
    refresh_token: RefreshToken,
    expires_in: DateTime<Utc>,
    oidc_id: Option<String>,
    oidc_role: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OpCallback {
    pub state: String,
    pub session_state: String,
    pub code: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let cookie_key = cookie::Key::generate();
    //println!("cookie key: {}", String::from_utf8_lossy(cookie_key.master()));
    let cookie_key = "GM4>?/%PNes8x[{5Cz$Y7ztOnF/tJ<=lQWLjr9J0:$|k*p6D)Bv)j%IDQ19!=BQz";

    let client = Arc::new(init_oidcclient().await);
    let psql_store = PsqlSessionStore::new("postgres://postgres:admin@127.0.0.1:5432/oidc")
        .await
        .expect("Error init psql session store");

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    //CookieSessionStore::default(),
                    psql_store.clone(),
                    cookie::Key::from(cookie_key.as_bytes()),
                )
                .cookie_secure(false)
                .build(),
            )
            .app_data(Data::new(AppState {
                oidc_client: client.clone(),
                //pkce: Mutex::new(None),
            }))
            .service(login)
            .service(callback)
            .service(userinfo)
            .service(logout)
            .default_service(web::to(forward_handler))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

async fn init_oidcclient() -> OIDCClient {
    let http_client = init_http_client().expect("Error initing http client");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("http://localhost:8080/realms/oidc".to_string())
            .expect("Error setting issuer url"),
        &http_client,
    )
    .await
    .expect("Error grabbing provider metadata");

    //openidconnect::core::CoreClient::from_provider_metadata(
    Client::from_provider_metadata(
        provider_metadata,
        ClientId::new("oidc-backend".to_owned()),
        Some(ClientSecret::new(
            "8cHU783LSC839uhapouji3dHJ34N32SC".to_owned(),
        )),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:8888/callback".to_owned())
            .expect("Error setting redirect url"),
    )
}

#[get("/login")]
async fn login(data: Data<AppState>, session: Session) -> Result<HttpResponse, Error> {
    // Generate PKCE challenge
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Prepare full authorization URL, to which the user should be redirected
    let (auth_url, csrf_token, nonce) = data
        .oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("oidc".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    // {
    //     let mut pkce = data.pkce.lock().unwrap();
    //     *pkce = Some(Pkce { pkce_verifier, csrf_token, nonce});
    // }

    let pkce = Pkce {
        pkce_verifier,
        csrf_token,
        nonce,
    };
    session.insert("pkce", &pkce)?;

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", auth_url.as_str()))
        .finish())
}

#[get("/callback")]
async fn callback(
    data: Data<AppState>,
    session: Session,
    authorization_info: Query<OpCallback>,
) -> Result<HttpResponse, Error> {
    // Now have access to the authorization code
    // should verify that the `state` parameter returned by the server matches `csrf_state`.
    if let Some(pkce) = session.get::<Pkce>("pkce")? {
        let http_client = init_http_client().expect("Error initing http client");

        // exchange authorization code for an access token and ID token
        let token_response = data
            .oidc_client
            .exchange_code(AuthorizationCode::new(authorization_info.code.to_string()))
            .expect("Error setting exchange code")
            .set_pkce_verifier(pkce.pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|_| OIDCError::ExchangeAccessToken)?;

        // Extract the ID token claims after verifying its authenticity and nonce
        let id_token = token_response.id_token().ok_or(OIDCError::ExtractIDToken)?;
        let id_token_verifier = data.oidc_client.id_token_verifier();
        let claims = id_token
            .claims(&id_token_verifier, &pkce.nonce)
            .map_err(|_| OIDCError::GetIDTokenClaim)?;

        // Verify the access token hash to ensure that the access token hasn't been substituted
        match claims.access_token_hash() {
            Some(expected_access_token_hash) => {
                let actual_access_token_hash = AccessTokenHash::from_token(
                    token_response.access_token(),
                    id_token
                        .signing_alg()
                        .map_err(|_| OIDCError::TokenSigning)?,
                    id_token
                        .signing_key(&id_token_verifier)
                        .map_err(|_| OIDCError::TokenSigning)?,
                )
                .map_err(|_| OIDCError::TokenSigning)?;

                if actual_access_token_hash != *expected_access_token_hash {
                    Err(OIDCError::VerifyToken)
                } else {
                    Ok(())
                }
            }
            None => Err(OIDCError::VerifyToken),
        }?;

        println!(
            "User {} with e-mail address {} has authenticated successfully.\nAdditional Claims: [id] {:?} ; [role] {:?}",
            claims.subject().as_str(),
            claims
                .email()
                .map(|email| email.as_str())
                .unwrap_or("<not provided>"),
            claims.additional_claims().oidc_id,
            claims.additional_claims().oidc_role,
        );

        let cred = SessionData {
            user_id: claims.subject().to_string(),
            access_token: token_response.access_token().to_owned(),
            refresh_token: token_response
                .refresh_token()
                .expect("Error retrieving refresh token")
                .to_owned(),
            expires_in: chrono::Utc::now()
                + chrono::Duration::seconds(
                    token_response
                        .expires_in()
                        .expect("Error retrieving expires_in duration")
                        .as_secs()
                        .try_into()
                        .unwrap(),
                ),
            oidc_id: claims.additional_claims().oidc_id.clone(),
            oidc_role: claims.additional_claims().oidc_role.clone(),
        };

        let session_id = uuid::Uuid::new_v4().to_string();
        println!("session_id: {}", session_id);
        session.insert(session_id.clone(), &cred)?;
        session.remove("pkce");

        Ok(HttpResponse::Ok()
            .append_header((
                "Set-Cookie",
                format!("session={}; HttpOnly; Secure; SameSite=Strict", session_id),
            ))
            .body("Success"))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[get("/userinfo")]
async fn userinfo(
    req: HttpRequest,
    data: Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    //println!("Request cookies: {:?}", req.cookies());
    if let Some(session_cookie) = req.cookie("session") {
        let session_id = session_cookie.value();
        if let Some(cred) = session.get::<SessionData>(session_id)? {
            let output = format!(
                "Session verified, token expire in: {:?} for user {}",
                cred.expires_in, cred.user_id
            );
            println!("{}", output);

            if Utc::now() > cred.expires_in {
                println!(
                    "Access token expired ({} > {})",
                    Utc::now(),
                    cred.expires_in
                );
                match try_refresh(data, cred).await {
                    Ok(cred_updated) => {
                        println!("Cred after update: {:?}", cred_updated);
                        session.insert(session_id, cred_updated)?;
                    }
                    Err(err) => {
                        println!("Refresh error: {:?}", err);
                        session.remove(session_id);
                        return Ok(HttpResponse::SeeOther()
                            .insert_header(("Location", "http://127.0.0.1:8888/login"))
                            .finish());
                    }
                }
            }

            return Ok(HttpResponse::Ok().body(output));
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

#[get("/logout")]
async fn logout(req: HttpRequest, session: Session) -> Result<HttpResponse, Error> {
    if let Some(_session_cookie) = req.cookie("session") {
        session.purge();
        // let session_id = session_cookie.value();
        // if session.remove(session_id).is_some() {
        //     println!("[{}] Session data cleaned.", session_id);
        // }
        return Ok(HttpResponse::SeeOther().insert_header(("Location", "http://localhost:8080/realms/oidc/protocol/openid-connect/logout?redirect_uri=http://127.0.0.1:8888/login")).finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
}

async fn forward_handler(req: HttpRequest, session: Session) -> impl Responder {
    if let Some(session_cookie) = req.cookie("session") {
        let session_id = session_cookie.value();
        if let Some(cred) = session.get::<SessionData>(session_id).unwrap() {
            let http_client = init_http_client().expect("Error initing http client");
            // TODO make it more generic
            let url = format!("http://127.0.0.1:8001{}", req.path());
            println!("url: {}", url);
            match http_client
                .get(url)
                .bearer_auth(cred.access_token.into_secret())
                .send()
                .await
            {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let body = resp.bytes().await.unwrap_or_default();

                    return HttpResponse::build(
                        actix_web::http::StatusCode::from_u16(status).unwrap(),
                    )
                    .content_type("application/octet-stream")
                    .body(body);
                }
                Err(error) => {
                    println!("Error occurred proxying the req: {:?}", error);
                    return HttpResponse::InternalServerError()
                        .body("Error occurred proxying the req");
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
    // HttpResponse::NotFound()
    //     .content_type("text/plain")
    //     .body(format!("No route found for: {}", req.path()))
}

async fn try_refresh(data: Data<AppState>, cred: SessionData) -> anyhow::Result<SessionData> {
    let http_client = init_http_client().expect("Error initing http client");

    let token_response = data
        .oidc_client
        .exchange_refresh_token(&cred.refresh_token)?
        .request_async(&http_client)
        .await?;
    Ok(SessionData {
        user_id: cred.user_id,
        access_token: token_response.access_token().to_owned(),
        refresh_token: token_response
            .refresh_token()
            .expect("Error retrieving refresh token")
            .to_owned(),
        expires_in: chrono::Utc::now()
            + chrono::Duration::seconds(
                token_response
                    .expires_in()
                    .expect("Error retrieving expires_in duration")
                    .as_secs()
                    .try_into()
                    .unwrap(),
            ),
        oidc_id: cred.oidc_id,
        oidc_role: cred.oidc_role,
    })
}

fn init_http_client() -> reqwest::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

#[derive(Debug, Display)]
enum OIDCError {
    #[display("Failed to exchange access token with authorization code")]
    ExchangeAccessToken,
    #[display("Failed to extract id token from token response")]
    ExtractIDToken,
    #[display("Failed to get id token claim")]
    GetIDTokenClaim,
    #[display("Failed to verify token signing")]
    TokenSigning,
    #[display("Failed to verify token")]
    VerifyToken,
}

impl actix_web::error::ResponseError for OIDCError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

type SessionState = HashMap<String, String>;
type PsqlPool = Pool<Postgres>;

#[derive(Clone)]
struct CacheConfiguration {
    cache_keygen: Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            cache_keygen: Arc::new(str::to_owned),
        }
    }
}

#[derive(Clone)]
struct PsqlSessionStore {
    client: Arc<PsqlPool>,
    config: CacheConfiguration,
}

impl PsqlSessionStore {
    async fn new(conn_str: &str) -> Result<Self, anyhow::Error> {
        Ok(PsqlSessionStore {
            client: Arc::new(
                PgPoolOptions::new()
                    .max_connections(1)
                    .connect(conn_str)
                    .await?,
            ),
            config: CacheConfiguration::default(),
        })
    }
}

impl SessionStore for PsqlSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let key = (self.config.cache_keygen)(session_key.as_ref());
        let rslt = query("SELECT session_state FROM sessions WHERE key = $1 AND expires > NOW()")
            .bind(key)
            .fetch_optional(self.client.as_ref())
            .await
            .map_err(Into::into)
            .map_err(LoadError::Other)?;
        if let Some(row) = rslt {
            let val = row.get("session_state");
            let session_state = serde_json::from_value(val)
                .map_err(Into::into)
                .map_err(LoadError::Deserialization)?;
            return Ok(session_state);
        }
        Ok(None)
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        let session_key: SessionKey = uuid::Uuid::new_v4().to_string().try_into().unwrap();
        let key = (self.config.cache_keygen)(session_key.as_ref());
        let val = serde_json::to_value(&session_state)
            .map_err(Into::into)
            .map_err(SaveError::Serialization)?;
        let exp = chrono::Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        query("INSERT INTO sessions(key, session_state, expires) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(key)
            .bind(val)
            .bind(exp)
            .execute(self.client.as_ref())
            .await
            .map_err(Into::into)
            .map_err(SaveError::Other)?;
        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let key = (self.config.cache_keygen)(session_key.as_ref());
        let val = serde_json::to_value(&session_state)
            .map_err(Into::into)
            .map_err(UpdateError::Serialization)?;
        let exp = chrono::Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        query("UPDATE sessions SET session_state = $1, expires = $2 WHERE key = $3")
            .bind(val)
            .bind(exp)
            .bind(key)
            .execute(self.client.as_ref())
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(session_key)
    }

    async fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> Result<(), anyhow::Error> {
        let key = (self.config.cache_keygen)(session_key.as_ref());
        let exp = chrono::Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        query("UPDATE sessions SET expires = $1 WHERE key = $2")
            .bind(exp)
            .bind(key)
            .execute(self.client.as_ref())
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let key = (self.config.cache_keygen)(session_key.as_ref());
        query("DELETE FROM sessions WHERE key = $1")
            .bind(key)
            .execute(self.client.as_ref())
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(())
    }
}
