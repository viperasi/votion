package cmd

import (
	"context"
	"server/internal/service"

	"github.com/gogf/gf/v2/frame/g"
	"github.com/gogf/gf/v2/net/ghttp"
	"github.com/gogf/gf/v2/os/gcmd"
	"server/internal/controller"
)

var (
	Main = gcmd.Command{
		Name:  "main",
		Usage: "main",
		Brief: "start http server",
		Func: func(ctx context.Context, parser *gcmd.Parser) (err error) {
			s := g.Server()
			s.Group("/", func(group *ghttp.RouterGroup) {
				group.Middleware(
					service.Middleware().CORS,
					ghttp.MiddlewareHandlerResponse,
				)
				group.Bind(
					controller.Hello,
					controller.Auth,
				)
				group.Group("/votion", func(group *ghttp.RouterGroup) {
					group.Middleware(
						service.Middleware().Auth,
						ghttp.MiddlewareHandlerResponse,
					)
					group.Bind(
						controller.Vdoc,
					)
				})
			})
			s.Run()
			return nil
		},
	}
)
