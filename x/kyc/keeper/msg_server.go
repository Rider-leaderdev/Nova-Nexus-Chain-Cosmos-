package keeper

import (
	"context"

	"github.com/SolanaPan/nnb/v1/x/kyc/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type msgServer struct {
	Keeper
}

func NewMsgServerImpl(keeper Keeper) types.MsgServer {
	return &msgServer{Keeper: keeper}
}

func (m msgServer) SubmitKYC(goCtx context.Context, msg *types.MsgSubmitKYC) (*types.MsgSubmitKYCResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	m.SetKYCStatus(ctx, msg.Address, types.KYCPending)
	return &types.MsgSubmitKYCResponse{}, nil
}

func (m msgServer) ApproveKYC(goCtx context.Context, msg *types.MsgApproveKYC) (*types.MsgApproveKYCResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	m.SetKYCStatus(ctx, msg.Address, types.KYCApproved)
	return &types.MsgApproveKYCResponse{}, nil
}

func (m msgServer) RejectKYC(goCtx context.Context, msg *types.MsgRejectKYC) (*types.MsgRejectKYCResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	m.SetKYCStatus(ctx, msg.Address, types.KYCRejected)
	return &types.MsgRejectKYCResponse{}, nil
}
