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
				"updated": "2023-05-09 22:47:17.047Z",
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
				"listRule": "id ?= @request.auth.access_stations.id",
				"viewRule": "id ?= @request.auth.access_stations.id",
				"createRule": null,
				"updateRule": null,
				"deleteRule": null,
				"options": {}
			},
			{
				"id": "_pb_users_auth_",
				"created": "2023-05-09 03:56:27.282Z",
				"updated": "2023-05-09 22:41:42.120Z",
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
					},
					{
						"system": false,
						"id": "jtgee2us",
						"name": "access_stations",
						"type": "relation",
						"required": false,
						"unique": false,
						"options": {
							"collectionId": "yugkj86nauvh93p",
							"cascadeDelete": false,
							"minSelect": null,
							"maxSelect": null,
							"displayFields": []
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
