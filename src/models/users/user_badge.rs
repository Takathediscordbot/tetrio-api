use std::sync::Arc;

use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Debug, Clone)]
pub enum UserBadgeTs {
    Boolean(bool),
    String(Arc<str>)
}

struct UserBadgeVisitor;
impl<'d> Visitor<'d> for UserBadgeVisitor {
    type Value = UserBadgeTs;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected boolean or string value.")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(UserBadgeTs::Boolean(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(UserBadgeTs::String(v.into()))
    }
}

impl Serialize for UserBadgeTs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            UserBadgeTs::Boolean(b) => serializer.serialize_bool(*b),
            UserBadgeTs::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'d> Deserialize<'d> for UserBadgeTs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d> {
            deserializer.deserialize_any(UserBadgeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserBadge {
    pub id: Arc<str>, 
    pub label: Arc<str>,
    pub ts: Option<UserBadgeTs>,
    pub group: Option<Arc<str>>
}