use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Auth {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Connection {
    address: String,
    user: String,
    password: String,
    db: String,
}
