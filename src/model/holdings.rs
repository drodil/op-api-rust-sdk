//! Models required for
//! [HoldingsV1](https://op-developer.fi/docs/api/3VZiIRoT2EowKC6yeA0gqQ/HoldingsI)
//! API

use serde::Deserialize;

/// Describes a HoldingsInformation in holdings response.
#[derive(Deserialize, Debug)]
pub struct HoldingsInformation {
    /// List of fund Holdings in holdings response.
    #[serde(rename = "fundHoldings")]
    pub fund_holdings: Vec<Holdings>,
    /// SumOfAllHoldings in holdings response.
    #[serde(rename = "sumOfAllHoldings")]
    pub sum_of_all_holdings: SumOfAllHoldings,
    /// List of instrument Holdings in holdings response.
    #[serde(rename = "instrumentHoldings")]
    pub instrument_holdings: Vec<Holdings>,
}

/// Describes a single Holdings in response.
#[derive(Deserialize, Debug)]
pub struct Holdings {
    /// Fund name.
    #[serde(rename = "fundName")]
    pub fund_name: Option<String>,
    /// Instrument name.
    #[serde(rename = "instrumentName")]
    pub instrument_name: Option<String>,
    /// International securities identification number code. It is a 12 digit code consisting of
    /// numbers and letters that distinctly identify securities.
    #[serde(rename = "isinCode")]
    pub isin_code: String,
    /// Market value of the holding
    #[serde(rename = "marketValue")]
    pub market_value: f64,
    /// List of HoldingsItems
    #[serde(rename = "holdingsItem")]
    pub holdings_item: Vec<HoldingsItem>,
    /// Change of value of the holding
    #[serde(rename = "changeOfValue")]
    pub change_of_value: f64,
    /// Subscription value of the holding
    #[serde(rename = "subscriptionValue")]
    pub subscription_value: f64,
    /// Change as percentage of the holding
    #[serde(rename = "changeAsPercentage")]
    pub change_as_percentage: f64,
}

/// Describes a single HoldingsItem in response.
#[derive(Deserialize, Debug)]
pub struct HoldingsItem {
    /// Date of the holdings item
    pub date: String,
    /// Market value of the holdings item
    #[serde(rename = "marketValue")]
    pub market_value: f64,
    /// Subscription value of the holdings item
    #[serde(rename = "subscriptionValue")]
    pub subscription_value: f64,
}

/// Describes a SumOfAllHoldings in holdings response.
#[derive(Deserialize, Debug)]
pub struct SumOfAllHoldings {
    /// Market value of all holdings
    #[serde(rename = "marketValue")]
    pub market_value: f64,
    /// Change of value of all holdings
    #[serde(rename = "changeOfValue")]
    pub change_of_value: f64,
    /// Subscription value of all holdings
    #[serde(rename = "subscriptionValue")]
    pub subscription_value: f64,
    /// Change as percentage of all holdings
    #[serde(rename = "changeAsPercentage")]
    pub change_as_percentage: f64,
}
