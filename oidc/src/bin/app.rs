use actix_web::web::Data;
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use derive_more::Display;
use openidconnect::core::{CoreProviderMetadata, CoreUserInfoClaims};
use openidconnect::{
    reqwest, AccessToken, AdditionalClaims, Client, ClientId, ClientSecret, IntrospectionUrl,
    IssuerUrl,
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
    openidconnect::EndpointSet,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(init_oidcclient().await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                oidc_client: client.clone(),
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
        IssuerUrl::new("https://dev.huo.ai:8443/realms/oidc".to_string())
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
    .set_introspection_url(
        IntrospectionUrl::new(
            "https://dev.huo.ai:8443/realms/oidc/protocol/openid-connect/token/introspect"
                .to_owned(),
        )
        .expect("Error setting introspection url"),
    )
}

#[get("/app")]
// $ curl -H "Authorization: Bearer xxx" -v http://127.0.0.1:8001/app
async fn app(req: HttpRequest, data: Data<AppState>) -> Result<HttpResponse, Error> {
    println!("Request: {req:?}");

    if let Some(cred_header) = req.headers().get("authorization") {
        if let Some(cred) = cred_header
            .to_str()
            .unwrap()
            .strip_prefix("Bearer ")
            .map(str::trim)
        {
            println!("Found access token: {cred:?}");
            let http_client = init_http_client().expect("Error initing http client");
            let token = AccessToken::new(cred.to_owned());
            // let resp = data.oidc_client.introspect(&token)
            //     .request_async(&http_client)
            //     .await
            //     .map_err(|_| OIDCError::VerifyToken)?;

            let info_req = data
                .oidc_client
                .user_info(token, None)
                .map_err(|_| OIDCError::VerifyToken)?;
            //println!("info_req");
            // let resp: CoreUserInfoClaims = info_req.request_async(&http_client).await.map_err(|_| OIDCError::VerifyToken)?;
            let response: Result<
                CoreUserInfoClaims,
                openidconnect::UserInfoError<openidconnect::HttpClientError<reqwest::Error>>,
            > = info_req.request_async(&http_client).await;
            match response {
                Ok(resp) => {
                    println!("Introspection result: {resp:?}");
                    return Ok(HttpResponse::Ok().body("Success"));
                }
                Err(err) => println!("Error: {err:?}"),
            }

            // if resp.active() {
            //     println!("Token is active");
            //     return Ok(HttpResponse::Ok().body("Success"));
            // }
        }
    }

    Ok(HttpResponse::Unauthorized().finish())
}

async fn catch_all_handler(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body(format!("[APP] No route found for: {}", req.path()))
}

fn init_http_client() -> reqwest::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

#[derive(Debug, Display)]
enum OIDCError {
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
