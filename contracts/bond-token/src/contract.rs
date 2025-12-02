use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, Storage, CosmosMsg, BankMsg, Coin, Decimal, Timestamp, WasmMsg,
};
use cw20_base::{
    contract::{execute as cw20_execute, instantiate as cw20_instantiate, query as cw20_query},
    msg::{ExecuteMsg as Cw20ExecuteMsg, InstantiateMsg as Cw20InstantiateMsg, QueryMsg as Cw20QueryMsg},
    state::{MinterData, TokenInfo},
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Custom state for bond specific data
pub const BOND_INFO: Item<BondInfo> = Item::new("bond_info");
pub const COUPON_PAYMENTS: Map<String, CouponPayment> = Map::new("coupon_payments");
pub const REDEMPTION_RECORDS: Map<String, RedemptionRecord> = Map::new("redemption_records");
pub const BOND_TRANSFERS: Map<String, BondTransfer> = Map::new("bond_transfers");
pub const INTEREST_CALCULATIONS: Map<String, InterestCalculation> = Map::new("interest_calculations");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BondInfo {
    pub bond_id: String,
    pub bond_name: String,
    pub issuer: Addr,
    pub bond_type: BondType,
    pub face_value: Decimal, // Face value per bond token
    pub total_issue_amount: Uint128, // Total number of bond tokens issued
    pub coupon_rate: Decimal, // Annual interest rate (e.g., 0.05 for 5%)
    pub coupon_frequency: CouponFrequency,
    pub maturity_date: Timestamp,
    pub issue_date: Timestamp,
    pub currency: String, // e.g., "USD", "EUR", "uom"
    pub bond_rating: BondRating,
    pub collateral_type: CollateralType,
    pub collateral_value: Decimal,
    pub trustee: Addr,
    pub paying_agent: Addr,
    pub total_coupons_paid: Decimal,
    pub total_principal_repaid: Decimal,
    pub outstanding_principal: Decimal,
    pub next_coupon_date: Timestamp,
    pub accrued_interest: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BondType {
    Corporate,
    Government,
    Municipal,
    AssetBacked,
    Convertible,
    ZeroCoupon,
    FloatingRate,
    Perpetual,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CouponFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    AtMaturity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BondRating {
    AAA,
    AA,
    A,
    BBB,
    BB,
    B,
    CCC,
    CC,
    C,
    D,
    Unrated,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CollateralType {
    RealEstate,
    Equipment,
    Inventory,
    AccountsReceivable,
    Cash,
    Securities,
    Commodities,
    IntellectualProperty,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CouponPayment {
    pub payment_id: String,
    pub payment_date: Timestamp,
    pub coupon_period_start: Timestamp,
    pub coupon_period_end: Timestamp,
    pub coupon_amount: Decimal,
    pub principal_amount: Decimal,
    pub total_payment: Decimal,
    pub payment_status: PaymentStatus,
    pub payment_method: PaymentMethod,
    pub transaction_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PaymentMethod {
    BankTransfer,
    CryptoTransfer,
    TokenTransfer,
    Escrow,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RedemptionRecord {
    pub redemption_id: String,
    pub redemption_date: Timestamp,
    pub bondholder: Addr,
    pub bonds_redeemed: Uint128,
    pub redemption_value: Decimal,
    pub redemption_type: RedemptionType,
    pub redemption_reason: String,
    pub transaction_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum RedemptionType {
    Maturity,
    EarlyRedemption,
    CallOption,
    PutOption,
    Default,
    Conversion,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BondTransfer {
    pub transfer_id: String,
    pub transfer_date: Timestamp,
    pub from: Addr,
    pub to: Addr,
    pub bonds_transferred: Uint128,
    pub transfer_price: Decimal,
    pub transfer_type: TransferType,
    pub transfer_reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TransferType {
    Sale,
    Gift,
    Inheritance,
    CollateralAssignment,
    Pledge,
    Repo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InterestCalculation {
    pub calculation_id: String,
    pub calculation_date: Timestamp,
    pub bondholder: Addr,
    pub bonds_held: Uint128,
    pub days_held: u32,
    pub accrued_interest: Decimal,
    pub coupon_rate: Decimal,
    pub calculation_method: CalculationMethod,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CalculationMethod {
    SimpleInterest,
    CompoundInterest,
    Actual365,
    Actual360,
    Thirty360,
}

// Extended instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_base: Cw20InstantiateMsg,
    pub bond_info: BondInfo,
}

// Extended execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Standard CW20 messages
    Transfer { recipient: String, amount: Uint128 },
    Burn { amount: Uint128 },
    Send { contract: String, amount: Uint128, msg: Binary },
    IncreaseAllowance { spender: String, amount: Uint128, expires: Option<cw20_base::msg::Expiration> },
    DecreaseAllowance { spender: String, amount: Uint128, expires: Option<cw20_base::msg::Expiration> },
    TransferFrom { owner: String, recipient: String, amount: Uint128 },
    SendFrom { owner: String, contract: String, amount: Uint128, msg: Binary },
    Mint { recipient: String, amount: Uint128 },
    UpdateMinter { new_minter: Option<String> },
    UpdateMarketing { project: Option<String>, description: Option<String>, marketing: Option<Addr> },
    UploadLogo(LogoInfo),
    
    // Bond specific messages
    PayCoupon {
        payment_id: String,
        coupon_period_start: Timestamp,
        coupon_period_end: Timestamp,
        coupon_amount: Decimal,
        principal_amount: Decimal,
        payment_method: PaymentMethod,
    },
    RedeemBonds {
        redemption_id: String,
        bondholder: String,
        bonds_to_redeem: Uint128,
        redemption_type: RedemptionType,
        redemption_reason: String,
    },
    RecordTransfer {
        transfer_id: String,
        from: String,
        to: String,
        bonds_transferred: Uint128,
        transfer_price: Decimal,
        transfer_type: TransferType,
        transfer_reason: String,
    },
    CalculateInterest {
        calculation_id: String,
        bondholder: String,
        bonds_held: Uint128,
        days_held: u32,
        calculation_method: CalculationMethod,
    },
    UpdatePaymentStatus {
        payment_id: String,
        status: PaymentStatus,
        transaction_hash: Option<String>,
    },
    UpdateBondRating {
        new_rating: BondRating,
    },
    UpdateCollateralValue {
        new_collateral_value: Decimal,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LogoInfo {
    pub url: Option<String>,
    pub upload: Option<String>,
}

// Extended query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Standard CW20 queries
    Balance { address: String },
    TokenInfo {},
    Minter {},
    Allowance { owner: String, spender: String },
    AllAllowances { owner: String, start_after: Option<String>, limit: Option<u32> },
    AllAccounts { start_after: Option<String>, limit: Option<u32> },
    MarketingInfo {},
    DownloadLogo {},
    
    // Bond specific queries
    BondInfo {},
    CouponPayment { payment_id: String },
    AllCouponPayments { start_after: Option<String>, limit: Option<u32> },
    RedemptionRecord { redemption_id: String },
    AllRedemptionRecords { start_after: Option<String>, limit: Option<u32> },
    BondTransfer { transfer_id: String },
    AllBondTransfers { start_after: Option<String>, limit: Option<u32> },
    InterestCalculation { calculation_id: String },
    AllInterestCalculations { start_after: Option<String>, limit: Option<u32> },
    BondholderInfo { bondholder: String },
    OutstandingPrincipal {},
    AccruedInterest { bondholder: String },
    NextCouponDate {},
    BondYield {},
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Store bond specific information
    BOND_INFO.save(deps.storage, &msg.bond_info)?;
    
    // Initialize the base CW20 contract
    cw20_instantiate(deps, env, info, msg.cw20_base)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        // Handle standard CW20 messages
        ExecuteMsg::Transfer { recipient, amount } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::Transfer { recipient, amount })
        }
        ExecuteMsg::Burn { amount } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::Burn { amount })
        }
        ExecuteMsg::Send { contract, amount, msg } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::Send { contract, amount, msg })
        }
        ExecuteMsg::IncreaseAllowance { spender, amount, expires } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::IncreaseAllowance { spender, amount, expires })
        }
        ExecuteMsg::DecreaseAllowance { spender, amount, expires } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::DecreaseAllowance { spender, amount, expires })
        }
        ExecuteMsg::TransferFrom { owner, recipient, amount } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::TransferFrom { owner, recipient, amount })
        }
        ExecuteMsg::SendFrom { owner, contract, amount, msg } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::SendFrom { owner, contract, amount, msg })
        }
        ExecuteMsg::Mint { recipient, amount } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::Mint { recipient, amount })
        }
        ExecuteMsg::UpdateMinter { new_minter } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::UpdateMinter { new_minter })
        }
        ExecuteMsg::UpdateMarketing { project, description, marketing } => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::UpdateMarketing { project, description, marketing })
        }
        ExecuteMsg::UploadLogo(logo_info) => {
            cw20_execute(deps, env, info, Cw20ExecuteMsg::UploadLogo(logo_info))
        }
        
        // Handle bond specific messages
        ExecuteMsg::PayCoupon { payment_id, coupon_period_start, coupon_period_end, coupon_amount, principal_amount, payment_method } => {
            pay_coupon(deps, env, info, payment_id, coupon_period_start, coupon_period_end, coupon_amount, principal_amount, payment_method)
        }
        ExecuteMsg::RedeemBonds { redemption_id, bondholder, bonds_to_redeem, redemption_type, redemption_reason } => {
            redeem_bonds(deps, env, info, redemption_id, bondholder, bonds_to_redeem, redemption_type, redemption_reason)
        }
        ExecuteMsg::RecordTransfer { transfer_id, from, to, bonds_transferred, transfer_price, transfer_type, transfer_reason } => {
            record_transfer(deps, env, info, transfer_id, from, to, bonds_transferred, transfer_price, transfer_type, transfer_reason)
        }
        ExecuteMsg::CalculateInterest { calculation_id, bondholder, bonds_held, days_held, calculation_method } => {
            calculate_interest(deps, env, info, calculation_id, bondholder, bonds_held, days_held, calculation_method)
        }
        ExecuteMsg::UpdatePaymentStatus { payment_id, status, transaction_hash } => {
            update_payment_status(deps, env, info, payment_id, status, transaction_hash)
        }
        ExecuteMsg::UpdateBondRating { new_rating } => {
            update_bond_rating(deps, env, info, new_rating)
        }
        ExecuteMsg::UpdateCollateralValue { new_collateral_value } => {
            update_collateral_value(deps, env, info, new_collateral_value)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Handle standard CW20 queries
        QueryMsg::Balance { address } => {
            cw20_query(deps, env, Cw20QueryMsg::Balance { address })
        }
        QueryMsg::TokenInfo {} => {
            cw20_query(deps, env, Cw20QueryMsg::TokenInfo {})
        }
        QueryMsg::Minter {} => {
            cw20_query(deps, env, Cw20QueryMsg::Minter {})
        }
        QueryMsg::Allowance { owner, spender } => {
            cw20_query(deps, env, Cw20QueryMsg::Allowance { owner, spender })
        }
        QueryMsg::AllAllowances { owner, start_after, limit } => {
            cw20_query(deps, env, Cw20QueryMsg::AllAllowances { owner, start_after, limit })
        }
        QueryMsg::AllAccounts { start_after, limit } => {
            cw20_query(deps, env, Cw20QueryMsg::AllAccounts { start_after, limit })
        }
        QueryMsg::MarketingInfo {} => {
            cw20_query(deps, env, Cw20QueryMsg::MarketingInfo {})
        }
        QueryMsg::DownloadLogo {} => {
            cw20_query(deps, env, Cw20QueryMsg::DownloadLogo {})
        }
        
        // Handle bond specific queries
        QueryMsg::BondInfo {} => {
            to_binary(&BOND_INFO.load(deps.storage)?)
        }
        QueryMsg::CouponPayment { payment_id } => {
            to_binary(&COUPON_PAYMENTS.load(deps.storage, payment_id)?)
        }
        QueryMsg::AllCouponPayments { start_after, limit } => {
            query_all_coupon_payments(deps, start_after, limit)
        }
        QueryMsg::RedemptionRecord { redemption_id } => {
            to_binary(&REDEMPTION_RECORDS.load(deps.storage, redemption_id)?)
        }
        QueryMsg::AllRedemptionRecords { start_after, limit } => {
            query_all_redemption_records(deps, start_after, limit)
        }
        QueryMsg::BondTransfer { transfer_id } => {
            to_binary(&BOND_TRANSFERS.load(deps.storage, transfer_id)?)
        }
        QueryMsg::AllBondTransfers { start_after, limit } => {
            query_all_bond_transfers(deps, start_after, limit)
        }
        QueryMsg::InterestCalculation { calculation_id } => {
            to_binary(&INTEREST_CALCULATIONS.load(deps.storage, calculation_id)?)
        }
        QueryMsg::AllInterestCalculations { start_after, limit } => {
            query_all_interest_calculations(deps, start_after, limit)
        }
        QueryMsg::BondholderInfo { bondholder } => {
            query_bondholder_info(deps, bondholder)
        }
        QueryMsg::OutstandingPrincipal {} => {
            query_outstanding_principal(deps)
        }
        QueryMsg::AccruedInterest { bondholder } => {
            query_accrued_interest(deps, bondholder)
        }
        QueryMsg::NextCouponDate {} => {
            query_next_coupon_date(deps)
        }
        QueryMsg::BondYield {} => {
            query_bond_yield(deps)
        }
    }
}

// Bond specific functions
fn pay_coupon(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    payment_id: String,
    coupon_period_start: Timestamp,
    coupon_period_end: Timestamp,
    coupon_amount: Decimal,
    principal_amount: Decimal,
    payment_method: PaymentMethod,
) -> StdResult<Response> {
    // Only the paying agent can make coupon payments
    let bond_info = BOND_INFO.load(deps.storage)?;
    if info.sender != bond_info.paying_agent {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only paying agent can make coupon payments".to_string() });
    }
    
    let total_payment = coupon_amount + principal_amount;
    
    // Create coupon payment record
    let coupon_payment = CouponPayment {
        payment_id: payment_id.clone(),
        payment_date: env.block.time,
        coupon_period_start,
        coupon_period_end,
        coupon_amount,
        principal_amount,
        total_payment,
        payment_status: PaymentStatus::Paid,
        payment_method,
        transaction_hash: None,
    };
    
    COUPON_PAYMENTS.save(deps.storage, &payment_id, &coupon_payment)?;
    
    // Update bond info
    let mut updated_info = bond_info;
    updated_info.total_coupons_paid += coupon_amount;
    updated_info.total_principal_repaid += principal_amount;
    updated_info.outstanding_principal = updated_info.outstanding_principal.checked_sub(principal_amount)?;
    
    // Calculate next coupon date based on frequency
    updated_info.next_coupon_date = calculate_next_coupon_date(updated_info.next_coupon_date, &updated_info.coupon_frequency);
    
    BOND_INFO.save(deps.storage, &updated_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "pay_coupon")
        .add_attribute("payment_id", payment_id)
        .add_attribute("total_payment", total_payment))
}

fn redeem_bonds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    redemption_id: String,
    bondholder: String,
    bonds_to_redeem: Uint128,
    redemption_type: RedemptionType,
    redemption_reason: String,
) -> StdResult<Response> {
    let bondholder_addr = deps.api.addr_validate(&bondholder)?;
    
    // Check if bondholder has enough bonds
    let balance = cw20_base::state::BALANCES.load(deps.storage, &bondholder_addr)?;
    if balance < bonds_to_redeem {
        return Err(cosmwasm_std::StdError::InsufficientFunds { 
            needed: bonds_to_redeem.into(), 
            available: balance.into() 
        });
    }
    
    // Calculate redemption value
    let bond_info = BOND_INFO.load(deps.storage)?;
    let redemption_value = bonds_to_redeem * bond_info.face_value;
    
    // Burn the bonds
    cw20_base::state::BALANCES.update(deps.storage, &bondholder_addr, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(bonds_to_redeem)?)
    })?;
    
    // Update total supply
    cw20_base::state::TOKEN_INFO.update(deps.storage, |mut info| -> StdResult<_> {
        info.total_supply = info.total_supply.checked_sub(bonds_to_redeem)?;
        Ok(info)
    })?;
    
    // Create redemption record
    let redemption_record = RedemptionRecord {
        redemption_id: redemption_id.clone(),
        redemption_date: env.block.time,
        bondholder: bondholder_addr.clone(),
        bonds_redeemed: bonds_to_redeem,
        redemption_value,
        redemption_type,
        redemption_reason,
        transaction_hash: None,
    };
    
    REDEMPTION_RECORDS.save(deps.storage, &redemption_id, &redemption_record)?;
    
    // Update bond info
    let mut updated_info = bond_info;
    updated_info.outstanding_principal = updated_info.outstanding_principal.checked_sub(redemption_value)?;
    BOND_INFO.save(deps.storage, &updated_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "redeem_bonds")
        .add_attribute("redemption_id", redemption_id)
        .add_attribute("bonds_redeemed", bonds_to_redeem)
        .add_attribute("redemption_value", redemption_value))
}

fn record_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    transfer_id: String,
    from: String,
    to: String,
    bonds_transferred: Uint128,
    transfer_price: Decimal,
    transfer_type: TransferType,
    transfer_reason: String,
) -> StdResult<Response> {
    // Create transfer record
    let transfer_record = BondTransfer {
        transfer_id: transfer_id.clone(),
        transfer_date: env.block.time,
        from: deps.api.addr_validate(&from)?,
        to: deps.api.addr_validate(&to)?,
        bonds_transferred,
        transfer_price,
        transfer_type,
        transfer_reason,
    };
    
    BOND_TRANSFERS.save(deps.storage, &transfer_id, &transfer_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "record_transfer")
        .add_attribute("transfer_id", transfer_id)
        .add_attribute("bonds_transferred", bonds_transferred))
}

fn calculate_interest(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    calculation_id: String,
    bondholder: String,
    bonds_held: Uint128,
    days_held: u32,
    calculation_method: CalculationMethod,
) -> StdResult<Response> {
    let bond_info = BOND_INFO.load(deps.storage)?;
    let bondholder_addr = deps.api.addr_validate(&bondholder)?;
    
    // Calculate accrued interest based on method
    let accrued_interest = match calculation_method {
        CalculationMethod::SimpleInterest => {
            bonds_held * bond_info.face_value * bond_info.coupon_rate * Decimal::from_ratio(days_held, 365)
        }
        CalculationMethod::Actual365 => {
            bonds_held * bond_info.face_value * bond_info.coupon_rate * Decimal::from_ratio(days_held, 365)
        }
        CalculationMethod::Actual360 => {
            bonds_held * bond_info.face_value * bond_info.coupon_rate * Decimal::from_ratio(days_held, 360)
        }
        CalculationMethod::Thirty360 => {
            bonds_held * bond_info.face_value * bond_info.coupon_rate * Decimal::from_ratio(days_held, 360)
        }
        _ => Decimal::zero(), // Compound interest would require more complex calculation
    };
    
    // Create interest calculation record
    let interest_calculation = InterestCalculation {
        calculation_id: calculation_id.clone(),
        calculation_date: env.block.time,
        bondholder: bondholder_addr,
        bonds_held,
        days_held,
        accrued_interest,
        coupon_rate: bond_info.coupon_rate,
        calculation_method,
    };
    
    INTEREST_CALCULATIONS.save(deps.storage, &calculation_id, &interest_calculation)?;
    
    Ok(Response::new()
        .add_attribute("action", "calculate_interest")
        .add_attribute("calculation_id", calculation_id)
        .add_attribute("accrued_interest", accrued_interest))
}

fn update_payment_status(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    payment_id: String,
    status: PaymentStatus,
    transaction_hash: Option<String>,
) -> StdResult<Response> {
    let mut payment_record = COUPON_PAYMENTS.load(deps.storage, &payment_id)?;
    payment_record.payment_status = status;
    payment_record.transaction_hash = transaction_hash;
    COUPON_PAYMENTS.save(deps.storage, &payment_id, &payment_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_payment_status")
        .add_attribute("payment_id", payment_id)
        .add_attribute("status", format!("{:?}", status)))
}

fn update_bond_rating(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_rating: BondRating,
) -> StdResult<Response> {
    // Only the issuer or trustee can update bond rating
    let bond_info = BOND_INFO.load(deps.storage)?;
    if info.sender != bond_info.issuer && info.sender != bond_info.trustee {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only issuer or trustee can update bond rating".to_string() });
    }
    
    let mut updated_info = bond_info;
    updated_info.bond_rating = new_rating;
    BOND_INFO.save(deps.storage, &updated_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_bond_rating")
        .add_attribute("new_rating", format!("{:?}", new_rating)))
}

fn update_collateral_value(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_collateral_value: Decimal,
) -> StdResult<Response> {
    // Only the trustee can update collateral value
    let bond_info = BOND_INFO.load(deps.storage)?;
    if info.sender != bond_info.trustee {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only trustee can update collateral value".to_string() });
    }
    
    let mut updated_info = bond_info;
    updated_info.collateral_value = new_collateral_value;
    BOND_INFO.save(deps.storage, &updated_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_collateral_value")
        .add_attribute("new_collateral_value", new_collateral_value))
}

// Helper function to calculate next coupon date
fn calculate_next_coupon_date(current_date: Timestamp, frequency: &CouponFrequency) -> Timestamp {
    match frequency {
        CouponFrequency::Monthly => Timestamp::from_seconds(current_date.seconds() + 30 * 24 * 60 * 60),
        CouponFrequency::Quarterly => Timestamp::from_seconds(current_date.seconds() + 90 * 24 * 60 * 60),
        CouponFrequency::SemiAnnually => Timestamp::from_seconds(current_date.seconds() + 180 * 24 * 60 * 60),
        CouponFrequency::Annually => Timestamp::from_seconds(current_date.seconds() + 365 * 24 * 60 * 60),
        CouponFrequency::AtMaturity => current_date, // No change for at-maturity bonds
    }
}

// Query functions
fn query_all_coupon_payments(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = COUPON_PAYMENTS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_redemption_records(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = REDEMPTION_RECORDS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_bond_transfers(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = BOND_TRANSFERS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_interest_calculations(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = INTEREST_CALCULATIONS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_bondholder_info(deps: Deps, bondholder: String) -> StdResult<Binary> {
    let bondholder_addr = deps.api.addr_validate(&bondholder)?;
    let balance = cw20_base::state::BALANCES.load(deps.storage, &bondholder_addr)?;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    struct BondholderInfo {
        address: String,
        bond_balance: Uint128,
        face_value_held: Decimal,
    }
    
    let bond_info = BOND_INFO.load(deps.storage)?;
    let face_value_held = balance * bond_info.face_value;
    
    let info = BondholderInfo {
        address: bondholder,
        bond_balance: balance,
        face_value_held,
    };
    
    to_binary(&info)
}

fn query_outstanding_principal(deps: Deps) -> StdResult<Binary> {
    let bond_info = BOND_INFO.load(deps.storage)?;
    to_binary(&bond_info.outstanding_principal)
}

fn query_accrued_interest(deps: Deps, bondholder: String) -> StdResult<Binary> {
    let bond_info = BOND_INFO.load(deps.storage)?;
    let bondholder_addr = deps.api.addr_validate(&bondholder)?;
    let balance = cw20_base::state::BALANCES.load(deps.storage, &bondholder_addr)?;
    
    // Simple calculation - in practice, this would be more sophisticated
    let accrued_interest = balance * bond_info.face_value * bond_info.coupon_rate;
    
    to_binary(&accrued_interest)
}

fn query_next_coupon_date(deps: Deps) -> StdResult<Binary> {
    let bond_info = BOND_INFO.load(deps.storage)?;
    to_binary(&bond_info.next_coupon_date)
}

fn query_bond_yield(deps: Deps) -> StdResult<Binary> {
    let bond_info = BOND_INFO.load(deps.storage)?;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    struct BondYield {
        coupon_rate: Decimal,
        current_yield: Decimal,
        yield_to_maturity: Decimal,
    }
    
    // Simplified yield calculation
    let yield_info = BondYield {
        coupon_rate: bond_info.coupon_rate,
        current_yield: bond_info.coupon_rate, // Simplified
        yield_to_maturity: bond_info.coupon_rate, // Simplified
    };
    
    to_binary(&yield_info)
}

use cosmwasm_std::Bound;
