use chrono::{NaiveDateTime, Utc};
use crate::utils;

#[derive(Debug, Identifiable, Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
#[changeset_options(treat_none_as_null="true")]
#[primary_key(uuid)]
pub struct User {
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub username: String,

    pub password_hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub password_hint: Option<String>,

    pub akey: String,
    pub private_key: String,
    pub public_key: String,

    pub security_stamp: String,
    pub totp_enable: bool,

    pub password_iterations: u32,
    pub kdf_type: u32,
    pub kdf_iterations: u32,
    pub scope: String,
}

/// Local methods
impl User {
    pub const CLIENT_KDF_TYPE_DEFAULT: u32 = 0;
    pub const CLIENT_KDF_ITER_DEFAULT: u32 = 100000;

    pub  fn new(name: String, password_iterations: u32) -> Self {
        let now =  Utc::now().naive_utc();
        let username = name.to_lowercase();
        Self {
            uuid: utils::create_uuid(),
            created_at: now,
            updated_at: now,
            username: username.clone(),
            password_hash: Vec::new(),
            salt: utils::crypto::get_random_64(),
            password_iterations: password_iterations,

            security_stamp: utils::create_uuid(),

            password_hint: None,
            private_key: String::new(),
            public_key: String::new(),
            akey: String::new(),

            totp_enable: false,

            kdf_type: Self::CLIENT_KDF_TYPE_DEFAULT,
            kdf_iterations: Self::CLIENT_KDF_ITER_DEFAULT,

            scope: String::from("[]"),
        }
    }

    // password refers to the hashed password gotten from the client
    pub fn validate_password(&self, password: &str) -> bool {
        utils::crypto::validate_password(
            password.as_bytes(), 
            &self.salt, 
            &self.password_hash, 
            self.password_iterations
        )
    }

    // password refers to the hashed password gotten from the client
    pub fn set_password(&mut self, password: &str) {
        self.password_hash = utils::crypto::hash_password(
            password.as_bytes(), 
            &self.salt, 
            self.password_iterations
        )
    }

    pub fn reset_security_stamp(&mut self) {
        self.security_stamp = utils::create_uuid();
    }
}

use crate::db::schema::users;
use crate::db::DbConn;
use diesel::prelude::*;

/// Database Methods

impl User {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "Id": self.uuid,
            "username": self.username,
            "MasterPasswordHint": self.password_hint,
            "TwoFactorEnabled": self.totp_enable,
            "Key": self.akey,
            "PrivateKey": self.private_key,
            "SecurityStamp": self.security_stamp,
            "Scope": self.scope
        })
    }

    pub fn save(&mut self, conn: &DbConn) {
        if self.username.trim().is_empty() {
            return
        }
        
        self.updated_at = Utc::now().naive_utc();
        diesel::insert_into(users::table)
            .values(&*self)
            .execute(&**conn);
    }

    

}