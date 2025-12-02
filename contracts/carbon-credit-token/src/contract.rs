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

// Custom state for carbon credit specific data
pub const CARBON_CREDIT_INFO: Item<CarbonCreditInfo> = Item::new("carbon_credit_info");
pub const VERIFICATION_RECORDS: Map<String, VerificationRecord> = Map::new("verification_records");
pub const RETIREMENT_RECORDS: Map<String, RetirementRecord> = Map::new("retirement_records");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CarbonCreditInfo {
    pub project_id: String,
    pub project_name: String,
    pub project_type: String, // e.g., "renewable_energy", "forest_conservation", "carbon_capture"
    pub verification_standard: String, // e.g., "VCS", "Gold Standard", "CAR"
    pub vintage_year: u32,
    pub country: String,
    pub total_credits_issued: Uint128,
    pub credits_retired: Uint128,
    pub credits_available: Uint128,
    pub co2_equivalent_per_credit: Decimal, // tons of CO2 per credit
    pub verification_body: Addr,
    pub project_developer: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VerificationRecord {
    pub verification_id: String,
    pub verification_date: Timestamp,
    pub credits_verified: Uint128,
    pub verification_body: Addr,
    pub verification_report_url: String,
    pub status: VerificationStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RetirementRecord {
    pub retirement_id: String,
    pub retirement_date: Timestamp,
    pub credits_retired: Uint128,
    pub retirement_purpose: String,
    pub retirement_entity: Addr,
    pub retirement_certificate_url: String,
}

// Extended instantiate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_base: Cw20InstantiateMsg,
    pub carbon_credit_info: CarbonCreditInfo,
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
    
    // Carbon credit specific messages
    VerifyCredits {
        verification_id: String,
        credits_to_verify: Uint128,
        verification_report_url: String,
    },
    RetireCredits {
        retirement_id: String,
        credits_to_retire: Uint128,
        retirement_purpose: String,
        retirement_certificate_url: String,
    },
    UpdateVerificationStatus {
        verification_id: String,
        status: VerificationStatus,
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
    
    // Carbon credit specific queries
    CarbonCreditInfo {},
    VerificationRecord { verification_id: String },
    AllVerificationRecords { start_after: Option<String>, limit: Option<u32> },
    RetirementRecord { retirement_id: String },
    AllRetirementRecords { start_after: Option<String>, limit: Option<u32> },
    AvailableCredits {},
    RetiredCredits {},
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Store carbon credit specific information
    CARBON_CREDIT_INFO.save(deps.storage, &msg.carbon_credit_info)?;
    
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
        
        // Handle carbon credit specific messages
        ExecuteMsg::VerifyCredits { verification_id, credits_to_verify, verification_report_url } => {
            verify_credits(deps, env, info, verification_id, credits_to_verify, verification_report_url)
        }
        ExecuteMsg::RetireCredits { retirement_id, credits_to_retire, retirement_purpose, retirement_certificate_url } => {
            retire_credits(deps, env, info, retirement_id, credits_to_retire, retirement_purpose, retirement_certificate_url)
        }
        ExecuteMsg::UpdateVerificationStatus { verification_id, status } => {
            update_verification_status(deps, env, info, verification_id, status)
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
        
        // Handle carbon credit specific queries
        QueryMsg::CarbonCreditInfo {} => {
            to_binary(&CARBON_CREDIT_INFO.load(deps.storage)?)
        }
        QueryMsg::VerificationRecord { verification_id } => {
            to_binary(&VERIFICATION_RECORDS.load(deps.storage, verification_id)?)
        }
        QueryMsg::AllVerificationRecords { start_after, limit } => {
            query_all_verification_records(deps, start_after, limit)
        }
        QueryMsg::RetirementRecord { retirement_id } => {
            to_binary(&RETIREMENT_RECORDS.load(deps.storage, retirement_id)?)
        }
        QueryMsg::AllRetirementRecords { start_after, limit } => {
            query_all_retirement_records(deps, start_after, limit)
        }
        QueryMsg::AvailableCredits {} => {
            query_available_credits(deps)
        }
        QueryMsg::RetiredCredits {} => {
            query_retired_credits(deps)
        }
    }
}

// Carbon credit specific functions
fn verify_credits(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    verification_id: String,
    credits_to_verify: Uint128,
    verification_report_url: String,
) -> StdResult<Response> {
    // Only the verification body can verify credits
    let carbon_credit_info = CARBON_CREDIT_INFO.load(deps.storage)?;
    if info.sender != carbon_credit_info.verification_body {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only verification body can verify credits".to_string() });
    }
    
    // Create verification record
    let verification_record = VerificationRecord {
        verification_id: verification_id.clone(),
        verification_date: env.block.time,
        credits_verified: credits_to_verify,
        verification_body: info.sender.clone(),
        verification_report_url,
        status: VerificationStatus::Verified,
    };
    
    VERIFICATION_RECORDS.save(deps.storage, &verification_id, &verification_record)?;
    
    // Update carbon credit info
    let mut updated_info = carbon_credit_info;
    updated_info.total_credits_issued += credits_to_verify;
    updated_info.credits_available += credits_to_verify;
    CARBON_CREDIT_INFO.save(deps.storage, &updated_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "verify_credits")
        .add_attribute("verification_id", verification_id)
        .add_attribute("credits_verified", credits_to_verify))
}

fn retire_credits(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    retirement_id: String,
    credits_to_retire: Uint128,
    retirement_purpose: String,
    retirement_certificate_url: String,
) -> StdResult<Response> {
    // Check if user has enough credits
    let balance = cw20_base::state::BALANCES.load(deps.storage, &info.sender)?;
    if balance < credits_to_retire {
        return Err(cosmwasm_std::StdError::InsufficientFunds { 
            needed: credits_to_retire.into(), 
            available: balance.into() 
        });
    }
    
    // Burn the credits (retirement = permanent removal)
    cw20_base::state::BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(credits_to_retire)?)
    })?;
    
    // Update total supply
    cw20_base::state::TOKEN_INFO.update(deps.storage, |mut info| -> StdResult<_> {
        info.total_supply = info.total_supply.checked_sub(credits_to_retire)?;
        Ok(info)
    })?;
    
    // Create retirement record
    let retirement_record = RetirementRecord {
        retirement_id: retirement_id.clone(),
        retirement_date: env.block.time,
        credits_retired: credits_to_retire,
        retirement_purpose,
        retirement_entity: info.sender.clone(),
        retirement_certificate_url,
    };
    
    RETIREMENT_RECORDS.save(deps.storage, &retirement_id, &retirement_record)?;
    
    // Update carbon credit info
    let mut carbon_credit_info = CARBON_CREDIT_INFO.load(deps.storage)?;
    carbon_credit_info.credits_retired += credits_to_retire;
    carbon_credit_info.credits_available = carbon_credit_info.credits_available.checked_sub(credits_to_retire)?;
    CARBON_CREDIT_INFO.save(deps.storage, &carbon_credit_info)?;
    
    Ok(Response::new()
        .add_attribute("action", "retire_credits")
        .add_attribute("retirement_id", retirement_id)
        .add_attribute("credits_retired", credits_to_retire)
        .add_attribute("retirement_entity", info.sender))
}

fn update_verification_status(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    verification_id: String,
    status: VerificationStatus,
) -> StdResult<Response> {
    // Only the verification body can update status
    let carbon_credit_info = CARBON_CREDIT_INFO.load(deps.storage)?;
    if info.sender != carbon_credit_info.verification_body {
        return Err(cosmwasm_std::StdError::Unauthorized { msg: "Only verification body can update status".to_string() });
    }
    
    let mut verification_record = VERIFICATION_RECORDS.load(deps.storage, &verification_id)?;
    verification_record.status = status;
    VERIFICATION_RECORDS.save(deps.storage, &verification_id, &verification_record)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_verification_status")
        .add_attribute("verification_id", verification_id)
        .add_attribute("status", format!("{:?}", status)))
}

fn query_all_verification_records(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = VERIFICATION_RECORDS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_all_retirement_records(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.map(Bound::exclusive);
    
    let records: StdResult<Vec<_>> = RETIREMENT_RECORDS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .collect();
    
    to_binary(&records?)
}

fn query_available_credits(deps: Deps) -> StdResult<Binary> {
    let carbon_credit_info = CARBON_CREDIT_INFO.load(deps.storage)?;
    to_binary(&carbon_credit_info.credits_available)
}

fn query_retired_credits(deps: Deps) -> StdResult<Binary> {
    let carbon_credit_info = CARBON_CREDIT_INFO.load(deps.storage)?;
    to_binary(&carbon_credit_info.credits_retired)
}

use cosmwasm_std::Bound;
