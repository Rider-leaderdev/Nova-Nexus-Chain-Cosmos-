package keeper

import (
	storetypes "cosmossdk.io/store/types"
	"github.com/SolanaPan/nnb/v1/x/kyc/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type Keeper struct {
	storeKey storetypes.StoreKey
}

func NewKeeper(storeKey storetypes.StoreKey) Keeper {
	return Keeper{storeKey: storeKey}
}

func (k Keeper) SetKYCStatus(ctx sdk.Context, addr sdk.AccAddress, status types.KYCStatus) {
	store := ctx.KVStore(k.storeKey)
	store.Set(addr.Bytes(), []byte{byte(status)})
}

func (k Keeper) GetKYCStatus(ctx sdk.Context, addr sdk.AccAddress) types.KYCStatus {
	store := ctx.KVStore(k.storeKey)
	bz := store.Get(addr.Bytes())
	if len(bz) == 0 {
		return types.KYCUnknown
	}
	return types.KYCStatus(bz[0])
}

// InitGenesis initializes the KYC module's genesis state.
func (k Keeper) InitGenesis(ctx sdk.Context, genState types.GenesisState) {
	// Initialize genesis state here if needed
}

// ExportGenesis returns the KYC module's exported genesis state.
func (k Keeper) ExportGenesis(ctx sdk.Context) *types.GenesisState {
	return types.DefaultGenesis()
}
