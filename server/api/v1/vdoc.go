package v1

import (
	"github.com/gogf/gf/v2/frame/g"
	"server/internal/model/entity"
)

type VdocAddReq struct {
	g.Meta `path:"/vdoc" method:"post" tags:"Vdoc" summary:"vdoc add"`
	Vdoc   entity.Vdoc `json:"vdoc"`
}

type VdocListReq struct {
	g.Meta `path:"/vdoc" method:"get" tags:"Vdoc" summary:"vdoc list"`
}

type VdocGetReq struct {
	g.Meta `path:"/vdoc/:id" method:"get" tags:"Vdoc" summary:"vdoc get by id"`
	Id     string `json:"id"`
}

type VdocUpdReq struct {
	g.Meta `path:"/vdoc/:id" method:"put" tags:"Vdoc" summary:"vdoc update"`
	Vdoc   entity.Vdoc `json:"vdoc"`
}

type VdocDelReq struct {
	g.Meta `path:"/vdoc/:id" method:"delete" tags:"Vdoc" summary:"vdoc delete"`
	Id     string `json:"id"`
}

type VdocRes struct {
	Vdoc entity.Vdoc `json:"vdoc"`
}
