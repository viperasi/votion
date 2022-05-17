package v1

import (
	"github.com/gogf/gf/v2/frame/g"
	"time"
)

//region auth controller
type UserLoginReq struct {
	g.Meta   `path:"/user/login" method:"post" tags:"User" summary:"user login."`
	Username string `json:"username"`
	Passwd   string `json:"passwd"`
}

type UserRefreshTokenReq struct {
	g.Meta `path:"/user/refresh_token" method:"post" tags:"User" summary:"user refresh token"`
}

type UserTokenRes struct {
	Token  string    `json:"token"`
	Expire time.Time `json:"expire"`
}

type UserLogoutReq struct {
	g.Meta `path:"/user/logout" method:"post" tags:"User" summary:"user logout"`
}

type UserLogoutRes struct {
}

//endregion
