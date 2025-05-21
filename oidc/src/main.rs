use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::web::{Data, Query};
use actix_web::{cookie, get, App, Error, HttpRequest, HttpResponse, HttpServer};
use derive_more::Display;
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::{
    reqwest, AccessToken, AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    RefreshToken, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

type OIDCClient = openidconnect::Client<
    openidconnect::EmptyAdditionalClaims,
    openidconnect::core::CoreAuthDisplay,
    openidconnect::core::CoreGenderClaim,
    openidconnect::core::CoreJweContentEncryptionAlgorithm,
    openidconnect::core::CoreJsonWebKey,
    openidconnect::core::CoreAuthPrompt,
    openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
    openidconnect::StandardTokenResponse<
        openidconnect::IdTokenFields<
            openidconnect::EmptyAdditionalClaims,
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

#[derive(Serialize, Deserialize)]
struct SessionData {
    user_id: String,
    access_token: AccessToken,
    refresh_token: RefreshToken,
    expires_in: Duration,
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
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
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
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

async fn init_oidcclient() -> OIDCClient {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Error initing http client");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("http://localhost:8080/realms/tresor".to_string())
            .expect("Error setting issuer url"),
        &http_client,
    )
    .await
    .expect("Error grabbing provider metadata");

    CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("tresor-backend".to_owned()),
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
        .add_scope(Scope::new("tresor".to_string()))
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
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Error initing http client");

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
            "User {} with e-mail address {} has authenticated successfully",
            claims.subject().as_str(),
            claims
                .email()
                .map(|email| email.as_str())
                .unwrap_or("<not provided>"),
        );

        let cred = SessionData {
            user_id: claims.subject().to_string(),
            access_token: token_response.access_token().to_owned(),
            refresh_token: token_response
                .refresh_token()
                .expect("Error retrieving refresh token")
                .to_owned(),
            expires_in: token_response
                .expires_in()
                .expect("Error retrieving expires_in duration"),
        };
        let session_id = uuid::Uuid::new_v4().to_string();
        println!("session_id: {}", session_id);
        session.insert(session_id.clone(), &cred)?;

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
async fn userinfo(req: HttpRequest, session: Session) -> Result<HttpResponse, Error> {
    //println!("Request cookies: {:?}", req.cookies());
    if let Some(session_cookie) = req.cookie("session") {
        let session_id = session_cookie.value();
        if let Some(cred) = session.get::<SessionData>(session_id)? {
            println!(
                "Session verified, token expire in: {:?} for user {}",
                cred.expires_in, cred.user_id
            );
            return Ok(HttpResponse::Ok().body("Success"));
        }
    }
    Ok(HttpResponse::Unauthorized().finish())
}

#[get("/logout")]
async fn logout(req: HttpRequest, session: Session) -> Result<HttpResponse, Error> {
    if let Some(session_cookie) = req.cookie("session") {
        let session_id = session_cookie.value();
        if session.remove(session_id).is_some() {
            println!("[{}] Session data cleaned.", session_id);
        }
        return Ok(HttpResponse::SeeOther().insert_header(("Location", "http://localhost:8080/realms/tresor/protocol/openid-connect/logout?redirect_uri=http://127.0.0.1:8888/login")).finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
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
