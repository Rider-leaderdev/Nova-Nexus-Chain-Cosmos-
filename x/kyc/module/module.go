package module

import (
	"encoding/json"
	"fmt"

	"github.com/SolanaPan/nnb/v1/x/kyc/keeper"
	"github.com/SolanaPan/nnb/v1/x/kyc/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	"github.com/spf13/cobra"
)

var (
	_ module.AppModuleBasic = (*AppModule)(nil)
)

// AppModuleBasic implements the AppModuleBasic interface that defines the
// independent methods a Cosmos SDK module needs to implement.
type AppModuleBasic struct {
	cdc codec.BinaryCodec
}

func NewAppModuleBasic(cdc codec.BinaryCodec) AppModuleBasic {
	return AppModuleBasic{cdc: cdc}
}

// Name returns the name of the module as a string.
func (AppModuleBasic) Name() string {
	return types.ModuleName
}

// RegisterLegacyAminoCodec registers the amino codec for the module.
func (AppModuleBasic) RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {}

// RegisterInterfaces registers a module's interface types and their concrete implementations.
func (a AppModuleBasic) RegisterInterfaces(reg cdctypes.InterfaceRegistry) {
	// Register interfaces here if needed
}

// DefaultGenesis returns a default GenesisState for the module.
func (AppModuleBasic) DefaultGenesis(cdc codec.JSONCodec) json.RawMessage {
	return cdc.MustMarshalJSON(types.DefaultGenesis())
}

// ValidateGenesis used to validate the GenesisState.
func (AppModuleBasic) ValidateGenesis(cdc codec.JSONCodec, config client.TxEncodingConfig, bz json.RawMessage) error {
	var genState types.GenesisState
	if err := cdc.UnmarshalJSON(bz, &genState); err != nil {
		return fmt.Errorf("failed to unmarshal %s genesis state: %w", types.ModuleName, err)
	}
	return genState.Validate()
}

// RegisterGRPCGatewayRoutes registers the gRPC Gateway routes for the module.
func (AppModuleBasic) RegisterGRPCGatewayRoutes(clientCtx client.Context, mux *runtime.ServeMux) {
	// Register gRPC gateway routes here if needed
}

// GetTxCmd returns the module's root tx command.
func (a AppModuleBasic) GetTxCmd() *cobra.Command {
	// Return tx command here if needed
	return &cobra.Command{}
}

// GetQueryCmd returns the module's root query command.
func (a AppModuleBasic) GetQueryCmd() *cobra.Command {
	// Return query command here if needed
	return &cobra.Command{}
}

// AppModule implements an application module for the KYC module.
type AppModule struct {
	AppModuleBasic

	keeper keeper.Keeper
}

// IsAppModule implements module.AppModule.
func (a AppModule) IsAppModule() {}

// IsOnePerModuleType implements module.AppModule.
func (a AppModule) IsOnePerModuleType() {}

// Name implements module.AppModule.
func (a AppModule) Name() string {
	return types.ModuleName
}

// RegisterGRPCGatewayRoutes implements module.AppModule.
func (a AppModule) RegisterGRPCGatewayRoutes(clientCtx client.Context, mux *runtime.ServeMux) {
	// Register gRPC gateway routes here if needed
}

// RegisterInterfaces implements module.AppModule.
func (a AppModule) RegisterInterfaces(reg cdctypes.InterfaceRegistry) {
	// Register interfaces here if needed
}

// RegisterLegacyAminoCodec implements module.AppModule.
func (a AppModule) RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {}

// InitGenesis performs genesis initialization for the KYC module.
func (am AppModule) InitGenesis(ctx sdk.Context, cdc codec.JSONCodec, gs json.RawMessage) {
	var genState types.GenesisState
	cdc.MustUnmarshalJSON(gs, &genState)
	am.keeper.InitGenesis(ctx, genState)
}

// ExportGenesis returns the exported genesis state as raw bytes for the KYC module.
func (am AppModule) ExportGenesis(ctx sdk.Context, cdc codec.JSONCodec) json.RawMessage {
	genState := am.keeper.ExportGenesis(ctx)
	return cdc.MustMarshalJSON(genState)
}

// ConsensusVersion implements AppModule/ConsensusVersion.
func (AppModule) ConsensusVersion() uint64 { return 1 }

func NewAppModule(
	cdc codec.Codec,
	keeper keeper.Keeper,
) AppModule {
	return AppModule{
		AppModuleBasic: NewAppModuleBasic(cdc),
		keeper:         keeper,
	}
}
