package server

import (
	ui "github.com/ItsNotGoodName/ipcmanview-ui"
	"github.com/labstack/echo/v5/middleware"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
)

func Routes(app *pocketbase.PocketBase) func(e *core.ServeEvent) error {
	return func(e *core.ServeEvent) error {
		activityLogger := apis.ActivityLogger(app)

		{
			stationProxy := stationProxy(app)
			e.Router.Any("/stations/:id/*", stationProxy, activityLogger)
			e.Router.Any("/stations/:id", stationProxy, activityLogger)
		}

		{
			e.Router.GET("/*", apis.StaticDirectoryHandler(ui.FS, true), middleware.Gzip())
		}

		return nil
	}
}
