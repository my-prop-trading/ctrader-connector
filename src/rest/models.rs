use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenRequest {
    pub login: i64,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenResponse {
    #[serde(rename = "webservToken")]
    pub token: String,
}

/// Note that there are two possible outputs depending on whether you specify a unique email
/// in the request body (an email that is not used by any of the users registered on your server).
/// If email is unique, the response will include all parameters from the below table.
/// If the specified email is already assigned to an existing user, the output will only include the userId parameter.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtidRequest {
    pub email: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: Option<String>,
}

/// Note that there are two possible outputs depending on whether you specify a unique email
/// in the request body (an email that is not used by any of the users registered on your server).
/// If email is unique, the response will include all parameters from the below table.
/// If the specified email is already assigned to an existing user, the output will only include the userId parameter.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtidResponse {
    /// The unique identifier of the user entity.
    #[serde(rename = "userId")]
    pub user_id: i64,
    /// The nickname of the user entity. By default, nickname=ctid{userId}.
    /// None when the specified email is already assigned to an existing user
    pub nickname: Option<String>,
    /// None when the specified email is already assigned to an existing user
    pub email: Option<String>,
    /// An Alpha-2 code denoting the preferred language of the user entity.
    /// None when the specified email is already assigned to an existing user
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: Option<String>,
    /// The epoch unix timestamp of the creation of the user entity.
    /// None when the specified email is already assigned to an existing user
    #[serde(rename = "utcCreateTimestamp")]
    pub timestamp: Option<u64>,
    /// None when the specified email is already assigned to an existing user
    pub status: Option<CtidStatus>,
}

/// The status of the new user entity. The following values are accepted.
/// "CTID_NEW". The default status for any new user.
/// "CTID_ACTIVE". The status denoting an existing active user who has confirmed their email address in the cTrader ecosystem. Note that only users with "CTID_ACTIVE" as their status receive trading notifications in their email inbox.
/// "CTID_DELETED". The status denoting a deleted user entity.
/// Note that receiving "CTID_ACTIVE" or "CTID_DELETED" in the response body would constitute unexpected behavior.
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
    /// The total amount of leverage available to the account; is specified in 10^2. E.g.,
    /// the 1:1 leverage is leverageInCents=100 while the 1:100 leverage is leverageInCents=10000.
    #[serde(rename = "leverageInCents")]
    pub leverage_in_cents: i64,
    /// The strategy via which the account margin is calculated. The following values are accepted.
    /// "MAX". The total margin requirements per symbol are equal to the maximum margin requirements for all positions opened for this symbol.
    /// "SUM". The total margin requirements per symbol are equal to the sum of all margin requirements of all positions opened for this symbol.
    /// "NET". The total margin requirements per symbol are equal to the difference between the margin requirements for all long positions and all short positions opened for this symbol.
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
    pub max_leverage: Option<i64>,
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
    pub country_id: Option<i64>,
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
pub struct LinkCtidRequest {
    #[serde(rename = "traderLogin")]
    pub trader_login: i64,
    #[serde(rename = "traderPasswordHash")]
    pub trader_password_hash: String,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "environmentName")]
    pub environment_name: String,
    /// A flag that denotes whether the ctidTraderAccountId key is returned in the response to this API call.
    /// Set it to true to ensure that the response to this call is not empty.
    #[serde(rename = "returnAccountDetails")]
    pub return_account_details: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkCtidResponse {
    #[serde(rename = "ctidTraderAccountId")]
    pub ctid_trader_account_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTraderRequest {
    #[serde(rename = "accessRights")]
    pub access_rights: Option<TraderAccessRights>,
    #[serde(rename = "accountType")]
    pub account_type: Option<TraderAccountType>,
    #[serde(rename = "brokerName")]
    pub broker_name: Option<String>,
    #[serde(rename = "depositCurrency")]
    pub deposit_currency: Option<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: Option<String>,
    /// The total amount of leverage available to the account; is specified in 10^2. E.g.,
    /// the 1:1 leverage is leverageInCents=100 while the 1:100 leverage is leverageInCents=10000.
    #[serde(rename = "leverageInCents")]
    pub leverage_in_cents: Option<i64>,
    /// The strategy via which the account margin is calculated. The following values are accepted.
    /// "MAX". The total margin requirements per symbol are equal to the maximum margin requirements for all positions opened for this symbol.
    /// "SUM". The total margin requirements per symbol are equal to the sum of all margin requirements of all positions opened for this symbol.
    /// "NET". The total margin requirements per symbol are equal to the difference between the margin requirements for all long positions and all short positions opened for this symbol.
    #[serde(rename = "totalMarginCalculationType")]
    pub total_margin_calculation_type: Option<TotalMarginCalculationType>,
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
    pub max_leverage: Option<i64>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTraderBalanceRequest {
    /// A short note that can be attached to any balance change. This note is not shown to retail clients.
    pub comment: Option<String>,
    /// A number that matches an external identifier of the broker’s choosing. For instance, the value of externalId could be equal to the number of the bank transfer order through which the user chose to make a deposit.
    #[serde(rename = "externalId")]
    pub external_id: Option<String>,
    /// A brief comment that can supplement a deposit or a withdrawal. In contrast to comment, this text is displayed to retail clients.
    #[serde(rename = "externalNote")]
    pub external_note: Option<String>,
    /// login	Yes	integer	The number of a specific trading account.
    pub login: i64,
    /// preciseAmount	Yes	double	The precise amount of withdrawn or deposited funds/credit. Specified as a decimal number. For BTC and similar assets, the value of preciseAmount can include as many as eight digits after the decimal point.
    #[serde(rename = "preciseAmount")]
    pub precise_amount: f64,
    /// source	No	string	The designated source of the deposit/withdrawal.
    pub source: Option<String>,
    /// The desired type of balance change. The following values are accepted.
    /// "DEPOSIT". Deposit funds to the trader.
    /// "WITHDRAW". Withdraw funds from the trader.
    /// "DEPOSIT_NONWITHDRAWABLE_BONUS". Deposit credit to the trader.
    /// "WITHDRAW_NONWITHDRAWABLE_BONUS". Withdraw credit from the trader.
    #[serde(rename = "type")]
    pub change_type: BalanceChangeType,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum BalanceChangeType {
    #[strum(to_string = "DEPOSIT")]
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[strum(to_string = "WITHDRAW")]
    #[serde(rename = "WITHDRAW")]
    Withdraw,
    #[strum(to_string = "DEPOSIT_NONWITHDRAWABLE_BONUS")]
    #[serde(rename = "DEPOSIT_NONWITHDRAWABLE_BONUS")]
    DepositNonwithdrawableBonus,
    #[strum(to_string = "WITHDRAW_NONWITHDRAWABLE_BONUS")]
    #[serde(rename = "WITHDRAW_NONWITHDRAWABLE_BONUS")]
    WithdrawNonwithdrawableBonus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTraderBalanceResponse {
    /// The identifier of a balance history entity containing all balance-related transactions for the specified trader.
    /// Note that bonus and credit are not included in balanceHistoryId.
    #[serde(rename = "balanceHistoryId")]
    pub balance_history_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTradersRequest {
    #[serde(with = "string_date_format")]
    pub from: DateTime<Utc>,
    #[serde(with = "string_date_format")]
    pub to: DateTime<Utc>,
    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTradersResponse {
    #[serde(rename = "trader")]
    pub items: Vec<TraderModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraderModel {
    pub bonus: i64,
    pub equity: i64,
    #[serde(rename = "freeMargin")]
    pub free_margin: i64,
    /// The current amount of funds that the account can withdraw.
    /// It is calculated via the following formula: cashEquity = balance + unrealized P&L - management fees,
    /// where management fees are all management fees charged by the providers of strategies that the
    /// account owner has invested in. Subject to moneyDigits.
    #[serde(rename = "cashEquity")]
    pub cash_equity: i64,
    #[serde(rename = "lastUpdateTimestamp")]
    pub last_update_timestamp: i64,
    pub login: i64,
    /// The number that determines how finance-related values are defined for the account. E.g.,
    /// if moneyDigits=2 and balance=1234512, the account balance is 12345.12 in the account deposit currency.
    /// Additional details are given in Section 3.
    #[serde(rename = "moneyDigits")]
    pub money_digits: u32,
    #[serde(rename = "registrationTimestamp")]
    pub registration_timestamp: i64,
    /// If this parameter equals true, rollover commissions are applied to the account instead of swaps.
    /// The reverse applies if the parameter is false. This field is useful for ensuring compliance with Sharia law.
    #[serde(rename = "swapFree")]
    pub swap_free: bool,
    #[serde(rename = "usedMargin")]
    pub used_margin: i64,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "depositCurrency")]
    pub deposit_currency: String,
    #[serde(rename = "accessRights")]
    pub access_rights: TraderAccessRights,
    #[serde(rename = "balance")]
    pub balance: i64,
    #[serde(rename = "nonWithdrawableBonus")]
    pub non_withdrawable_bonus: i64,
    #[serde(rename = "leverageInCents")]
    pub leverage_in_cents: u32,
    #[serde(rename = "contactDetails")]
    pub contact_details: TraderContactDetails,
    #[serde(rename = "lastConnectionTimestamp")]
    pub last_connection_timestamp: Option<i64>,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "introducingBroker")]
    pub introducing_broker: bool,
    #[serde(rename = "introducingBrokerCommissionRate")]
    pub introducing_broker_commission_rate: f64,
    #[serde(rename = "pocketCommissionRate")]
    pub pocket_commission_rate: f64,
    #[serde(rename = "pocketMarkupRate")]
    pub pocket_markup_rate: f64,
    #[serde(rename = "rebateRate")]
    pub rebate_rate: f64,
    #[serde(rename = "defaultIntroducingBrokerCommissionRate")]
    pub default_introducing_broker_commission_rate: f64,
    #[serde(rename = "defaultPocketCommissionRate")]
    pub default_pocket_commission_rate: f64,
    #[serde(rename = "defaultPocketMarkupRate")]
    pub default_pocket_markup_rate: f64,
    #[serde(rename = "defaultRebateRate")]
    pub default_rebate_rate: f64,
    #[serde(rename = "defaultSplitRevenue")]
    pub default_split_revenue: bool,
    #[serde(rename = "limitedRisk")]
    pub limited_risk: bool,
    #[serde(rename = "sendOwnStatement")]
    pub send_own_statement: bool,
    #[serde(rename = "splitRevenue")]
    pub split_revenue: bool,
    //#[serde(rename = "ranks")]
    //pub ranks: Ranks,
    #[serde(rename = "totalMarginCalculationType")]
    pub total_margin_calculation_type: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "frenchRisk")]
    pub french_risk: bool,
    #[serde(rename = "isLimitedRisk")]
    pub is_limited_risk: bool,
    #[serde(rename = "defaultIbCommissionsType")]
    pub default_ib_commissions_type: String,
    #[serde(rename = "ibCommissionsType")]
    pub ib_commissions_type: String,
}

mod string_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f"; // yyyy-MM-ddTHH:mm:ss.SSS e.g., 2018-01-01T12:12:12.000

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

/// The difference between the timestamps specified in the from and to parameters must be equal to two days or less.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetClosedPositionsRequest {
    #[serde(with = "string_date_format")]
    pub from: DateTime<Utc>,
    #[serde(with = "string_date_format")]
    pub to: DateTime<Utc>,
    pub login: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedPositionModel {
    #[serde(rename = "login")]
    pub login: i64,
    #[serde(rename = "positionId")]
    pub position_id: i64,
    #[serde(rename = "dealId")]
    pub deal_id: i64,
    #[serde(with = "string_date_format")]
    #[serde(rename = "openTimestamp")]
    pub open_timestamp: DateTime<Utc>,
    #[serde(with = "string_date_format")]
    #[serde(rename = "closeTimestamp")]
    pub close_timestamp: DateTime<Utc>,
    #[serde(rename = "entryPrice")]
    pub entry_price: f64,
    #[serde(rename = "closePrice")]
    pub close_price: f64,
    /// The position direction. The following values are permitted.
    /// "BUY". Denotes a long position.
    /// "SELL". Denotes a short position.
    #[serde(rename = "direction")]
    pub direction: PositionDirection,
    #[serde(rename = "volume")]
    pub volume: f64,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "commission")]
    pub commission: f64,
    #[serde(rename = "swap")]
    pub swap: f64,
    #[serde(rename = "pnl")]
    pub pnl: f64,
    #[serde(rename = "depositConversionRate")]
    pub deposit_conversion_rate: f64,
    #[serde(rename = "usdConversionRate")]
    pub usd_conversion_rate: f64,
    #[serde(rename = "bookType")]
    pub book_type: BookType,
    #[serde(rename = "stake")]
    pub stake: f64,
    #[serde(rename = "spreadBetting")]
    pub spread_betting: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTraderGroupsResponse {
    #[serde(rename = "traderGroup")]
    pub items: Vec<TraderGroupModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraderGroupModel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetSymbolsResponse {
    #[serde(rename = "symbol")]
    pub items: Vec<SymbolModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolModel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    pub category: String,
    /// The flag that determines whether a symbol is currently receiving quotes.
    #[serde(rename = "quotesEnabled")]
    pub quotes_enabled: bool,
    /// The flag that determines whether a symbol is currently shown in cTrader applications.
    #[serde(rename = "showInCtrader")]
    pub show_in_ctrader: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOpenedPositionsRequest {
    pub login: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenedPositionModel {
    #[serde(rename = "login")]
    pub login: i64,
    #[serde(rename = "positionId")]
    pub position_id: i64,
    #[serde(rename = "openTimestamp", with = "string_date_format")]
    pub open_timestamp: DateTime<Utc>,
    #[serde(rename = "entryPrice")]
    pub entry_price: f64,
    #[serde(rename = "direction")]
    pub direction: PositionDirection,
    #[serde(rename = "volume")]
    pub volume: f64,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "commission")]
    pub commission: f64,
    #[serde(rename = "swap")]
    pub swap: f64,
    #[serde(rename = "bookType")]
    pub book_type: BookType,
    #[serde(rename = "stake")]
    pub stake: f64,
    #[serde(rename = "spreadBetting")]
    pub spread_betting: bool,
    #[serde(rename = "usedMargin")]
    pub used_margin: f64,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum BookType {
    #[strum(to_string = "BOOK_A")]
    #[serde(rename = "BOOK_A")]
    BookA,
    #[strum(to_string = "BOOK_B")]
    #[serde(rename = "BOOK_B")]
    BookB,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum PositionDirection {
    #[strum(to_string = "BUY")]
    #[serde(rename = "BUY")]
    Buy,
    #[strum(to_string = "SELL")]
    #[serde(rename = "SELL")]
    Sell,
}
