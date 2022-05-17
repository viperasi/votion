package entity

type Vdoc struct {
	Id          interface{}   `json:"id" bson:"_id,omitempty"`
	Title       string        `json:"title"`
	Version     string        `json:"version"`
	Name        string        `json:"name"`
	Children    []interface{} `json:"children"`
	Parent      interface{}   `json:"parent"`
	Type        string        `json:"type"`
	CreatedTime int           `json:"created_time"`
	UpdatedTime int           `json:"updated_time"`
	Schema      Vschema       `json:"schema"`
}

type Vschema struct {
	Id      interface{}     `json:"id" bson:"_id,omitempty"`
	Name    string          `json:"name"`
	Type    string          `json:"type"`
	Options []VschemeOption `json:"options"`
}

type VschemeOption struct {
	Color string `json:"color"`
	Value string `json:"value"`
}
