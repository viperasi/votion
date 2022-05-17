package service

import (
	"context"
	"github.com/gogf/gf/v2/errors/gerror"
	"github.com/gogf/gf/v2/frame/g"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"server/internal/model/entity"
)

const (
	collection_user = "user"
)

type (
	// sUser is service struct of module User.
	sUser struct{}
)

var (
	// insUser is the instance of service User.
	userService = sUser{}
)

// User returns the interface of User service.
func User() *sUser {
	return &userService
}

//获取用户信息
func (s *sUser) GetById(ctx context.Context, id string) (user *entity.User, err error) {
	if id != "" {
		collection, err := GetCollection(collection_user)
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
			return nil, gerror.Newf("找不到该用户[%s]: %+v", id, result.Err())
		}
		user := entity.User{}
		outErr := result.Decode(&user)
		if outErr != nil {
			return nil, gerror.Newf("用户转换失败: %+v", outErr)
		}
		user.Passwd = ""
		return &user, nil
	} else {
		return nil, gerror.Newf("用户id为空: %s", id)
	}
}

// user login
// must return map[string]interface{} for gf-jwt PayloadFunc
func (s *sUser) Login(ctx context.Context, username string, passwd string) (cuser map[string]interface{}, err error) {
	g.Log().Info(ctx, "User Login---->", username, passwd)

	if username != "" && passwd != "" {
		collection, err := GetCollection(collection_user)
		if err != nil {
			return nil, err
		}
		filter := bson.M{"username": username}
		result := collection.FindOne(context.Background(), filter)
		if result.Err() != nil {
			return nil, gerror.Newf("找不到该用户: %+v", result.Err())
		}
		user := entity.User{}
		outErr := result.Decode(&user)
		if outErr != nil {
			return nil, gerror.Newf("用户转换失败: %+v", outErr)
		}
		if user.Passwd == passwd {
			return g.Map{
				"id":       user.Id.(primitive.ObjectID).Hex(),
				"username": user.Username,
				"nickname": user.Nickname,
			}, nil
		} else {
			return nil, gerror.Newf("找不到该用户: %s", username)
		}
	} else {
		return nil, gerror.Newf("用户名密码不能为空")
	}
}
