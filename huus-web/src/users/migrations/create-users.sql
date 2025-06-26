CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL,
    CONSTRAINT username_unique UNIQUE (username)
);

CREATE INDEX idx_users_username ON users(username);

-- Create the Sessions table
CREATE TABLE user_sessions (
    token TEXT PRIMARY KEY,
    timeout BIGINT NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_on TIMESTAMP NOT NULL
);

-- Indexes for better performance
CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
