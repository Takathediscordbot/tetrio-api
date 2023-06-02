use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for UserRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRank::X => f.write_str("X"),
            UserRank::U => f.write_str("U"),
            UserRank::SS => f.write_str("SS"),
            UserRank::SPlus => f.write_str("S+"),
            UserRank::S => f.write_str("S"),
            UserRank::SMinus => f.write_str("S-"),
            UserRank::APlus => f.write_str("A+"),
            UserRank::A => f.write_str("A"),
            UserRank::AMinus => f.write_str("A-"),
            UserRank::BPlus => f.write_str("B+"),
            UserRank::B => f.write_str("B"),
            UserRank::BMinus => f.write_str("B-"),
            UserRank::CPlus => f.write_str("C+"),
            UserRank::C => f.write_str("C"),
            UserRank::CMinus => f.write_str("C-"),
            UserRank::DPlus => f.write_str("D+"),
            UserRank::D => f.write_str("D"),
            UserRank::Z => f.write_str("Z"),
        }
    }
}
