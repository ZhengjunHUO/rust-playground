use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::web::{Data, Query};
use actix_web::{cookie, get, App, Error, HttpResponse, HttpServer};
use derive_more::Display;
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::{
    reqwest, AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl,
    Nonce, OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("tresor-backend".to_owned()),
        Some(ClientSecret::new(
            "8cHU783LSC839uhapouji3dHJ34N32SC".to_owned(),
        )),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:8888/callback".to_owned())
            .expect("Error setting redirect url"),
    );

    return client;
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
            .map_err(|_| OIDCError::ExchangeAccessTokenError)?;

        // Extract the ID token claims after verifying its authenticity and nonce
        let id_token = token_response
            .id_token()
            .ok_or_else(|| OIDCError::ExtractIDTokenError)?;
        let id_token_verifier = data.oidc_client.id_token_verifier();
        let claims = id_token
            .claims(&id_token_verifier, &pkce.nonce)
            .map_err(|_| OIDCError::GetIDTokenClaimError)?;

        // Verify the access token hash to ensure that the access token hasn't been substituted
        match claims.access_token_hash() {
            Some(expected_access_token_hash) => {
                let actual_access_token_hash = AccessTokenHash::from_token(
                    token_response.access_token(),
                    id_token
                        .signing_alg()
                        .map_err(|_| OIDCError::TokenSigningError)?,
                    id_token
                        .signing_key(&id_token_verifier)
                        .map_err(|_| OIDCError::TokenSigningError)?,
                )
                .map_err(|_| OIDCError::TokenSigningError)?;

                if actual_access_token_hash != *expected_access_token_hash {
                    Err(OIDCError::VerifyTokenError)
                } else {
                    Ok(())
                }
            }
            None => Err(OIDCError::VerifyTokenError),
        }?;

        println!(
            "User {} with e-mail address {} has authenticated successfully",
            claims.subject().as_str(),
            claims
                .email()
                .map(|email| email.as_str())
                .unwrap_or("<not provided>"),
        );
        Ok(HttpResponse::Ok().body("Success"))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[derive(Debug, Display)]
enum OIDCError {
    #[display("Failed to exchange access token with authorization code")]
    ExchangeAccessTokenError,
    #[display("Failed to extract id token from token response")]
    ExtractIDTokenError,
    #[display("Failed to get id token claim")]
    GetIDTokenClaimError,
    #[display("Failed to verify token signing")]
    TokenSigningError,
    #[display("Failed to verify token")]
    VerifyTokenError,
}

impl actix_web::error::ResponseError for OIDCError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
