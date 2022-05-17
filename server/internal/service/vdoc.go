package service

import (
	"context"
	"github.com/gogf/gf/v2/errors/gerror"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"server/internal/model/entity"
)

const (
	collection_vdoc    = "vdoc"
	collection_vschema = "vschema"
)

type (
	// sUser is service struct of module User.
	sVdoc struct{}
)

var (
	// insUser is the instance of service User.
	vdocService = sVdoc{}
)

// User returns the interface of User service.
func Vdoc() *sVdoc {
	return &vdocService
}

//获取文档
func (s *sVdoc) GetById(ctx context.Context, id string) (vdoc *entity.Vdoc, err error) {
	if id != "" {
		collection, err := GetCollection(collection_vdoc)
		if err != nil {
			return nil, err
		}
		objId, errr := primitive.ObjectIDFromHex(id)
		if errr != nil {
			return nil, gerror.Newf("id[s]无法转换: %v", id, err)
		}
		filter := bson.M{"_id": objId}
		result := collection.FindOne(context.Background(), filter)
		if result.Err() != nil {
			return nil, gerror.Newf("找不到文档[%s]: %+v", id, result.Err())
		}
		vdoc := entity.Vdoc{}
		outErr := result.Decode(&vdoc)
		if outErr != nil {
			return nil, gerror.Newf("文档转换失败: %+v", outErr)
		}
		return &vdoc, nil
	} else {
		return nil, gerror.Newf("用户id为空: %s", id)
	}
}

//region doc
func (s *sVdoc) AddDoc(ctx context.Context, in *entity.Vdoc) (vdoc *entity.Vdoc, err error) {
	//get connection and collection
	collection, err := GetCollection(collection_vdoc)
	if err != nil {
		return in, err
	}
	//new ObjectID, must set objectId otherwise id is null
	in.Id = primitive.NewObjectID()
	//insert doc
	_, err = collection.InsertOne(context.Background(), in)
	if err != nil {
		return in, gerror.Newf("插入失败: %v", err)
	}
	//return
	return in, nil
}

//endregion

//region schema

func (s *sVdoc) AddSchema(ctx context.Context, in *entity.Vschema) (vschema *entity.Vschema, err error) {
	//get connection and collection
	collection, err := GetCollection(collection_vschema)
	if err != nil {
		return in, err
	}
	//new ObjectID, must set objectId otherwise id is null
	in.Id = primitive.NewObjectID()
	//insert doc
	_, err = collection.InsertOne(context.Background(), in)
	if err != nil {
		return in, gerror.Newf("插入失败: %v", err)
	}
	//return
	return in, nil
}

//endregion
