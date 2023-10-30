use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    id: Uuid,
    email: String,
    password: String,
    creation_date: DateTime<Utc>,
    verified: bool,
}
