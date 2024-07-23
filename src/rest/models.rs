use serde::Serialize;
use serde_derive::Deserialize;

pub trait CtraderRequest: Serialize {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenRequest {
    pub login: i32,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenResponse {
    #[serde(rename = "webservToken")]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtidRequest {
    pub email: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtidResponse {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub nickname: String,
    pub email: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: String,
    #[serde(rename = "utcCreateTimestamp")]
    pub timestamp: u64,
    pub status: CtidStatus,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum CtidStatus {
    #[strum(to_string = "CTID_NEW")]
    #[serde(rename = "CTID_NEW")]
    CtidNew,
    #[strum(to_string = "CTID_ACTIVE")]
    #[serde(rename = "CTID_ACTIVE")]
    CtidActive,
    #[strum(to_string = "CTID_DELETED")]
    #[serde(rename = "CTID_DELETED")]
    CtidDeleted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTraderRequest {
    #[serde(rename = "accessRights")]
    pub access_rights: TraderAccessRights,
    #[serde(rename = "accountType")]
    pub account_type: TraderAccountType,
    pub balance: i64,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "depositCurrency")]
    pub deposit_currency: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
    #[serde(rename = "leverageInCents")]
    pub leverage_in_cents: i32,
    #[serde(rename = "totalMarginCalculationType")]
    pub total_margin_calculation_type: TotalMarginCalculationType,
    #[serde(rename = "contactDetails")]
    pub contact_details: Option<TraderContactDetails>,
    pub description: Option<String>,
    #[serde(rename = "isLimitedRisk")]
    pub is_limited_risk: Option<bool>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    /// The margin calculation strategy used for the limited risk account. The following values are accepted.
    #[serde(rename = "limitedRiskMarginCalculationStrategy")]
    pub limited_risk_margin_calculation_strategy: Option<LimitedRiskMarginCalculationStrategy>,
    /// The maximum amount of leverage (in the base currency units) available to the account. Specified in 10^2.
    #[serde(rename = "maxLeverage")]
    pub max_leverage: Option<i32>,
    /// The first name of the account holder.
    pub name: Option<String>,
    /// A flag determining whether a daily trading statement is sent to the trader.
    #[serde(rename = "sendOwnStatement")]
    pub send_own_statement: Option<bool>,
    /// A flag determining whether a daily account trading statement is sent to the broker under which the account is registered.
    #[serde(rename = "sendStatementToBroker")]
    pub send_statement_to_broker: Option<bool>,
    /// A flag determining whether the account is charged swaps (swapFree=true) or administrative fees (swapFree=false).
    #[serde(rename = "swapFree")]
    pub swap_free: Option<bool>,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum LimitedRiskMarginCalculationStrategy {
    /// Margin requirements have to be calculated based on the account leverage.
    #[strum(to_string = "ACCORDING_TO_LEVERAGE")]
    #[serde(rename = "ACCORDING_TO_LEVERAGE")]
    AccordingToLeverage,
    /// Margin requirements have to be calculated based on the distance between the position open price and the guaranteed stop loss.
    #[strum(to_string = "ACCORDING_TO_GSL")]
    #[serde(rename = "ACCORDING_TO_GSL")]
    AccordingToGsl,
    /// cServer calculates the leverage-based and GSL-based margin requirements, and chooses the larger of the two values.
    #[strum(to_string = "ACCORDING_TO_GSL_AND_LEVERAGE")]
    #[serde(rename = "ACCORDING_TO_GSL_AND_LEVERAGE")]
    AccordingToGslAndLeverage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraderContactDetails {
    pub address: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "countryId")]
    pub country_id: Option<i32>,
    #[serde(rename = "documentId")]
    pub document_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "zipCode")]
    pub zip_code: Option<String>,
    #[serde(rename = "introducingBroker1")]
    pub introducing_broker_1: Option<String>,
    #[serde(rename = "introducingBroker2")]
    pub introducing_broker_2: Option<String>,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum TotalMarginCalculationType {
    /// The total margin requirements per symbol are equal to the maximum margin requirements for all positions opened for this symbol.
    #[strum(to_string = "MAX")]
    #[serde(rename = "MAX")]
    Max,
    /// The total margin requirements per symbol are equal to the sum of all margin requirements of all positions opened for this symbol.
    #[strum(to_string = "SUM")]
    #[serde(rename = "SUM")]
    Sum,
    /// The total margin requirements per symbol are equal to the difference between the margin requirements for all long positions and all short positions opened for this symbol.
    #[strum(to_string = "NET")]
    #[serde(rename = "NET")]
    Net,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum TraderAccountType {
    /// The account can open positions in both directions for the same symbol simultaneously.
    #[strum(to_string = "HEDGED")]
    #[serde(rename = "HEDGED")]
    Hedged,
    /// The account can only positions in one directions for a given symbol.
    #[strum(to_string = "NETTED")]
    #[serde(rename = "NETTED")]
    Netted,
    /// The account can perform spread betting operations.
    #[strum(to_string = "SPREAD_BETTING")]
    #[serde(rename = "SPREAD_BETTING")]
    SpreadBetting,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum TraderAccessRights {
    #[strum(to_string = "FULL_ACCESS")]
    #[serde(rename = "FULL_ACCESS")]
    FullAccess,
    #[strum(to_string = "CLOSE_ONLY")]
    #[serde(rename = "CLOSE_ONLY")]
    CloseOnly,
    #[strum(to_string = "NO_TRADING")]
    #[serde(rename = "NO_TRADING")]
    NoTrading,
    #[strum(to_string = "NO_LOGIN")]
    #[serde(rename = "NO_LOGIN")]
    NoLogin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTraderResponse {
    pub bonus: i32,
    pub equity: i32,
    #[serde(rename = "freeMargin")]
    pub free_margin: i32,
    /// The current amount of funds that the account can withdraw.
    // It is calculated via the following formula: cashEquity = balance + unrealized P&L - management fees,
    // where management fees are all management fees charged by the providers of strategies that the
    // account owner has invested in. Subject to moneyDigits.
    #[serde(rename = "cashEquity")]
    pub cash_equity: i32,
    #[serde(rename = "lastUpdateTimestamp")]
    pub last_update_timestamp: i32,
    pub login: i32,
    /// The number that determines how finance-related values are defined for the account. E.g.,
    /// if moneyDigits=2 and balance=1234512, the account balance is 12345.12 in the account deposit currency.
    /// Additional details are given in Section 3.
    #[serde(rename = "moneyDigits")]
    pub money_digits: i32,
    #[serde(rename = "nonWithdrawableBonus")]
    pub non_withdrawal_bonus: i32,
    #[serde(rename = "registrationTimestamp")]
    pub registration_timestamp: i32,
    /// If this parameter equals true, rollover commissions are applied to the account instead of swaps.
    /// The reverse applies if the parameter is false. This field is useful for ensuring compliance with Sharia law.
    #[serde(rename = "swapFree")]
    pub swap_free: bool,
    #[serde(rename = "usedMargin")]
    pub used_margin: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkCtidRequest {
    #[serde(rename = "traderLogin")]
    pub trader_login: i32,
    #[serde(rename = "traderPasswordHash")]
    pub trader_password_hash: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "environmentName")]
    pub environment_name: String,
    /// A flag that denotes whether the ctidTraderAccountId key is returned in the response to this API call.
    /// Set it to true to ensure that the response to this call is not empty.
    #[serde(rename = "returnAccountDetails")]
    pub return_account_details: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkCtidResponse {
    #[serde(rename = "ctidTraderAccountId")]
    pub ctid_trader_account_id: Option<i32>,
}
