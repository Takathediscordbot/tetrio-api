/// [Activity](https://tetr.io/about/api/#generalactivity) model


use crate::models::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Activity {
    /// A graph of user activity over the last 2 days. A user is seen as active if they logged in or received XP within the last 30 minutes.
    /// An array of plot points, newest points first.
    pub activity: Vec<i64>,
}

/// The packet returned by the API when requesting the activity.
pub type ActivityPacket = Packet<Activity>;
