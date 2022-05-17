package controller

import (
	"context"
	v1 "server/api/v1"
	"server/internal/service"
)

//region vdoc
type cVdoc struct{}

var Vdoc = cVdoc{}

func (c *cVdoc) VdocAdd(ctx context.Context, req *v1.VdocAddReq) (res *v1.VdocRes, err error) {
	res = &v1.VdocRes{}
	vdoc, _ := service.Vdoc().AddDoc(ctx, &req.Vdoc)
	res.Vdoc = *vdoc
	return
}

func (c *cVdoc) VdocUpd(ctx context.Context, req *v1.VdocUpdReq) (res *v1.VdocRes, err error) {
	return
}

func (c *cVdoc) VdocDel(ctx context.Context, req *v1.VdocDelReq) (res *v1.VdocRes, err error) {
	return
}

func (c *cVdoc) VdocList(ctx context.Context, req *v1.VdocListReq) (res *v1.VdocRes, err error) {
	return
}

func (c *cVdoc) VdocGetId(ctx context.Context, req *v1.VdocGetReq) (res *v1.VdocRes, err error) {
	return
}

//endregion
