#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

/// Marketplace IDs
#[allow(unused)]
pub mod marketplace_ids {
    // North America
    pub const CA: &str = "A2EUQ1WTGCTBG2";
    pub const US: &str = "ATVPDKIKX0DER";
    pub const MX: &str = "A1AM78C64UM0Y8";
    pub const BR: &str = "A2Q3Y263D00KWC";
    // Europe
    pub const IE: &str = "A28R8C7NBKEWEA";
    pub const ES: &str = "A1RKKUPIHCS9HS";
    pub const UK: &str = "A1F83G8C2ARO7P";
    pub const FR: &str = "A13V1IB3VIYZZH";
    pub const BE: &str = "AMEN7PMS3EDWL";
    pub const NL: &str = "A1805IZSGTT6HS";
    pub const DE: &str = "A1PA6795UKMFR9";
    pub const IT: &str = "APJ6JRA9NG5V4";
    pub const SE: &str = "A2NODRKZP88ZB9";
    pub const ZA: &str = "AE08WJ6YKNBMC";
    pub const PL: &str = "A1C3SOZRARQ6R3";
    pub const EG: &str = "ARBP9OOSHTCHU";
    pub const TR: &str = "A33AVAJ2PDY3EV";
    pub const SA: &str = "A17E79C6D8DWNP";
    pub const AE: &str = "A2VIGQ35RCS4UG";
    pub const IN: &str = "A21TJRUUN4KGV";
    // Far East
    pub const SG: &str = "A19VAU5U5O7RUS";
    pub const AU: &str = "A39IBJ37TRP1C6";
    pub const JP: &str = "A1VC38T7YXB528";
}

/// Locale codes
#[allow(unused)]
pub mod locale {
    pub const EN_US: &str = "en_US";
    pub const EN_CA: &str = "en_CA";
    pub const ES_MX: &str = "es_MX";
    pub const PT_BR: &str = "pt_BR";
    pub const EN_GB: &str = "en_GB";
    pub const FR_FR: &str = "fr_FR";
    pub const NL_NL: &str = "nl_NL";
    pub const DE_DE: &str = "de_DE";
    pub const IT_IT: &str = "it_IT";
    pub const SV_SE: &str = "sv_SE";
    pub const PL_PL: &str = "pl_PL";
    pub const AR_EG: &str = "ar_EG";
    pub const TR_TR: &str = "tr_TR";
    pub const AR_SA: &str = "ar_SA";
    pub const EN_IN: &str = "en_IN";
    pub const EN_AU: &str = "en_AU";
    pub const JA_JP: &str = "ja_JP";
}

pub mod apis;
pub mod models;
pub mod client;
pub mod client_apis;
