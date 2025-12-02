package keeper

import (
	"cosmossdk.io/store/prefix"
	"github.com/SolanaPan/nnb/v1/x/tokenfactory/types"
	"github.com/cosmos/cosmos-sdk/runtime"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// StoreEscrowAddress sets the total set of params.
func (k Keeper) StoreEscrowAddress(ctx sdk.Context, address sdk.AccAddress) {
	store := runtime.KVStoreAdapter(k.storeService.OpenKVStore(ctx))
	prefixStore := prefix.NewStore(store, types.EscrowAddressKey)
	prefixStore.Set(address.Bytes(), []byte{0})
}

func (k Keeper) IsEscrowAddress(ctx sdk.Context, address sdk.AccAddress) bool {
	store := runtime.KVStoreAdapter(k.storeService.OpenKVStore(ctx))
	prefixStore := prefix.NewStore(store, types.EscrowAddressKey)
	bz := prefixStore.Get(address.Bytes())

	return len(bz) != 0
}
