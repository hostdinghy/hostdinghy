CREATE TABLE apps (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    server_id TEXT NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    created_on TIMESTAMP NOT NULL
);

CREATE INDEX idx_apps_team_id ON apps(team_id);
CREATE INDEX idx_apps_server_id ON apps(server_id);
