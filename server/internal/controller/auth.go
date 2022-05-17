package controller

import (
	"context"
	v1 "server/api/v1"
	"server/internal/service"
)

//region user auth
type cAuth struct{}

var Auth = cAuth{}

func (c *cAuth) AdminUserLogin(ctx context.Context, req *v1.UserLoginReq) (res *v1.UserTokenRes, err error) {
	res = &v1.UserTokenRes{}
	res.Token, res.Expire = service.Auth().LoginHandler(ctx)
	return
}

func (c *cAuth) RefreshToken(ctx context.Context, req *v1.UserRefreshTokenReq) (res *v1.UserTokenRes, err error) {
	res = &v1.UserTokenRes{}
	res.Token, res.Expire = service.Auth().RefreshHandler(ctx)
	return
}

func (c *cAuth) Logout(ctx context.Context, req *v1.UserLogoutReq) (res *v1.UserLogoutRes, err error) {
	service.Auth().LogoutHandler(ctx)
	return
}

//endregion
