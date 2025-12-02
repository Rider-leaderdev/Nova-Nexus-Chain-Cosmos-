# Publishing Mantrachain Guide

## Overview
This guide covers publishing your Mantrachain blockchain to testnet or mainnet.

## Pre-Launch Checklist

### 1. **Security Audit**
- [ ] Code review completed
- [ ] Security audit by professional firm
- [ ] Penetration testing
- [ ] Smart contract audits (for CW20 tokens)

### 2. **Genesis Configuration**
- [ ] Finalize chain ID
- [ ] Set initial validator set
- [ ] Configure initial token supply
- [ ] Set governance parameters
- [ ] Configure module parameters

### 3. **Network Configuration**
- [ ] RPC endpoints configured
- [ ] API endpoints configured
- [ ] P2P seed nodes configured
- [ ] Persistent peers configured

## Launch Options

### Option 1: Testnet Launch (Recommended First)

#### Step 1: Prepare Genesis
```bash
# Create genesis with proper chain ID
./build/nnbd init mantrachain-testnet --chain-id mantrachain-testnet-1 --home ~/.mantrachain-testnet

# Add initial validators
./build/nnbd keys add validator1 --keyring-backend test --home ~/.mantrachain-testnet
./build/nnbd keys add validator2 --keyring-backend test --home ~/.mantrachain-testnet
./build/nnbd keys add validator3 --keyring-backend test --home ~/.mantrachain-testnet

# Add genesis accounts
./build/nnbd genesis add-genesis-account $(./build/nnbd keys show validator1 -a --keyring-backend test --home ~/.mantrachain-testnet) 100000000000000uom --home ~/.mantrachain-testnet
./build/nnbd genesis add-genesis-account $(./build/nnbd keys show validator2 -a --keyring-backend test --home ~/.mantrachain-testnet) 100000000000000uom --home ~/.mantrachain-testnet
./build/nnbd genesis add-genesis-account $(./build/nnbd keys show validator3 -a --keyring-backend test --home ~/.mantrachain-testnet) 100000000000000uom --home ~/.mantrachain-testnet

# Create genesis transactions
./build/nnbd genesis gentx validator1 100000000uom --chain-id mantrachain-testnet-1 --keyring-backend test --home ~/.mantrachain-testnet
./build/nnbd genesis gentx validator2 100000000uom --chain-id mantrachain-testnet-1 --keyring-backend test --home ~/.mantrachain-testnet
./build/nnbd genesis gentx validator3 100000000uom --chain-id mantrachain-testnet-1 --keyring-backend test --home ~/.mantrachain-testnet

# Collect genesis transactions
./build/nnbd genesis collect-gentxs --home ~/.mantrachain-testnet
```

#### Step 2: Configure Network
```bash
# Edit config.toml
nano ~/.mantrachain-testnet/config/config.toml

# Set seed nodes
seeds = "validator1@your-node-1:26656,validator2@your-node-2:26656,validator3@your-node-3:26656"

# Set persistent peers
persistent_peers = "validator1@your-node-1:26656,validator2@your-node-2:26656,validator3@your-node-3:26656"

# Configure RPC
laddr = "tcp://0.0.0.0:26657"
```

#### Step 3: Launch Testnet
```bash
# Start the network
./build/nnbd start --home ~/.mantrachain-testnet --minimum-gas-prices 0.001uom
```

### Option 2: Mainnet Launch

#### Step 1: Finalize Genesis
```bash
# Create mainnet genesis
./build/nnbd init mantrachain --chain-id mantrachain-1 --home ~/.mantrachain-mainnet

# Add production validators
# (Repeat for each validator)
./build/nnbd keys add validator-prod --keyring-backend file --home ~/.mantrachain-mainnet
./build/nnbd genesis add-genesis-account $(./build/nnbd keys show validator-prod -a --keyring-backend file --home ~/.mantrachain-mainnet) 100000000000000uom --home ~/.mantrachain-mainnet
./build/nnbd genesis gentx validator-prod 100000000uom --chain-id mantrachain-1 --keyring-backend file --home ~/.mantrachain-mainnet

# Collect all genesis transactions
./build/nnbd genesis collect-gentxs --home ~/.mantrachain-mainnet
```

#### Step 2: Deploy CW20 Contracts
```bash
# Deploy carbon credit token
./scripts/deploy-cw20-tokens.sh

# Instantiate contracts with production parameters
nnbd tx wasm instantiate <carbon_credit_code_id> '{"cw20_base":{...},"carbon_credit_info":{...}}' --from validator --label "Carbon Credit Token" --admin <admin_address>

# Deploy other tokens similarly
```

## Post-Launch Steps

### 1. **Monitor Network**
- Set up monitoring (Prometheus, Grafana)
- Monitor validator performance
- Track transaction metrics
- Monitor consensus health

### 2. **Community Setup**
- Create documentation website
- Set up block explorer
- Create faucet for testnet
- Set up governance forum

### 3. **Integration**
- List on Cosmos ecosystem directories
- Integrate with wallets (Keplr, Cosmostation)
- Set up IBC connections
- Deploy to exchanges

## Infrastructure Requirements

### Minimum Requirements
- **CPU**: 4+ cores
- **RAM**: 8GB+
- **Storage**: 100GB+ SSD
- **Network**: 100Mbps+ bandwidth

### Recommended for Production
- **CPU**: 8+ cores
- **RAM**: 32GB+
- **Storage**: 1TB+ NVMe SSD
- **Network**: 1Gbps+ bandwidth
- **Backup**: Automated backups
- **Monitoring**: Full observability stack

## Security Considerations

1. **Validator Security**
   - Use hardware security modules (HSMs)
   - Implement key rotation policies
   - Use multi-signature setups
   - Regular security audits

2. **Network Security**
   - DDoS protection
   - Firewall configuration
   - Regular security updates
   - Incident response plan

3. **Smart Contract Security**
   - Audit all CW20 contracts
   - Implement upgrade mechanisms
   - Monitor for vulnerabilities
   - Emergency response procedures

## Governance Setup

1. **Initial Governance Parameters**
   - Voting period: 14 days
   - Deposit period: 7 days
   - Minimum deposit: 1000uom
   - Quorum: 40%
   - Threshold: 50%

2. **Governance Proposals**
   - Parameter changes
   - Software upgrades
   - Community pool spending
   - Module additions/removals

## Monitoring and Maintenance

### Key Metrics to Monitor
- Block production rate
- Validator uptime
- Transaction throughput
- Network latency
- Disk usage
- Memory usage

### Maintenance Tasks
- Regular software updates
- Database maintenance
- Log rotation
- Backup verification
- Security patches

## Emergency Procedures

### Network Halt
1. Identify the cause
2. Coordinate with validators
3. Implement fix
4. Restart network
5. Communicate with community

### Security Incident
1. Assess the threat
2. Implement immediate mitigations
3. Notify validators
4. Coordinate response
5. Post-incident review

## Resources

- [Cosmos SDK Documentation](https://docs.cosmos.network/)
- [Tendermint Documentation](https://docs.tendermint.com/)
- [CosmWasm Documentation](https://docs.cosmwasm.com/)
- [IBC Documentation](https://ibc.cosmos.network/)

## Support

For technical support:
- GitHub Issues: [MANTRA-Chain/mantrachain](https://github.com/MANTRA-Chain/mantrachain)
- Discord: [Mantrachain Community](https://discord.gg/mantrachain)
- Documentation: [docs.mantrachain.io](https://docs.mantrachain.io)

