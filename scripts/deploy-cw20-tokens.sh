#!/bin/bash

# Deploy CW20-Style Token Contracts
# This script deploys the carbon credit, oil reserve, and bond token contracts

set -e

# Configuration
CHAIN_ID="517"
NODE_HOME="/root/david/2.cosmos/1.NNB/0.nnb-chain"
KEYRING_BACKEND="test"
FROM="validator"
GAS="auto"
GAS_ADJUSTMENT="1.3"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Deploying CW20-Style Token Contracts${NC}"
echo "================================================"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}📋 Checking prerequisites...${NC}"

if ! command_exists nnbd; then
    echo -e "${RED}❌ nnbd command not found. Please build the binary first.${NC}"
    exit 1
fi

if ! command_exists cargo; then
    echo -e "${RED}❌ cargo command not found. Please install Rust toolchain.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites check passed${NC}"

# Build contracts
echo -e "${YELLOW}🔨 Building contracts...${NC}"

CONTRACTS=("carbon-credit-token" "oil-reserve-token" "bond-token")
WASM_FILES=()

for contract in "${CONTRACTS[@]}"; do
    echo -e "${BLUE}Building $contract...${NC}"
    cd "contracts/$contract"
    
    # Build the contract
    cargo wasm
    
    # Copy the wasm file to a known location
    cp target/wasm32-unknown-unknown/release/${contract}.wasm ../../build/
    WASM_FILES+=("build/${contract}.wasm")
    
    cd ../..
    echo -e "${GREEN}✅ $contract built successfully${NC}"
done

# Deploy contracts
echo -e "${YELLOW}🚀 Deploying contracts...${NC}"

CODE_IDS=()

for i in "${!CONTRACTS[@]}"; do
    contract="${CONTRACTS[$i]}"
    wasm_file="${WASM_FILES[$i]}"
    
    echo -e "${BLUE}Deploying $contract...${NC}"
    
    # Store the contract
    STORE_RESULT=$(nnbd tx wasm store "$wasm_file" \
        --from "$FROM" \
        --keyring-backend "$KEYRING_BACKEND" \
        --home "$NODE_HOME" \
        --chain-id "$CHAIN_ID" \
        --gas "$GAS" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --output json \
        --yes)
    
    # Extract code ID
    CODE_ID=$(echo "$STORE_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ "$CODE_ID" = "null" ] || [ -z "$CODE_ID" ]; then
        echo -e "${RED}❌ Failed to get code ID for $contract${NC}"
        echo "Store result: $STORE_RESULT"
        exit 1
    fi
    
    CODE_IDS+=("$CODE_ID")
    echo -e "${GREEN}✅ $contract deployed with code ID: $CODE_ID${NC}"
done

# Create deployment summary
echo -e "${YELLOW}📊 Deployment Summary${NC}"
echo "========================"

for i in "${!CONTRACTS[@]}"; do
    contract="${CONTRACTS[$i]}"
    code_id="${CODE_IDS[$i]}"
    echo -e "${GREEN}$contract: Code ID $code_id${NC}"
done

# Save deployment info to file
DEPLOYMENT_INFO="deployment-info.json"
cat > "$DEPLOYMENT_INFO" << EOF
{
  "chain_id": "$CHAIN_ID",
  "deployment_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
EOF

for i in "${!CONTRACTS[@]}"; do
    contract="${CONTRACTS[$i]}"
    code_id="${CODE_IDS[$i]}"
    
    if [ $i -eq $((${#CONTRACTS[@]} - 1)) ]; then
        # Last contract, no comma
        cat >> "$DEPLOYMENT_INFO" << EOF
    "$contract": {
      "code_id": "$code_id",
      "wasm_file": "${WASM_FILES[$i]}"
    }
EOF
    else
        # Not last contract, add comma
        cat >> "$DEPLOYMENT_INFO" << EOF
    "$contract": {
      "code_id": "$code_id",
      "wasm_file": "${WASM_FILES[$i]}"
    },
EOF
    fi
done

cat >> "$DEPLOYMENT_INFO" << EOF
  }
}
EOF

echo -e "${GREEN}📄 Deployment info saved to $DEPLOYMENT_INFO${NC}"

# Next steps
echo -e "${YELLOW}🎯 Next Steps${NC}"
echo "============="
echo "1. Instantiate contracts with appropriate parameters"
echo "2. Set up verification bodies, auditors, and other authorized entities"
echo "3. Configure initial parameters for each token type"
echo "4. Test the contracts with sample data"

echo -e "${BLUE}📖 For detailed usage instructions, see contracts/README.md${NC}"

# Example instantiation commands
echo -e "${YELLOW}💡 Example Instantiation Commands${NC}"
echo "=================================="

for i in "${!CONTRACTS[@]}"; do
    contract="${CONTRACTS[$i]}"
    code_id="${CODE_IDS[$i]}"
    
    echo -e "${BLUE}$contract (Code ID: $code_id):${NC}"
    
    case $contract in
        "carbon-credit-token")
            echo "nnbd tx wasm instantiate $code_id '{\"cw20_base\":{\"name\":\"Carbon Credit Token\",\"symbol\":\"CCT\",\"decimals\":6,\"initial_balances\":[],\"mint\":{\"minter\":\"'\"\$ADMIN_ADDRESS\"'\"}},\"carbon_credit_info\":{\"project_id\":\"CC-001\",\"project_name\":\"Sample Project\",\"project_type\":\"renewable_energy\",\"verification_standard\":\"VCS\",\"vintage_year\":2024,\"country\":\"USA\",\"total_credits_issued\":\"0\",\"credits_retired\":\"0\",\"credits_available\":\"0\",\"co2_equivalent_per_credit\":\"1.0\",\"verification_body\":\"'\"\$VERIFICATION_BODY\"'\"}},\"project_developer\":\"'\"\$DEVELOPER\"'\"}}' --from validator --label \"Carbon Credit Token\" --admin \$ADMIN_ADDRESS"
            ;;
        "oil-reserve-token")
            echo "nnbd tx wasm instantiate $code_id '{\"cw20_base\":{\"name\":\"Oil Reserve Token\",\"symbol\":\"ORT\",\"decimals\":6,\"initial_balances\":[],\"mint\":{\"minter\":\"'\"\$ADMIN_ADDRESS\"'\"}},\"oil_reserve_info\":{\"reserve_id\":\"OR-001\",\"reserve_name\":\"Sample Reserve\",\"location\":\"USA\",\"field_name\":\"Sample Field\",\"oil_type\":\"LightSweet\",\"api_gravity\":\"35.0\",\"sulfur_content\":\"0.5\",\"total_reserves_barrels\":\"1000000\",\"extracted_barrels\":\"0\",\"available_barrels\":\"1000000\",\"barrels_per_token\":\"1.0\",\"extraction_company\":\"'\"\$EXTRACTION_COMPANY\"'\"}},\"reserve_auditor\":\"'\"\$AUDITOR\"'\"}},\"government_authority\":\"'\"\$GOVERNMENT\"'\"}},\"extraction_start_date\":\"1640995200\",\"estimated_extraction_end_date\":\"1672531200\"}}' --from validator --label \"Oil Reserve Token\" --admin \$ADMIN_ADDRESS"
            ;;
        "bond-token")
            echo "nnbd tx wasm instantiate $code_id '{\"cw20_base\":{\"name\":\"Bond Token\",\"symbol\":\"BT\",\"decimals\":6,\"initial_balances\":[],\"mint\":{\"minter\":\"'\"\$ADMIN_ADDRESS\"'\"}},\"bond_info\":{\"bond_id\":\"B-001\",\"bond_name\":\"Sample Bond\",\"issuer\":\"'\"\$ISSUER\"'\"}},\"bond_type\":\"Corporate\",\"face_value\":\"1000.0\",\"total_issue_amount\":\"1000000\",\"coupon_rate\":\"0.05\",\"coupon_frequency\":\"SemiAnnually\",\"maturity_date\":\"1672531200\",\"issue_date\":\"1640995200\",\"currency\":\"USD\",\"bond_rating\":\"A\",\"collateral_type\":\"RealEstate\",\"collateral_value\":\"1000000.0\",\"trustee\":\"'\"\$TRUSTEE\"'\"}},\"paying_agent\":\"'\"\$PAYING_AGENT\"'\"}},\"total_coupons_paid\":\"0\",\"total_principal_repaid\":\"0\",\"outstanding_principal\":\"1000000.0\",\"next_coupon_date\":\"1648771200\",\"accrued_interest\":\"0\"}}' --from validator --label \"Bond Token\" --admin \$ADMIN_ADDRESS"
            ;;
    esac
    echo ""
done

echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
