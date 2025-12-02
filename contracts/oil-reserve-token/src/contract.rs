use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, Storage, CosmosMsg, BankMsg, Coin, Decimal, Timestamp,
};
use cw20_base::{
    contract::{execute as cw20_execute, instantiate as cw20_instantiate, query as cw20_query},
    msg::{ExecuteMsg as Cw20ExecuteMsg, InstantiateMsg as Cw20InstantiateMsg, QueryMsg as Cw20QueryMsg},
    state::{MinterData, TokenInfo},
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Custom state for oil reserve specific data
pub const OIL_RESERVE_INFO: Item<OilReserveInfo> = Item::new("oil_reserve_info");
pub const EXTRACTION_RECORDS: Map<String, ExtractionRecord> = Map::new("extraction_records");
pub const RESERVE_AUDITS: Map<String, ReserveAudit> = Map::new("reserve_audits");
pub const TRADING_RECORDS: Map<String, TradingRecord> = Map::new("trading_records");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OilReserveInfo {
    pub reserve_id: String,
    pub reserve_name: String,
    pub location: String, // Country/Region
    pub field_name: String,
    pub oil_type: OilType,
    pub api_gravity: Decimal, // API gravity of the oil
    pub sulfur_content: Decimal, // Sulfur content percentage
    pub total_reserves_barrels: Uint128,
    pub extracted_barrels: Uint128,
    pub available_barrels: Uint128,
    pub barrels_per_token: Decimal, // How many barrels each token represents
    pub extraction_company: Addr,
    pub reserve_auditor: Addr,
    pub government_authority: Addr,
    pub extraction_start_date: Timestamp,
    pub estimated_extraction_end_date: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OilType {
    LightSweet,
    LightSour,
    HeavySweet,
    HeavySour,
    ExtraHeavy,
    Condensate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExtractionRecord {
    pub extraction_id: String,
    pub extraction_date: Timestamp,
    pub barrels_extracted: Uint128,
    pub extraction_method: ExtractionMethod,
    pub extraction_company: Addr,
    pub environmental_impact_score: Decimal, // 0-100 scale
    pub carbon_footprint_per_barrel: Decimal, // CO2 emissions per barrel
    pub extraction_cost_per_barrel: Decimal, // Cost in USD per barrel
    pub quality_certificate_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExtractionMethod {
    ConventionalDrilling,
    HydraulicFracturing,
    SteamInjection,
    HorizontalDrilling,
    OffshoreDrilling,
    EnhancedOilRecovery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReserveAudit {
    pub audit_id: String,
    pub audit_date: Timestamp,
    pub auditor: Addr,
    pub audited_reserves: Uint128,
    pub audit_report_url: String,
    pub audit_status: AuditStatus,
    pub reserve_quality_grade: String, // A, B, C grade
    pub extraction_feasibility_score: Decimal, // 0-100 scale
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AuditStatus {
    Pending,
    Approved,
    Rejected,
    RequiresReview,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradingRecord {
    pub trade_id: String,
    pub trade_date: Timestamp,
    pub seller: Addr,
    pub buyer: Addr,
    pub tokens_traded: Uint128,
    pub price_per_token: Decimal, // Price in USD
    pub total_value: Decimal,
    pub trade_type: TradeType,
    pub settlement_date: Timestamp,
    pub trade_status: TradeStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TradeType {
    Spot,
    Forward,
    Futures,
    Swap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TradeStatus {
    Pending,
    Executed,
    Settled,
    Cancelled,
}

// Extended instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_base: Cw20InstantiateMsg,
    pub oil_reserve_info: OilReserveInfo,
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
    
    // Oil reserve specific messages
    RecordExtraction {
        extraction_id: String,
        barrels_extracted: Uint128,
        extraction_method: ExtractionMethod,
        environmental_impact_score: Decimal,
        carbon_footprint_per_barrel: Decimal,
        extraction_cost_per_barrel: Decimal,
        quality_certificate_url: String,
    },
    ConductReserveAudit {
        audit_id: String,
        audited_reserves: Uint128,
        audit_report_url: String,
        reserve_quality_grade: String,
        extraction_feasibility_score: Decimal,
    },
    UpdateAuditStatus {
        audit_id: String,
        status: AuditStatus,
    },
    RecordTrade {
        trade_id: String,
        seller: String,
        buyer: String,
        tokens_traded: Uint128,
        price_per_token: Decimal,
        trade_type: TradeType,
        settlement_date: Timestamp,
    },
    UpdateTradeStatus {
        trade_id: String,
        status: TradeStatus,
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
    
    // Oil reserve specific queries
    OilReserveInfo {},
    ExtractionRecord { extraction_id: String },
    AllExtractionRecords { start_after: Option<String>, limit: Option<u32> },
    ReserveAudit { audit_id: String },
    AllReserveAudits { start_after: Option<String>, limit: Option<u32> },
    TradingRecord { trade_id: String },
    AllTradingRecords { start_after: Option<String>, limit: Option<u32> },
    AvailableBarrels {},
    ExtractedBarrels {},
    ReserveQualityMetrics {},
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Store oil reserve specific information
    OIL_RESERVE_INFO.save(deps.storage, &msg.oil_reserve_info)?;
    
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
        
        // Handle oil reserve specific messages
        ExecuteMsg::RecordExtraction { extraction_id, barrels_extracted, extraction_method, environmental_impact_score, carbon_footprint_per_barrel, extraction_cost_per_barrel, quality_certificate_url } => {
            record_extraction(deps, env, info, extraction_id, barrels_extracted, extraction_method, environmental_impact_score, carbon_footprint_per_barrel, extraction_cost_per_barrel, quality_certificate_url)
        }
        ExecuteMsg::ConductReserveAudit { audit_id, audited_reserves, audit_report_url, reserve_quality_grade, extraction_feasibility_score } => {
            conduct_reserve_audit(deps, env, info, audit_id, audited_reserves, audit_report_url, reserve_quality_grade, extraction_feasibility_score)
        }
        ExecuteMsg::UpdateAuditStatus { audit_id, status } => {
            update_audit_status(deps, env, info, audit_id, status)
        }
        ExecuteMsg::RecordTrade { trade_id, seller, buyer, tokens_traded, price_per_token, trade_type, settlement_date } => {
            record_trade(deps, env, info, trade_id, seller, buyer, tokens_traded, price_per_token, trade_type, settlement_date)
        }
        ExecuteMsg::UpdateTradeStatus { trade_id, status } => {
            update_trade_status(deps, env, info, trade_id, status)
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
        
        // Handle oil reserve specific queries
        QueryMsg::OilReserveInfo {} => {
            to_binary(&OIL_RESERVE_INFO.load(deps.storage)?)
        }
        QueryMsg::ExtractionRecord { extraction_id } => {
            to_binary(&EXTRACTION_RECORDS.load(deps.storage, extraction_id)?)
        }
        QueryMsg::AllExtractionRecords { start_after, limit } => {
            query_all_extraction_records(deps, start_after, limit)
        }
        QueryMsg::ReserveAudit { audit_id } => {
            to_binary(&RESERVE_AUDITS.load(deps.storage, audit_id)?)
        }
        QueryMsg::AllReserveAudits { start_after, limit } => {
            query_all_reserve_audits(deps, start_after, limit)
        }
        QueryMsg::TradingRecord { trade_id } => {
            to_binary(&TRADING_RECORDS.load(deps.storage, trade_id)?)
        }
        QueryMsg::AllTradingRecords { start_after, limit } => {
            query_all_trading_records(deps, start_after, limit)
        }
        QueryMsg::AvailableBarrels {} => {
            query_available_barrels(deps)
        }
        QueryMsg::ExtractedBarrels {} => {
            query_extracted_barrels(deps)
        }
        QueryMsg::ReserveQualityMetrics {} => {
            query_reserve_quality_metrics(deps)
        }
    }
}

// Oil reserve specific functions
fn record_extraction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    extraction_id: String,
    barrels_extracted: Uint128,
    extraction_method: ExtractionMethod,
    environmental_impact_score: Decimal,
    carbon_footprint_per_barrel: Decimal,
    extraction_cost_per_barrel: Decimal,
    quality_certificate_url: String,
) -> StdResult<Response> {
    // Only the extraction company can record extractions
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    if info.sender != oil_reserve_info.extraction_company {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only extraction company can record extractions".to_string() });
    }
    
    // Check if extraction exceeds available reserves
    if barrels_extracted > oil_reserve_info.available_barrels {
        return Err(cosmwasm_std::StdError::Overflow { source: cosmwasm_std::OverflowError::new(cosmwasm_std::OverflowOperation::Sub, oil_reserve_info.available_barrels, barrels_extracted) });
    }
    
    // Calculate tokens to mint based on barrels extracted
    let tokens_to_mint = barrels_extracted * oil_reserve_info.barrels_per_token;
    
    // Create extraction record
    let extraction_record = ExtractionRecord {
        extraction_id: extraction_id.clone(),
        extraction_date: env.block.time,
        barrels_extracted,
        extraction_method,
        extraction_company: info.sender.clone(),
        environmental_impact_score,
        carbon_footprint_per_barrel,
        extraction_cost_per_barrel,
        quality_certificate_url,
    };
    
    EXTRACTION_RECORDS.save(deps.storage, &extraction_id, &extraction_record)?;
    
    // Update oil reserve info
    let mut updated_info = oil_reserve_info;
    updated_info.extracted_barrels += barrels_extracted;
    updated_info.available_barrels = updated_info.available_barrels.checked_sub(barrels_extracted)?;
    OIL_RESERVE_INFO.save(deps.storage, &updated_info)?;
    
    // Mint tokens to the extraction company
    let mint_msg = Cw20ExecuteMsg::Mint { 
        recipient: info.sender.to_string(), 
        amount: tokens_to_mint 
    };
    cw20_execute(deps, env, info, mint_msg)?;
    
    Ok(Response::new()
        .add_attribute("action", "record_extraction")
        .add_attribute("extraction_id", extraction_id)
        .add_attribute("barrels_extracted", barrels_extracted)
        .add_attribute("tokens_minted", tokens_to_mint))
}

fn conduct_reserve_audit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    audit_id: String,
    audited_reserves: Uint128,
    audit_report_url: String,
    reserve_quality_grade: String,
    extraction_feasibility_score: Decimal,
) -> StdResult<Response> {
    // Only the reserve auditor can conduct audits
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    if info.sender != oil_reserve_info.reserve_auditor {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only reserve auditor can conduct audits".to_string() });
    }
    
    // Create audit record
    let audit_record = ReserveAudit {
        audit_id: audit_id.clone(),
        audit_date: env.block.time,
        auditor: info.sender.clone(),
        audited_reserves,
        audit_report_url,
        audit_status: AuditStatus::Pending,
        reserve_quality_grade,
        extraction_feasibility_score,
    };
    
    RESERVE_AUDITS.save(deps.storage, &audit_id, &audit_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "conduct_reserve_audit")
        .add_attribute("audit_id", audit_id)
        .add_attribute("audited_reserves", audited_reserves))
}

fn update_audit_status(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    audit_id: String,
    status: AuditStatus,
) -> StdResult<Response> {
    // Only the reserve auditor can update audit status
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    if info.sender != oil_reserve_info.reserve_auditor {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only reserve auditor can update audit status".to_string() });
    }
    
    let mut audit_record = RESERVE_AUDITS.load(deps.storage, &audit_id)?;
    audit_record.audit_status = status;
    RESERVE_AUDITS.save(deps.storage, &audit_id, &audit_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_audit_status")
        .add_attribute("audit_id", audit_id)
        .add_attribute("status", format!("{:?}", status)))
}

fn record_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    seller: String,
    buyer: String,
    tokens_traded: Uint128,
    price_per_token: Decimal,
    trade_type: TradeType,
    settlement_date: Timestamp,
) -> StdResult<Response> {
    let total_value = tokens_traded * price_per_token;
    
    // Create trading record
    let trading_record = TradingRecord {
        trade_id: trade_id.clone(),
        trade_date: env.block.time,
        seller: deps.api.addr_validate(&seller)?,
        buyer: deps.api.addr_validate(&buyer)?,
        tokens_traded,
        price_per_token,
        total_value,
        trade_type,
        settlement_date,
        trade_status: TradeStatus::Pending,
    };
    
    TRADING_RECORDS.save(deps.storage, &trade_id, &trading_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "record_trade")
        .add_attribute("trade_id", trade_id)
        .add_attribute("tokens_traded", tokens_traded)
        .add_attribute("total_value", total_value))
}

fn update_trade_status(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    status: TradeStatus,
) -> StdResult<Response> {
    let mut trading_record = TRADING_RECORDS.load(deps.storage, &trade_id)?;
    trading_record.trade_status = status;
    TRADING_RECORDS.save(deps.storage, &trade_id, &trading_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_trade_status")
        .add_attribute("trade_id", trade_id)
        .add_attribute("status", format!("{:?}", status)))
}

// Query functions
fn query_all_extraction_records(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = EXTRACTION_RECORDS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_reserve_audits(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = RESERVE_AUDITS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_trading_records(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = TRADING_RECORDS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_available_barrels(deps: Deps) -> StdResult<Binary> {
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    to_binary(&oil_reserve_info.available_barrels)
}

fn query_extracted_barrels(deps: Deps) -> StdResult<Binary> {
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    to_binary(&oil_reserve_info.extracted_barrels)
}

fn query_reserve_quality_metrics(deps: Deps) -> StdResult<Binary> {
    let oil_reserve_info = OIL_RESERVE_INFO.load(deps.storage)?;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    struct QualityMetrics {
        api_gravity: Decimal,
        sulfur_content: Decimal,
        oil_type: OilType,
        extraction_feasibility_score: Decimal,
    }
    
    let metrics = QualityMetrics {
        api_gravity: oil_reserve_info.api_gravity,
        sulfur_content: oil_reserve_info.sulfur_content,
        oil_type: oil_reserve_info.oil_type,
        extraction_feasibility_score: Decimal::zero(), // This would be calculated from recent audits
    };
    
    to_binary(&metrics)
}

use cosmwasm_std::Bound;
