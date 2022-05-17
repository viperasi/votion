package service

import (
	"context"
	"github.com/gogf/gf/v2/frame/g"
	"github.com/gogf/gf/v2/os/gctx"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
	"log"
)

func GetConnection() (*mongo.Client, error) {
	ctx := gctx.New()
	url, _ := g.Cfg().Get(ctx, "mongodb.link")
	client, err := mongo.Connect(context.Background(), options.Client().ApplyURI(url.String()))
	if err != nil {
		log.Fatal(err)
	}
	err = client.Ping(context.Background(), readpref.Primary())
	if err != nil {
		log.Fatal(err)
	}
	return client, nil
}

func GetCollection(collectionName string) (*mongo.Collection, error) {
	ctx := gctx.New()
	database, _ := g.Cfg().Get(ctx, "mongodb.database")
	client, err := GetConnection()
	if err != nil {
		return nil, err
	}
	collection := client.Database(database.String()).Collection(collectionName)
	return collection, nil
}
