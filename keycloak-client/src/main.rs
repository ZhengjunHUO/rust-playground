use keycloak::{
    types::*,
    {KeycloakAdmin, KeycloakAdminToken},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:8080".into());
    let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
    let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "password".into());
    let realm = std::env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "default".into());

    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;
    eprintln!("Token: [{:?}]", admin_token);

    let admin = KeycloakAdmin::new(&url, admin_token, client);
    admin
        .post(RealmRepresentation {
            realm: Some(realm.clone()),
            ..Default::default()
        })
        .await?;

    let users = admin
        .realm_users_get(
            &realm, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None,
        )
        .await?;

    eprintln!("In realm [{}], found users {:?}", realm, users);

    Ok(())
}
