use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SummaryType {
    #[serde(rename = "40l")]
    Sprint,
    Blitz,
    Zenith,
    ZenithEX,
    League,
    All,
    Zen
}

impl ToString for SummaryType {
    fn to_string(&self) -> String {
        match self {
            Self::Sprint => { return "40l".to_string(); },
            Self::Blitz => { return "blitz".to_string(); },
            Self::Zenith => { return "zenith".to_string(); },
            Self::ZenithEX => { return "zenithex".to_string(); },
            Self::League => { return "league ".to_string(); },
            Self::All => { return "all".to_string(); },
            Self::Zen => { return "zen ".to_string(); },
        };
    }
}
