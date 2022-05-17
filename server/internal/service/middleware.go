package service

import (
	"github.com/gogf/gf/v2/net/ghttp"
)

type (
	// sMiddleware is service struct of module Middleware.
	sUserMiddleware struct{}
)

var (
	// insMiddleware is the instance of service Middleware.
	insUserMiddleware = sUserMiddleware{}
)

// Middleware returns the interface of Middleware service.
func Middleware() *sUserMiddleware {
	return &insUserMiddleware
}

func (s *sUserMiddleware) CORS(r *ghttp.Request) {
	r.Response.CORSDefault()
	r.Middleware.Next()
}

func (s *sUserMiddleware) Auth(r *ghttp.Request) {
	//admin user auth mid
	Auth().MiddlewareFunc()(r)
	r.Middleware.Next()
}
