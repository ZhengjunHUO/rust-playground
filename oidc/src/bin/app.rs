use actix_web::web::Data;
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use openidconnect::core::CoreProviderMetadata;
use openidconnect::{
    reqwest, AdditionalClaims, Client, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    PkceCodeVerifier, RedirectUrl,
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
            .default_service(actix_web::web::to(catch_all_handler))
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
async fn app(req: HttpRequest, _data: Data<AppState>) -> Result<HttpResponse, Error> {
    println!("Request: {req:?}");

    if let Some(cred) = req.headers().get("authorization") {
        println!("Found access token: {cred:?}");
    }

    // TODO: grab Access Token and check it against IdP

    Ok(HttpResponse::Ok().body("Success"))
}

async fn catch_all_handler(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body(format!("[APP] No route found for: {}", req.path()))
}

fn init_http_client() -> reqwest::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
}
