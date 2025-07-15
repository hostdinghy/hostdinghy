CREATE TABLE servers (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    addr TEXT NOT NULL,
    name TEXT NOT NULL,
    api_token TEXT NOT NULL,
    tls_cert TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL,
    CONSTRAINT tls_cert_unique UNIQUE (tls_cert)
);

CREATE INDEX idx_servers_team_id ON servers(team_id);
