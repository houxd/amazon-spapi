#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#![allow(unused)]
pub mod region {
    pub static NA: &str = "na";
    pub static EU: &str = "eu";
    pub static FE: &str = "fe";
}

#[allow(unused)]
pub mod marketplace_ids {
    // North America
    pub static CA: &str = "A2EUQ1WTGCTBG2";
    pub static US: &str = "ATVPDKIKX0DER";
    pub static MX: &str = "A1AM78C64UM0Y8";
    pub static BR: &str = "A2Q3Y263D00KWC";
    // Europe
    pub static IE: &str = "A28R8C7NBKEWEA";
    pub static ES: &str = "A1RKKUPIHCS9HS";
    pub static UK: &str = "A1F83G8C2ARO7P";
    pub static FR: &str = "A13V1IB3VIYZZH";
    pub static BE: &str = "AMEN7PMS3EDWL";
    pub static NL: &str = "A1805IZSGTT6HS";
    pub static DE: &str = "A1PA6795UKMFR9";
    pub static IT: &str = "APJ6JRA9NG5V4";
    pub static SE: &str = "A2NODRKZP88ZB9";
    pub static ZA: &str = "AE08WJ6YKNBMC";
    pub static PL: &str = "A1C3SOZRARQ6R3";
    pub static EG: &str = "ARBP9OOSHTCHU";
    pub static TR: &str = "A33AVAJ2PDY3EV";
    pub static SA: &str = "A17E79C6D8DWNP";
    pub static AE: &str = "A2VIGQ35RCS4UG";
    pub static IN: &str = "A21TJRUUN4KGV";
    // Far East
    pub static SG: &str = "A19VAU5U5O7RUS";
    pub static AU: &str = "A39IBJ37TRP1C6";
    pub static JP: &str = "A1VC38T7YXB528";
}

#[allow(unused)]
pub mod locale {
    pub static EN_US: &str = "en_US";
    pub static EN_CA: &str = "en_CA";
    pub static ES_MX: &str = "es_MX";
    pub static PT_BR: &str = "pt_BR";
    pub static EN_GB: &str = "en_GB";
    pub static FR_FR: &str = "fr_FR";
    pub static NL_NL: &str = "nl_NL";
    pub static DE_DE: &str = "de_DE";
    pub static IT_IT: &str = "it_IT";
    pub static SV_SE: &str = "sv_SE";
    pub static PL_PL: &str = "pl_PL";
    pub static AR_EG: &str = "ar_EG";
    pub static TR_TR: &str = "tr_TR";
    pub static AR_SA: &str = "ar_SA";
    pub static EN_IN: &str = "en_IN";
    pub static EN_AU: &str = "en_AU";
    pub static JA_JP: &str = "ja_JP";
}

pub mod apis;
pub mod models;
pub mod client;
pub mod client_apis;
