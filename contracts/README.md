# CW20-Style Token Contracts for Real-World Assets

This directory contains CosmWasm smart contracts implementing CW20-style tokens for real-world assets: Carbon Credits, Oil Reserves, and Bonds.

## Overview

These contracts extend the standard CW20 token functionality with domain-specific features for each asset type:

- **Carbon Credit Token**: Manages carbon credits with verification, retirement, and environmental impact tracking
- **Oil Reserve Token**: Tracks oil extraction, reserves, quality metrics, and trading
- **Bond Token**: Handles bond issuance, coupon payments, redemptions, and interest calculations

## Architecture

Each contract is built on top of the `cw20-base` contract, inheriting all standard CW20 functionality while adding specialized features:

```
CW20 Base Contract
├── Standard Token Functions (transfer, mint, burn, etc.)
├── Carbon Credit Extensions
│   ├── Verification System
│   ├── Retirement Tracking
│   └── Environmental Metrics
├── Oil Reserve Extensions
│   ├── Extraction Records
│   ├── Reserve Audits
│   └── Quality Metrics
└── Bond Extensions
    ├── Coupon Payments
    ├── Redemption Management
    └── Interest Calculations
```

## Contracts

### 1. Carbon Credit Token (`carbon-credit-token/`)

**Purpose**: Tokenize carbon credits with full lifecycle management

**Key Features**:
- Project verification and certification
- Credit retirement tracking
- Environmental impact metrics
- Compliance with carbon standards (VCS, Gold Standard, CAR)

**Key Messages**:
- `VerifyCredits`: Verify new carbon credits
- `RetireCredits`: Permanently retire credits (burn tokens)
- `UpdateVerificationStatus`: Update verification status

**Key Queries**:
- `CarbonCreditInfo`: Get project details
- `AvailableCredits`: Check available credits
- `RetiredCredits`: View retirement history

### 2. Oil Reserve Token (`oil-reserve-token/`)

**Purpose**: Tokenize oil reserves with extraction and quality tracking

**Key Features**:
- Oil extraction recording
- Reserve auditing and quality grading
- Trading record management
- Environmental impact tracking

**Key Messages**:
- `RecordExtraction`: Record oil extraction
- `ConductReserveAudit`: Audit reserve quality
- `RecordTrade`: Record trading transactions

**Key Queries**:
- `OilReserveInfo`: Get reserve details
- `AvailableBarrels`: Check available barrels
- `ReserveQualityMetrics`: Get quality data

### 3. Bond Token (`bond-token/`)

**Purpose**: Tokenize bonds with full financial lifecycle management

**Key Features**:
- Coupon payment management
- Bond redemption tracking
- Interest calculations
- Transfer and trading records

**Key Messages**:
- `PayCoupon`: Make coupon payments
- `RedeemBonds`: Redeem bonds at maturity
- `CalculateInterest`: Calculate accrued interest

**Key Queries**:
- `BondInfo`: Get bond details
- `OutstandingPrincipal`: Check outstanding amount
- `BondYield`: Get yield information

## Usage Examples

### Deploying a Carbon Credit Token

```rust
let instantiate_msg = InstantiateMsg {
    cw20_base: Cw20InstantiateMsg {
        name: "Carbon Credit Token".to_string(),
        symbol: "CCT".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: env.contract.address.to_string(),
            cap: None,
        }),
        marketing: None,
    },
    carbon_credit_info: CarbonCreditInfo {
        project_id: "CC-2024-001".to_string(),
        project_name: "Solar Farm Project".to_string(),
        project_type: "renewable_energy".to_string(),
        verification_standard: "VCS".to_string(),
        vintage_year: 2024,
        country: "USA".to_string(),
        total_credits_issued: Uint128::zero(),
        credits_retired: Uint128::zero(),
        credits_available: Uint128::zero(),
        co2_equivalent_per_credit: Decimal::from_str("1.0")?,
        verification_body: verification_body_addr,
        project_developer: developer_addr,
    },
};
```

### Verifying Carbon Credits

```rust
let verify_msg = ExecuteMsg::VerifyCredits {
    verification_id: "VER-001".to_string(),
    credits_to_verify: Uint128::from(1000u128),
    verification_report_url: "https://verification-reports.com/ver-001".to_string(),
};
```

### Recording Oil Extraction

```rust
let extraction_msg = ExecuteMsg::RecordExtraction {
    extraction_id: "EXT-001".to_string(),
    barrels_extracted: Uint128::from(10000u128),
    extraction_method: ExtractionMethod::ConventionalDrilling,
    environmental_impact_score: Decimal::from_str("75.5")?,
    carbon_footprint_per_barrel: Decimal::from_str("0.05")?,
    extraction_cost_per_barrel: Decimal::from_str("45.0")?,
    quality_certificate_url: "https://quality-certs.com/ext-001".to_string(),
};
```

### Making Bond Coupon Payments

```rust
let coupon_msg = ExecuteMsg::PayCoupon {
    payment_id: "CP-001".to_string(),
    coupon_period_start: Timestamp::from_seconds(1640995200), // Jan 1, 2022
    coupon_period_end: Timestamp::from_seconds(1648771200),   // Apr 1, 2022
    coupon_amount: Decimal::from_str("50000.0")?,
    principal_amount: Decimal::zero(),
    payment_method: PaymentMethod::BankTransfer,
};
```

## Building and Deploying

### Prerequisites

- Rust 1.70+
- CosmWasm toolchain
- Node running with CosmWasm support

### Build Contracts

```bash
# Build all contracts
cd contracts/carbon-credit-token
cargo wasm

cd ../oil-reserve-token
cargo wasm

cd ../bond-token
cargo wasm
```

### Deploy to Network

```bash
# Store contract code
nnbd tx wasm store carbon_credit_token.wasm --from validator --gas auto --gas-adjustment 1.3

# Instantiate contract
nnbd tx wasm instantiate <code_id> '{"cw20_base": {...}, "carbon_credit_info": {...}}' --from validator --label "Carbon Credit Token" --admin <admin_address>
```

## Integration with Existing Infrastructure

These contracts integrate seamlessly with the existing Mantrachain infrastructure:

1. **TokenFactory Module**: Can create wrapper tokens for these CW20 contracts
2. **CosmWasm Module**: Full support for contract execution and queries
3. **IBC Module**: Tokens can be transferred across chains
4. **Bank Module**: Standard token operations work with these contracts

## Security Considerations

- **Access Control**: Each contract implements role-based access control
- **Verification**: External verification bodies must be trusted entities
- **Audit Trails**: All operations are recorded with timestamps and signatures
- **Compliance**: Contracts support regulatory compliance requirements

## Future Enhancements

- **Cross-Chain Support**: IBC integration for multi-chain asset trading
- **Oracle Integration**: Real-time price feeds for assets
- **Governance**: DAO-style governance for asset management
- **Insurance**: Integration with insurance protocols
- **Analytics**: Advanced reporting and analytics features

## Testing

Each contract includes comprehensive test suites covering:

- Standard CW20 functionality
- Domain-specific features
- Edge cases and error conditions
- Integration scenarios

Run tests with:
```bash
cargo test
```

## Contributing

When contributing to these contracts:

1. Follow Rust best practices
2. Add comprehensive tests
3. Update documentation
4. Consider security implications
5. Maintain backward compatibility

## License

These contracts are licensed under the same terms as the Mantrachain project.
