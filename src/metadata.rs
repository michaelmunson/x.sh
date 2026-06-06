use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptMetadata {
    pub description: Option<String>,
    pub groups: Vec<String>,
    pub program: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub created: String,
    pub updated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_executed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMetadata {
    pub scripts: HashMap<String, Activity>,
}

