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
