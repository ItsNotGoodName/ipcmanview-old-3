package main

import (
	"log"

	"github.com/ItsNotGoodName/ipcmanview/ipcmanview-server/server"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/plugins/migratecmd"

	// uncomment once you have at least one .go migration file in the "migrations" directory
	_ "github.com/ItsNotGoodName/ipcmanview/ipcmanview-server/migrations"
)

func main() {
	app := pocketbase.New()

	migratecmd.MustRegister(app, app.RootCmd, &migratecmd.Options{
		Automigrate: true, // auto creates migration files when making collection changes
	})

	server.AddRoutes(app)

	server.OnStation(app)
	server.OnPermission(app)

	if err := app.Start(); err != nil {
		log.Fatal(err)
	}
}
