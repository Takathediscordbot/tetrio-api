use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
}
