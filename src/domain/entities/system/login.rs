use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(FromForm, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn validate(&self) -> HashMap<&str, &str> {
        let mut errors = HashMap::new();

        if self.username.trim().is_empty() {
            errors.insert("username", "Username is required");
        }

        if self.password.trim().is_empty() {
            errors.insert("password", "Password is required");
        }

        errors
    }
}