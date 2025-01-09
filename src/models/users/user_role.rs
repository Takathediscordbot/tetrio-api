use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {

    #[serde(rename = "anon")]
    Anon,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "mod")]
    Mod,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "banned")]
    Banned,
    #[serde(rename = "halfmod")]
    HalfMod,
    #[serde(rename = "sysop")]
    SysOp,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(untagged)]
    Unknown(String)
}
