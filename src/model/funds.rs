//! Models required for
//! [Funds data V1](https://op-developer.fi/docs/api/5lWcjqy3JY2G2y4UGmS6Yw/Funds-data#operation/getFunds)
//! API
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Funds {
    pub document: Document,

    #[serde(rename = "fundName")]
    pub fund_name: String,

    #[serde(rename = "isinCode")]
    pub isin_code: String,

    #[serde(rename = "unitPrice")]
    pub unit_price: f32,
}

#[derive(Deserialize, Debug)]
pub struct Document {
    pub rules: String,

    pub brochure: String,

    #[serde(rename = "quartReport")]
    pub quart_report: String,
}
