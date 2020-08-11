CREATE TABLE users ( 
    uuid                    CHAR(36)         NOT NULL PRIMARY KEY, 
    username                varchar(256)    NOT NULL UNIQUE,
    
    password_hint           TEXT,
    password_hash           BLOB            NOT NULL,
    salt                    BLOB            NOT NULL,
    
    password_iterations     INT UNSIGNED    NOT NULL,
    kdf_type                INT UNSIGNED    NOT NULL,
    kdf_iterations          INT UNSIGNED    NOT NULL,
    
    created_at              DATETIME        NOT NULL,
    updated_at              DATETIME        NOT NULL,
    totp_enable             BOOLEAN         NOT NULL DEFAULT FALSE,
    
    akey                    TEXT            NOT NULL,
    private_key             TEXT            NOT NULL,
    public_key              TEXT            NOT NULL,
    security_stamp          TEXT            NOT NULL,

    scope                   TEXT            NOT NULL
);

CREATE TABLE totp (
    uuid        CHAR(36)    NOT NULL PRIMARY KEY,
    user_uuid   CHAR(36)    NOT NULL REFERENCES users(uuid),
    type        TEXT        NOT NULL,
    secret      TEXT        NOT NULL,
    recover     TEXT        NOT NULL
);

CREATE TABLE files (
    uuid                CHAR(36)        NOT NULL PRIMARY KEY,
    user_uuid           CHAR(36)        NOT NULL REFERENCES users(uuid),
    stored_name         varchar(40)     NOT NULL,
    file_type           varchar(40)     NOT NULL,
    size                INT UNSIGNED    NOT NULL,
    file_hash           varchar(40)     NOT NULL UNIQUE,
    time_uploaded       varchar(40)     NOT NULL,
    protected           BOOLEAN         NOT NULL DEFAULT FALSE,
    listed              BOOLEAN         NOT NULL DEFAULT TRUE
);

CREATE TABLE grades (
    uuid            CHAR(36)        NOT NULL PRIMARY KEY,
    user_uuid       CHAR(36)        NOT NULL REFERENCES users(uuid),
    class_name      varchar(256)    NOT NULL UNIQUE,
    scale_type      BOOLEAN         NOT NULL DEFAULT FALSE,
    scale           JSON            NOT NULL,
    grades          JSON            NOT NULL
);

CREATE TABLE notes (
    uuid                CHAR(36)        NOT NULL PRIMARY KEY,
    user_uuid           CHAR(36)        NOT NULL REFERENCES users(uuid),
    notes               JSON            NOT NULL
);