package entity

type User struct {
	Id       interface{} `json:"id" bson:"_id,omitempty"`
	Username string      `json:"username"`
	Passwd   string      `json:"passwd"`
	Nickname string      `json:"nickname"`
	Headpic  string      `json:"headpic"`
}
