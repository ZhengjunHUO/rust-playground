use actix_web::web::Data;
use actix_web::{cookie, get, App, Error, HttpRequest, HttpResponse, HttpServer};
use openidconnect::core::{CoreAuthenticationFlow, CoreProviderMetadata};
use openidconnect::{
    reqwest, AccessToken, AccessTokenHash, AdditionalClaims, AuthorizationCode, Client, ClientId,
    ClientSecret, CsrfToken, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
}

#[derive(Serialize, Deserialize)]
struct Pkce {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
    nonce: Nonce,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(init_oidcclient().await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                oidc_client: client.clone(),
                //pkce: Mutex::new(None),
            }))
            .service(app)
    })
    .bind(("127.0.0.1", 8001))?
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

#[get("/app")]
async fn app(req: HttpRequest, data: Data<AppState>) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", req);
    // TODO: grab Access Token and check it against IdP

    return Ok(HttpResponse::Ok().body("Success"));
}

fn init_http_client() -> reqwest::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
}
