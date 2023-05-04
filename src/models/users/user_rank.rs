use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserRank {
    X,
    U,
    SS,
    #[serde(rename = "s+")]
    SPlus,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "s-")]
    SMinus,
    #[serde(rename = "a+")]
    APlus,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "a-")]
    AMinus,
    #[serde(rename = "b+")]
    BPlus,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "b-")]
    BMinus,
    #[serde(rename = "c+")]
    CPlus,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c-")]
    CMinus,
    #[serde(rename = "d+")]
    DPlus,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "z")]
    Z,
}
