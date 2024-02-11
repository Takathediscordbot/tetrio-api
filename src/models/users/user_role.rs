use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum UserRole {
    /// Anonymous user (user that doesn't own an account)
    #[serde(rename = "anon")]
    Anon,
    /// User that owns an account
    #[serde(rename = "user")]
    User,
    /// Bot account
    #[serde(rename = "bot")]
    Bot,
    /// TETR.IO Moderator
    #[serde(rename = "mod")]
    Mod,
    /// TETR.IO Admin
    #[serde(rename = "admin")]
    Admin,
    /// Banned user account
    #[serde(rename = "banned")]
    Banned,
    /// TETR.IO Half Moderator
    #[serde(rename = "halfmod")]
    HalfMod,
    /// TETR.IO Sysop
    #[serde(rename = "sysop")]
    SysOp,
}
