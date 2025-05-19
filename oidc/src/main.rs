use actix_web::web::Data;
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use derive_more::Display;
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{reqwest, ClientId, ClientSecret, IssuerUrl, RedirectUrl};
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(init_oidcclient().await);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                oidc_client: client.clone(),
            }))
            .service(foo)
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
        IssuerUrl::new("http://localhost:8080/realms/foobar".to_string())
            .expect("Error setting issuer url"),
        &http_client,
    )
    .await
    .expect("Error grabbing provider metadata");

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("foo".to_owned()),
        Some(ClientSecret::new("bar".to_owned())),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8888/foo".to_owned())
            .expect("Error setting redirect url"),
    );

    return client;
}

#[get("/foo")]
async fn foo(data: Data<AppState>) -> Result<HttpResponse, Error> {
    println!("{:?}", data.oidc_client.token_uri());
    Ok(HttpResponse::Ok().body("Success"))
}

// #[derive(Debug, Display)]
// enum AuthError {
//     #[display("Failed to initilize an OIDC client")]
//     InitClientError,
// }

// impl actix_web::error::ResponseError for AuthError {
//     fn status_code(&self) -> actix_web::http::StatusCode {
//         actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
//     }

//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::InternalServerError().finish()
//     }
// }
