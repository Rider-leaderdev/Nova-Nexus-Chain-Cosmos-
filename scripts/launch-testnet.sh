#!/bin/bash

# Launch Mantrachain Testnet
# This script sets up and launches a testnet with multiple validators

set -e

# Configuration
CHAIN_ID="mantrachain-testnet-1"
NODE_HOME="/root/david/2.cosmos/1.NNB/mantrachain-testnet"
KEYRING_BACKEND="test"
NUM_VALIDATORS=3
VALIDATOR_STAKE="100000000uom"
GENESIS_SUPPLY="100000000000000uom"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Launching Mantrachain Testnet${NC}"
echo "=================================="
echo "Chain ID: $CHAIN_ID"
echo "Validators: $NUM_VALIDATORS"
echo "Stake per validator: $VALIDATOR_STAKE"
echo ""

# Clean up any existing setup
if [ -d "$NODE_HOME" ]; then
    echo -e "${YELLOW}🧹 Cleaning up existing setup...${NC}"
    rm -rf "$NODE_HOME"
fi

# Initialize the chain
echo -e "${YELLOW}📋 Initializing chain...${NC}"
./build/nnbd init mantrachain-testnet --chain-id "$CHAIN_ID" --home "$NODE_HOME" --default-denom uom

# Create validators and add to genesis
echo -e "${YELLOW}👥 Setting up validators...${NC}"

for i in $(seq 1 $NUM_VALIDATORS); do
    validator_name="validator$i"
    echo -e "${BLUE}Creating $validator_name...${NC}"
    
    # Create validator key
    ./build/nnbd keys add "$validator_name" --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME"
    
    # Get validator address
    validator_addr=$(./build/nnbd keys show "$validator_name" -a --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME")
    
    # Add genesis account
    ./build/nnbd genesis add-genesis-account "$validator_addr" "$GENESIS_SUPPLY" --home "$NODE_HOME"
    
    # Create genesis transaction
    ./build/nnbd genesis gentx "$validator_name" "$VALIDATOR_STAKE" --chain-id "$CHAIN_ID" --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME"
    
    echo -e "${GREEN}✅ $validator_name created${NC}"
done

# Collect genesis transactions
echo -e "${YELLOW}📦 Collecting genesis transactions...${NC}"
./build/nnbd genesis collect-gentxs --home "$NODE_HOME"

# Update genesis parameters
echo -e "${YELLOW}⚙️  Configuring genesis parameters...${NC}"

# Update genesis.json with better parameters for testnet
GENESIS_FILE="$NODE_HOME/config/genesis.json"

# Reduce block time for faster testing
jq '.consensus_params.block.time_iota_ms = "1000"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"

# Set minimum gas prices
jq '.app_state.crisis.constant_fee.denom = "uom"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"
jq '.app_state.crisis.constant_fee.amount = "1000"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"

# Configure governance parameters
jq '.app_state.gov.params.voting_period = "120s"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"
jq '.app_state.gov.params.deposit_period = "60s"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"
jq '.app_state.gov.params.min_deposit[0].denom = "uom"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"
jq '.app_state.gov.params.min_deposit[0].amount = "1000000"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"

# Configure staking parameters
jq '.app_state.staking.params.unbonding_time = "60s"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"
jq '.app_state.staking.params.max_validators = 100' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"

# Configure distribution parameters
jq '.app_state.distribution.params.community_tax = "0.000000000000000000"' "$GENESIS_FILE" > tmp_genesis.json && mv tmp_genesis.json "$GENESIS_FILE"

echo -e "${GREEN}✅ Genesis parameters configured${NC}"

# Configure node settings
echo -e "${YELLOW}🔧 Configuring node settings...${NC}"

CONFIG_FILE="$NODE_HOME/config/config.toml"

# Update config.toml
sed -i 's/timeout_commit = ".*"/timeout_commit = "1s"/' "$CONFIG_FILE"
sed -i 's/timeout_propose = ".*"/timeout_propose = "1s"/' "$CONFIG_FILE"
sed -i 's/timeout_prevote = ".*"/timeout_prevote = "100ms"/' "$CONFIG_FILE"
sed -i 's/timeout_precommit = ".*"/timeout_precommit = "100ms"/' "$CONFIG_FILE"

# Enable RPC
sed -i 's/laddr = ".*"/laddr = "tcp:\/\/0.0.0.0:26657"/' "$CONFIG_FILE"

# Enable API
API_CONFIG_FILE="$NODE_HOME/config/app.toml"
sed -i 's/enable = false/enable = true/' "$API_CONFIG_FILE"
sed -i 's/address = ".*"/address = "tcp:\/\/0.0.0.0:1317"/' "$API_CONFIG_FILE"

# Enable gRPC
sed -i 's/address = ".*"/address = "0.0.0.0:9090"/' "$API_CONFIG_FILE"

echo -e "${GREEN}✅ Node settings configured${NC}"

# Create launch script
echo -e "${YELLOW}📝 Creating launch script...${NC}"

cat > "$NODE_HOME/start-testnet.sh" << 'EOF'
#!/bin/bash

# Start Mantrachain Testnet
echo "🚀 Starting Mantrachain Testnet..."

# Set environment variables
export CHAIN_ID="mantrachain-testnet-1"
export NODE_HOME="/root/david/2.cosmos/1.NNB/mantrachain-testnet"
export MINIMUM_GAS_PRICES="0.001uom"

# Start the node
./build/nnbd start \
    --home "$NODE_HOME" \
    --minimum-gas-prices "$MINIMUM_GAS_PRICES" \
    --log_level info \
    --log_format json
EOF

chmod +x "$NODE_HOME/start-testnet.sh"

# Create validator info file
echo -e "${YELLOW}📊 Creating validator information...${NC}"

VALIDATOR_INFO_FILE="$NODE_HOME/validator-info.json"
cat > "$VALIDATOR_INFO_FILE" << EOF
{
  "chain_id": "$CHAIN_ID",
  "validators": [
EOF

for i in $(seq 1 $NUM_VALIDATORS); do
    validator_name="validator$i"
    validator_addr=$(./build/nnbd keys show "$validator_name" -a --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME")
    validator_pubkey=$(./build/nnbd keys show "$validator_name" -p --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME")
    
    if [ $i -eq $NUM_VALIDATORS ]; then
        # Last validator, no comma
        cat >> "$VALIDATOR_INFO_FILE" << EOF
    {
      "name": "$validator_name",
      "address": "$validator_addr",
      "pubkey": "$validator_pubkey",
      "stake": "$VALIDATOR_STAKE"
    }
EOF
    else
        # Not last validator, add comma
        cat >> "$VALIDATOR_INFO_FILE" << EOF
    {
      "name": "$validator_name",
      "address": "$validator_addr",
      "pubkey": "$validator_pubkey",
      "stake": "$VALIDATOR_STAKE"
    },
EOF
    fi
done

cat >> "$VALIDATOR_INFO_FILE" << EOF
  ],
  "genesis_supply": "$GENESIS_SUPPLY",
  "rpc_endpoint": "http://localhost:26657",
  "api_endpoint": "http://localhost:1317",
  "grpc_endpoint": "localhost:9090"
}
EOF

echo -e "${GREEN}✅ Validator information saved to $VALIDATOR_INFO_FILE${NC}"

# Display summary
echo -e "${YELLOW}📋 Testnet Setup Summary${NC}"
echo "=========================="
echo "Chain ID: $CHAIN_ID"
echo "Validators: $NUM_VALIDATORS"
echo "Stake per validator: $VALIDATOR_STAKE"
echo "Genesis supply: $GENESIS_SUPPLY"
echo "RPC endpoint: http://localhost:26657"
echo "API endpoint: http://localhost:1317"
echo "gRPC endpoint: localhost:9090"
echo ""

# Display validator addresses
echo -e "${YELLOW}👥 Validator Addresses${NC}"
echo "====================="
for i in $(seq 1 $NUM_VALIDATORS); do
    validator_name="validator$i"
    validator_addr=$(./build/nnbd keys show "$validator_name" -a --keyring-backend "$KEYRING_BACKEND" --home "$NODE_HOME")
    echo -e "${GREEN}$validator_name: $validator_addr${NC}"
done
echo ""

# Next steps
echo -e "${YELLOW}🎯 Next Steps${NC}"
echo "============="
echo "1. Start the testnet: $NODE_HOME/start-testnet.sh"
echo "2. Check status: ./build/nnbd status --node http://localhost:26657"
echo "3. Query validators: ./build/nnbd query staking validators --node http://localhost:26657"
echo "4. Deploy CW20 contracts: ./scripts/deploy-cw20-tokens.sh"
echo ""

echo -e "${GREEN}🎉 Testnet setup completed successfully!${NC}"
echo -e "${BLUE}📖 For more information, see PUBLISHING_GUIDE.md${NC}"

