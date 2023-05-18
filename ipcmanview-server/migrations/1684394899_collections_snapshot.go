package migrations

import (
	"encoding/json"

	"github.com/pocketbase/dbx"
	"github.com/pocketbase/pocketbase/daos"
	m "github.com/pocketbase/pocketbase/migrations"
	"github.com/pocketbase/pocketbase/models"
)

func init() {
	m.Register(func(db dbx.Builder) error {
		jsonData := `[
			{
				"id": "yugkj86nauvh93p",
				"created": "2023-05-09 00:31:36.129Z",
				"updated": "2023-05-18 07:20:36.612Z",
				"name": "stations",
				"type": "base",
				"system": false,
				"schema": [
					{
						"system": false,
						"id": "iotmwfur",
						"name": "url",
						"type": "url",
						"required": true,
						"unique": false,
						"options": {
							"exceptDomains": [],
							"onlyDomains": []
						}
					},
					{
						"system": false,
						"id": "ngxxbb0p",
						"name": "name",
						"type": "text",
						"required": true,
						"unique": false,
						"options": {
							"min": null,
							"max": 64,
							"pattern": ""
						}
					}
				],
				"indexes": [
					"CREATE UNIQUE INDEX ` + "`" + `idx_goOFdFl` + "`" + ` ON ` + "`" + `stations` + "`" + ` (` + "`" + `url` + "`" + `)",
					"CREATE UNIQUE INDEX ` + "`" + `idx_S38EZva` + "`" + ` ON ` + "`" + `stations` + "`" + ` (` + "`" + `name` + "`" + `)"
				],
				"listRule": "@request.auth.id ?= @collection.permissions.user.id && id ?= @collection.permissions.station.id",
				"viewRule": "@request.auth.id ?= @collection.permissions.user.id && id ?= @collection.permissions.station.id",
				"createRule": null,
				"updateRule": null,
				"deleteRule": null,
				"options": {}
			},
			{
				"id": "_pb_users_auth_",
				"created": "2023-05-17 08:14:18.793Z",
				"updated": "2023-05-18 04:50:29.879Z",
				"name": "users",
				"type": "auth",
				"system": false,
				"schema": [
					{
						"system": false,
						"id": "users_name",
						"name": "name",
						"type": "text",
						"required": false,
						"unique": false,
						"options": {
							"min": null,
							"max": null,
							"pattern": ""
						}
					},
					{
						"system": false,
						"id": "users_avatar",
						"name": "avatar",
						"type": "file",
						"required": false,
						"unique": false,
						"options": {
							"maxSelect": 1,
							"maxSize": 5242880,
							"mimeTypes": [
								"image/jpeg",
								"image/png",
								"image/svg+xml",
								"image/gif",
								"image/webp"
							],
							"thumbs": null,
							"protected": false
						}
					}
				],
				"indexes": [],
				"listRule": "id = @request.auth.id",
				"viewRule": "id = @request.auth.id",
				"createRule": "",
				"updateRule": "id = @request.auth.id",
				"deleteRule": "id = @request.auth.id",
				"options": {
					"allowEmailAuth": true,
					"allowOAuth2Auth": true,
					"allowUsernameAuth": true,
					"exceptEmailDomains": null,
					"manageRule": null,
					"minPasswordLength": 8,
					"onlyEmailDomains": null,
					"requireEmail": false
				}
			},
			{
				"id": "i5olmj6c13jfrcn",
				"created": "2023-05-18 05:39:39.718Z",
				"updated": "2023-05-18 06:43:59.431Z",
				"name": "permissions",
				"type": "base",
				"system": false,
				"schema": [
					{
						"system": false,
						"id": "fatyxent",
						"name": "user",
						"type": "relation",
						"required": true,
						"unique": false,
						"options": {
							"collectionId": "_pb_users_auth_",
							"cascadeDelete": true,
							"minSelect": null,
							"maxSelect": 1,
							"displayFields": []
						}
					},
					{
						"system": false,
						"id": "sxr1ljhn",
						"name": "station",
						"type": "relation",
						"required": true,
						"unique": false,
						"options": {
							"collectionId": "yugkj86nauvh93p",
							"cascadeDelete": true,
							"minSelect": null,
							"maxSelect": 1,
							"displayFields": []
						}
					},
					{
						"system": false,
						"id": "deigmibk",
						"name": "role",
						"type": "select",
						"required": true,
						"unique": false,
						"options": {
							"maxSelect": 1,
							"values": [
								"viewer",
								"manager"
							]
						}
					},
					{
						"system": false,
						"id": "wms8olbk",
						"name": "cameras",
						"type": "json",
						"required": false,
						"unique": false,
						"options": {}
					}
				],
				"indexes": [
					"CREATE INDEX ` + "`" + `idx_jbtjZGB` + "`" + ` ON ` + "`" + `permissions` + "`" + ` (\n  ` + "`" + `user` + "`" + `,\n  ` + "`" + `station` + "`" + `\n)"
				],
				"listRule": null,
				"viewRule": null,
				"createRule": null,
				"updateRule": null,
				"deleteRule": null,
				"options": {}
			}
		]`

		collections := []*models.Collection{}
		if err := json.Unmarshal([]byte(jsonData), &collections); err != nil {
			return err
		}

		return daos.New(db).ImportCollections(collections, true, nil)
	}, func(db dbx.Builder) error {
		return nil
	})
}
