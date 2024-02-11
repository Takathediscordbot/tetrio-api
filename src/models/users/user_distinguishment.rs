use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
/// A user's distinguishment banner.
pub struct UserDistinguishment {
    /// The type of distinguishment banner.
    #[serde(rename = "type")]
    pub distinguishment_type: Arc<str>,
    /// Details shown on the banner
    pub detail: Option<Arc<str>>,
    /// The header of the banner
    pub header: Option<Arc<str>>,
    /// The footer of the banner
    pub footer: Option<Arc<str>>
}
