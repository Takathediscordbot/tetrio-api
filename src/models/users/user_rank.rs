use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
pub enum UserRank {
    /// TETR.IO X rank
    X,
    /// TETR.IO U rank
    U,
    /// TETR.IO SS rank
    SS,
    /// TETR.IO S+ rank
    #[serde(rename = "s+")]
    SPlus,
    /// TETR.IO S rank
    #[serde(rename = "s")]
    S,
    /// TETR.IO S- rank
    #[serde(rename = "s-")]
    SMinus,
    /// TETR.IO A+ rank
    #[serde(rename = "a+")]
    APlus,
    /// TETR.IO A rank
    #[serde(rename = "a")]
    A,
    /// TETR.IO A- rank
    #[serde(rename = "a-")]
    AMinus,
    /// TETR.IO B+ rank
    #[serde(rename = "b+")]
    BPlus,
    /// TETR.IO B rank
    #[serde(rename = "b")]
    B,
    /// TETR.IO B- rank
    #[serde(rename = "b-")]
    BMinus,
    /// TETR.IO C+ rank
    #[serde(rename = "c+")]
    CPlus,
    /// TETR.IO C rank
    #[serde(rename = "c")]
    C,
    /// TETR.IO C- rank
    #[serde(rename = "c-")]
    CMinus,
    /// TETR.IO D+ rank
    #[serde(rename = "d+")]
    DPlus,
    /// TETR.IO D rank
    #[serde(rename = "d")]
    D,
    /// TETR.IO ? rank
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
