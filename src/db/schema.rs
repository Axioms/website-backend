table! {
    files (uuid) {
        uuid -> Char,
        user_uuid -> Char,
        stored_name -> Varchar,
        file_type -> Varchar,
        size -> Unsigned<Integer>,
        file_hash -> Varchar,
        time_uploaded -> Varchar,
        protected -> Bool,
        listed -> Bool,
    }
}

table! {
    grades (uuid) {
        uuid -> Char,
        user_uuid -> Char,
        class_name -> Varchar,
        scale_type -> Bool,
        scale -> Longtext,
        #[sql_name = "grades"]
        scores -> Longtext,
    }
}

table! {
    notes (uuid) {
        uuid -> Char,
        user_uuid -> Char,
        #[sql_name = "notes"]
        texts -> Longtext,
    }
}

table! {
    totp (uuid) {
        uuid -> Char,
        user_uuid -> Char,
        #[sql_name = "type"]
        type_ -> Text,
        secret -> Text,
        recover -> Text,
    }
}

table! {
    users (uuid) {
        uuid -> Char,
        username -> Varchar,
        password_hint -> Nullable<Text>,
        password_hash -> Blob,
        salt -> Blob,
        password_iterations -> Unsigned<Integer>,
        kdf_type -> Unsigned<Integer>,
        kdf_iterations -> Unsigned<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        totp_enable -> Bool,
        akey -> Text,
        private_key -> Text,
        public_key -> Text,
        security_stamp -> Text,
        scope -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    grades,
    notes,
    totp,
    users,
);
