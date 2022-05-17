一个替代notion, 支持web和客户端的开源工具.

## 功能

* 初始化
* 用户管理
* 文档管理
* 博客管理

## 用户管理

管理当前唯一用户的登录和管理, 包括:

* 忘记密码
* 用户登录
* 修改密码

## 初始化

系统安装时自动初始化系统相关内容,并设置默认账号和密码, 包括:

* 环境安装
* 前端部署
* 数据库初始化
* 用户初始化
* 博客初始化

## 文档管理

管理用户的创建的文档, 包括:

* 文档层级管理
* 文档链接管理

## 博客管理

用户可开放博客系统,将指定的文档开放为博客, 包括:

* 域名设置
* theme设置
* 内容管理

## 开发选型

* 后台: go
* 前端: vue
* 数据库: mongodb

## notion数据结构分析

```json
{
    "result": {
        "type": "reducer",
        "reducerResults": {
            "collection_group_results": {
                "type": "results",
                "blockIds": [
                    "227c6aae-b54a-4a9c-92e8-12832fa653cd",
                    "43f9fa74-bc7d-4a2f-9f5e-89604db7bfb5",
                    "55eec2d8-b46c-4ab3-82ae-5e0511ea33d3"
                ],
                "hasMore": false
            }
        }
    },
    "recordMap": {
        //内容块
        "block": {
            //uuid
            "227c6aae-b54a-4a9c-92e8-12832fa653cd": {
                "role": "editor",
                //内容
                "value": {
                    "id": "227c6aae-b54a-4a9c-92e8-12832fa653cd",
                    "version": 33,
                    //内置类型
                    "type": "page",
                    //属性
                    "properties": {
                        //对应属性和值
                        "jJIv": [
                            [
                                "已通过"
                            ]
                        ],
                        "title": [
                            [
                                "开发计划"
                            ]
                        ]
                    },
                    //内容连接
                    "content": [
                        "44b1c7fe-f74f-46e2-9c8a-e0734c02c0bb"
                    ],
                    "created_time": 1641637020000,
                    "last_edited_time": 1642500660000,
                    "parent_id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                    "parent_table": "collection",
                    "alive": true,
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                }
            },
            "43f9fa74-bc7d-4a2f-9f5e-89604db7bfb5": {
                "role": "editor",
                "value": {
                    "id": "43f9fa74-bc7d-4a2f-9f5e-89604db7bfb5",
                    "version": 70,
                    "type": "page",
                    "properties": {
                        "8nXq": [
                            [
                                "Technical Spec"
                            ]
                        ],
                        "jJIv": [
                            [
                                "已通过"
                            ]
                        ],
                        "x<$~": [
                            [
                                "‣",
                                [
                                    [
                                        "u",
                                        "adf1976a-dd9f-4277-9521-219f43715666"
                                    ]
                                ]
                            ]
                        ],
                        "title": [
                            [
                                "功能列表"
                            ]
                        ]
                    },
                    "content": [
                        "4d83aa7f-4cd4-4e7f-940b-ad9d31688456",
                        "f7c858a7-6f55-408d-ad13-731d4328a3b0",
                        "df30010f-56d4-4672-ad16-e6c698516d33"
                    ],
                    "format": {
                        "copied_from_pointer": {
                            "id": "f08498f9-7dd2-41ce-a747-6be347bb39c0",
                            "table": "block",
                            "spaceId": "1f88cc90-92fd-4ce4-bfcd-25daec2ffbbe"
                        }
                    },
                    "created_by": "adf1976a-dd9f-4277-9521-219f43715666",
                    "created_time": 1641633035490,
                    "last_edited_by": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_time": 1642494840000,
                    "parent_id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                    "parent_table": "collection",
                    "alive": true,
                    "copied_from": "f08498f9-7dd2-41ce-a747-6be347bb39c0",
                    "file_ids": [
                        "0096139f-8b26-479d-b63f-99d4de449b23",
                        "17ac3bf3-3dfa-4ca0-9c63-d7465ad99ab5",
                        "cfb4856b-2857-4d17-ad5a-46ce71119eb4",
                        "568ad95b-4973-4dca-940b-d4fce62af21d",
                        "97c635d9-3637-42e3-9f7a-05e51446529b",
                        "89f99c5d-c6b8-4490-aa63-f489b1eb0aae",
                        "d7fde1e3-4834-49b2-bdb7-5b731f385e64"
                    ],
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                }
            },
            "55eec2d8-b46c-4ab3-82ae-5e0511ea33d3": {
                "role": "editor",
                "value": {
                    "id": "55eec2d8-b46c-4ab3-82ae-5e0511ea33d3",
                    "version": 123,
                    "type": "page",
                    "properties": {
                        "8nXq": [
                            [
                                "Architecture Overview"
                            ]
                        ],
                        "jJIv": [
                            [
                                "已通过"
                            ]
                        ],
                        "x<$~": [
                            [
                                "‣",
                                [
                                    [
                                        "u",
                                        "b2f4fb03-3454-46a3-983d-6bb0d7995afc"
                                    ]
                                ]
                            ],
                            [
                                ","
                            ],
                            [
                                "‣",
                                [
                                    [
                                        "u",
                                        "bd3552d4-2e14-448e-b60d-8d0fe921a4ae"
                                    ]
                                ]
                            ]
                        ],
                        "title": [
                            [
                                "原型及设计"
                            ]
                        ]
                    },
                    "content": [
                        "b3436c14-704c-4add-8064-ac748184e6c8",
                        "be67bd54-75c1-458a-a755-bbf411c3dcf5",
                        "bdab07c5-9fe5-4226-b66c-17868c8f7dd0",
                        "10bbc7db-5aad-4ec5-9e98-545d09e983db"
                    ],
                    "format": {
                        "copied_from_pointer": {
                            "id": "48874c17-78a2-4071-917f-7e251bc55a44",
                            "table": "block",
                            "spaceId": "1f88cc90-92fd-4ce4-bfcd-25daec2ffbbe"
                        }
                    },
                    "created_by": "adf1976a-dd9f-4277-9521-219f43715666",
                    "created_time": 1641633035490,
                    "last_edited_by": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_time": 1642214940000,
                    "parent_id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                    "parent_table": "collection",
                    "alive": true,
                    "copied_from": "48874c17-78a2-4071-917f-7e251bc55a44",
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                }
            },
            "43ba42bc-d043-4cc7-a99a-10f06551a1e3": {
                "role": "editor",
                "value": {
                    "id": "43ba42bc-d043-4cc7-a99a-10f06551a1e3",
                    "version": 61,
                    "type": "collection_view_page",
                    "view_ids": [
                        "01b6727b-0155-4878-8f76-fad3acca0855"
                    ],
                    "collection_id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                    "format": {
                        "page_icon": "📓",
                        "block_locked": false,
                        "block_locked_by": "d0a8cba1-b998-46a3-bcd1-0efb420410b1",
                        "collection_pointer": {
                            "id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                            "table": "collection",
                            "spaceId": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                        },
                        "copied_from_pointer": {
                            "id": "697d17c4-4597-4949-9c24-c220aefc31f7",
                            "table": "block",
                            "spaceId": "1f88cc90-92fd-4ce4-bfcd-25daec2ffbbe"
                        }
                    },
                    "created_by": "09bf218a-16a3-4fba-b398-46b75242a1fe",
                    "created_time": 1641633035490,
                    "last_edited_by": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_time": 1642500780000,
                    "parent_id": "c961b437-05e4-43f4-8285-01111c7a30ee",
                    "parent_table": "block",
                    "alive": true,
                    "copied_from": "697d17c4-4597-4949-9c24-c220aefc31f7",
                    "file_ids": [
                        "ffbdd33d-0f3e-486d-8596-c6451cd8836b",
                        "fa6d2b9c-2c7e-49b1-9e81-6841a9420de2",
                        "d9132a82-4967-4b93-ae97-43d36c3f4dbb",
                        "ea46d788-de25-495a-8724-419fdb4b3e5b",
                        "83832a8f-955f-420b-91f4-b980f25bb2c8",
                        "47d79812-ee38-486f-aafc-844605820bb6",
                        "21115949-45fd-462a-8dba-7c45d6e82313",
                        "47594516-7eee-4ed9-ad48-80e6a4b02dee",
                        "c4aeb953-014d-4c20-bfc7-5949426f27ac",
                        "d67ee9ec-fdad-4810-9d33-c905e6127ef9",
                        "89c40d79-21ad-4aa0-a355-296a9956ecdf",
                        "1e8ac41a-960d-42af-a589-d8881095cc67",
                        "630a0426-fc2a-4b14-b6c2-1211daca3036",
                        "348247ee-ddb9-4756-b78e-668456015e4e",
                        "af064b87-f621-47c3-844e-4ec43c110596",
                        "815bbef1-2ac7-4c3d-b8a6-8a33c5627fbf",
                        "9ebfdca1-34fc-49aa-b2cd-314828134927",
                        "1dc96f8d-7e43-466e-b1b4-b58a27359ebf",
                        "150cf840-db6c-416f-8379-f5fdfba1aace",
                        "1f8ee26b-43b1-40b1-a9f6-531343d266e4",
                        "70fdb2ac-2d86-4214-a448-078ffc02ddf9"
                    ],
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                }
            },
            "c961b437-05e4-43f4-8285-01111c7a30ee": {
                "role": "editor",
                "value": {
                    "id": "c961b437-05e4-43f4-8285-01111c7a30ee",
                    "version": 42,
                    "type": "page",
                    "properties": {
                        "title": [
                            [
                                "已完成"
                            ]
                        ]
                    },
                    "content": [
                        "6e429630-2692-40a9-823c-f752e911ed5f",
                        "43ba42bc-d043-4cc7-a99a-10f06551a1e3",
                        "ced136bd-2d71-49bd-a453-d0f530ff90e6"
                    ],
                    "permissions": [
                        {
                            "role": "editor",
                            "type": "user_permission",
                            "user_id": "061e8601-496d-480b-b6ce-289b0aa17a09"
                        }
                    ],
                    "created_time": 1642500720000,
                    "last_edited_time": 1647828480000,
                    "parent_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f",
                    "parent_table": "space",
                    "alive": true,
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f"
                }
            }
        },
        //集合
        "collection": {
            //uuid
            "affacb9a-f845-404a-a52c-61d99c0340d4": {
                "role": "editor",
                "value": {
                    "id": "affacb9a-f845-404a-a52c-61d99c0340d4",
                    //版本
                    "version": 14,
                    //标题
                    "name": [
                        [
                            "鑫源收费工具文档"
                        ]
                    ],
                    //描述
                    "description": [
                        [
                            "鑫源收费工具文档集合"
                        ]
                    ],
                    //属性定义
                    "schema": {
                        //随机数
                        "(kk8": {
                            //名称
                            "name": "Created By",
                            //内置类型
                            "type": "created_by"
                        },
                        "0jxo": {
                            "name": "Created",
                            "type": "created_time"
                        },
                        "jJIv": {
                            "name": "Status",
                            "type": "select",
                            //select 配置
                            "options": [
                                {
                                    "id": "025c7ddc-113c-4e6b-9c36-1fd806e614da",
                                    "color": "yellow",
                                    "value": "进行中"
                                },
                                {
                                    "id": "4d776c96-0f23-4815-b661-c4b4f0892cd0",
                                    "color": "orange",
                                    "value": "审核中"
                                },
                                {
                                    "id": "4edc6f8b-ac8c-4415-8a2a-f26176b51939",
                                    "color": "green",
                                    "value": "已通过"
                                },
                                {
                                    "id": "fabf421f-2d1f-4b82-bd9c-1525f76fef29",
                                    "color": "gray",
                                    "value": "已废弃"
                                }
                            ]
                        },
                        "u_Xe": {
                            "name": "Last Edited Time",
                            "type": "last_edited_time"
                        },
                        "u`\\{": {
                            "name": "Last Edited By",
                            "type": "last_edited_by"
                        },
                        "title": {
                            "name": "Name",
                            "type": "title"
                        }
                    },
                    //图标
                    "icon": "📎",
                    //格式化
                    "format": {
                        "copied_from_pointer": {
                            "id": "e0067c74-c315-43ff-bb87-49efb687880a",
                            "table": "collection",
                            "spaceId": "1f88cc90-92fd-4ce4-bfcd-25daec2ffbbe"
                        },
                        //集合内页面属性配置
                        "collection_page_properties": [
                            {
                                "visible": true,
                                "property": "(kk8"
                            },
                            {
                                "visible": true,
                                "property": "x<$~"
                            },
                            {
                                "visible": false,
                                "property": "jJIv"
                            },
                            {
                                "visible": false,
                                "property": "8nXq"
                            },
                            {
                                "visible": false,
                                "property": "0jxo"
                            },
                            {
                                "visible": true,
                                "property": "u_Xe"
                            },
                            {
                                "visible": true,
                                "property": "u`\\{"
                            }
                        ]
                    },
                    //父级
                    "parent_id": "43ba42bc-d043-4cc7-a99a-10f06551a1e3",
                    //父级标签
                    "parent_table": "block",
                    "alive": true,
                    "copied_from": "e0067c74-c315-43ff-bb87-49efb687880a",
                    "migrated": true,
                    "space_id": "2f70b155-4fdc-48f9-8ced-a3417285c25f",
                    //已删除属性
                    "deleted_schema": {
                        "8nXq": {
                            "name": "Type",
                            "type": "select",
                            "options": [
                                {
                                    "id": "822b14d8-9d75-4dad-91dc-f8917c51910c",
                                    "color": "yellow",
                                    "value": "Project Kickoff 🚀"
                                },
                                {
                                    "id": "fd761e8f-b8bf-407e-8296-e7e0d1167f2d",
                                    "color": "blue",
                                    "value": "Technical Spec"
                                },
                                {
                                    "id": "7ed9a4aa-016c-4d13-8219-0f90f9ba93d9",
                                    "color": "purple",
                                    "value": "Architecture Overview"
                                }
                            ]
                        },
                        "x<$~": {
                            "name": "Stakeholders",
                            "type": "person"
                        }
                    }
                }
            }
        },
        //空间
        "space": {
            //uuid
            "2f70b155-4fdc-48f9-8ced-a3417285c25f": {
                //权限
                "role": "editor",
                //内容
                "value": {
                    "id": "2f70b155-4fdc-48f9-8ced-a3417285c25f",
                    //版本
                    "version": 82,
                    //标题
                    "name": "viperasi's Notion",
                    //可使用权限
                    "permissions": [
                        {
                            "role": "editor",
                            "type": "user_permission",
                            "user_id": "061e8601-496d-480b-b6ce-289b0aa17a09"
                        }
                    ],
                    //启用测试
                    "beta_enabled": false,
                    //包含页面
                    "pages": [
                        "91a23ff4-f239-4c58-bf53-2da2d4eaae1e",
                        "1b5bbd28-2a18-4af9-a700-a03e5337fa33",
                        "6259ab1a-3bb3-4b86-b8fd-262e9b53a468",
                        "19a9ecce-c56e-4601-a23e-d9811f97756c",
                        "69f2462d-4427-4515-a705-51bc531ff130",
                        "a1e3f389-6810-4a9c-8b2f-1e94b21e9ed2",
                        "0d001c6a-fd4d-41a7-87f9-b4bc75badae9",
                        "47b400cc-dcc8-446c-8054-48807030c0ff",
                        "c961b437-05e4-43f4-8285-01111c7a30ee",
                        "0ea14817-0b97-424a-a8ea-b247d1ed04aa"
                    ],
                    "created_time": 1641218421193,
                    "last_edited_time": 1647970440000,
                    "created_by_table": "notion_user",
                    "created_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "last_edited_by_table": "notion_user",
                    "last_edited_by_id": "061e8601-496d-480b-b6ce-289b0aa17a09",
                    "plan_type": "personal",
                    "invite_link_enabled": true
                }
            }
        }
    }
}
```

## 详细设计

> User(用户)

```json
{
    "username":"",
    "passwd":"",
    "nickname":"",
    "headpic":"",
}
```

> DocType(文档类型)

```json
{
    "code":"",
    "name":"",
    "desc":"",
}
```

> DocType(文档类型-常量数据)

|code|name|desc|
|:-:|:-:|:-:|
|基本类型|-|-|
|text|Text|基本文本标记|
|page|Page|内嵌子页面|
|todo-list|To-do list|标准代办任务列表|
|heading1|Heading1|1级标题|
|heading2|Heading2|2级标题|
|heading3|Heading3|3级标题|
|bulleted-list|Bulleted list|无序列表|
|numbered-list|Numbered list|有序列表|
|toggle-list|Toggle list|可展开列表|
|quote|Quote|引用|
|divider|Divider|分割线|
|link-to-page|Link to page|链接到页面|
|callout|Callout|特别提示框|
|数据库类型|-|-|
|table-inline|Table-Inline|行内表格|
|board-inline|Board-Inline|行内公告板|
|gallery-inline|Gallery-Inline|行内图库|
|list-inline|List-Inline|行内列表|
|calendar-inline|Calendar-Inline|行内日历|
|table-fullpage|Table-Full Page|表格页面|
|board-fullpage|Board-Full Page|公告板页面|
|gallery-fullpage|Gallery-Full Page|图库页面|
|list-fullpage|List-Full Page|列表页面|
|calendar-fullpage|Calendar-Full Page|日历页面|
|媒体文件|-|-|
|image|Image|图片|
|video|Video|视频|
|audio|Audio|音频|
|file|File|文件|
|code|Code|代码片段|
|web-bookmark|Web Bookmark|网页书签|
|特殊|-|-|
|math|Math|数学公式|
|template-button|Template Button|模板按钮|
|breadcrumb|Breadcrumb|面包屑导航栏|
|toc|Table of Contents|文档目录|
|特殊指令|-|-|
|@<page>|Mention a page|快速链接到页面|
|@<person>|Mention a person|at某人|
|@<Date>|Date or reminder|创建纪念日|
|:<code>|Emoji|emoji表情|

> Documents(文档)

```json
{
    "title":"",
    "version":"",
    "name":"",
    "children":[@DBRef],
    "parent":{@DBRef},
    "type":"",
    "created_time":timestamp,
    "updated_time":timestamp,
}
```

> Schema(定义)

```json
{
    "name":"",
    "type":"",
    "options":{
        "color":"",
        "value":"",
    }
}
```




