# Launch dep services
```
docker run --name postgres-oidc -p 5432:5432 -e POSTGRES_PASSWORD=admin -d postgres
docker run --name keycloak-oidc -p 8080:8080 -e KC_BOOTSTRAP_ADMIN_USERNAME=admin -e KC_BOOTSTRAP_ADMIN_PASSWORD=admin quay.io/keycloak/keycloak:26.2.4 start-dev
```

# Prepare postgresql db
```
create database oidc;
\c oidc
CREATE TABLE sessions(
    key TEXT PRIMARY KEY NOT NULL,
    session_state JSONB,
    expires TIMESTAMP WITH TIME ZONE NOT NULL
);
```

# GraphQL introspection
```
cargo run --bin graphql
curl -X POST 127.0.0.1:8080/graphql -H "Content-Type: application/json" -d '{"query":"query IntrospectionQuery { __schema { types { name kind fields { name } } } }"}' | jq
```
