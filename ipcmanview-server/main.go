package main

import (
	"log"
	"net/url"

	"github.com/ItsNotGoodName/ipcmanview/ipcmanview-server/server"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/core"
	"github.com/pocketbase/pocketbase/plugins/migratecmd"

	// uncomment once you have at least one .go migration file in the "migrations" directory
	_ "github.com/ItsNotGoodName/ipcmanview/ipcmanview-server/migrations"
)

func main() {
	app := pocketbase.New()

	migratecmd.MustRegister(app, app.RootCmd, &migratecmd.Options{
		Automigrate: true, // auto creates migration files when making collection changes
	})

	app.OnBeforeServe().Add(server.Routes(app))

	app.OnRecordBeforeCreateRequest("stations").Add(func(e *core.RecordCreateEvent) error {
		// Default name to hostname
		if e.Record.GetString("name") == "" {
			urL, err := url.Parse(e.Record.GetString("url"))
			if err != nil {
				return err
			}

			e.Record.Set("name", urL.Hostname())
		}

		return nil
	})

	if err := app.Start(); err != nil {
		log.Fatal(err)
	}
}
