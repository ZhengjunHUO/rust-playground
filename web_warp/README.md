## Run the server
```sh
export AUTH_ENDPOINT="https://<keycloak_url>/auth/realms/<target_realm>/protocol/openid-connect/userinfo"
cargo run
```

## Send request example
```sh
curl -X GET -H "Authorization: Bearer <TOKEN>" 127.0.0.1:8000/show
```
