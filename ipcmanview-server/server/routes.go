package server

import (
	ui "github.com/ItsNotGoodName/ipcmanview-ui"
	"github.com/labstack/echo/v5/middleware"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
)

func AddRoutes(app *pocketbase.PocketBase) {
	app.OnBeforeServe().Add(func(e *core.ServeEvent) error {
		activityLogger := apis.ActivityLogger(app)
		a := e.Router.Group("/app")

		{
			stationProxy := stationProxy(app)
			stationAuth := stationAuth(app)
			loadAuthContextFromCookie := LoadAuthContextFromCookie(app)
			a.Any("/stations/:id/*", stationProxy, activityLogger, loadAuthContextFromCookie, stationAuth)
			a.Any("/stations/:id", stationProxy, activityLogger, loadAuthContextFromCookie, stationAuth)
		}

		e.Router.GET("/*", apis.StaticDirectoryHandler(ui.FS, true), middleware.Gzip())

		return nil
	})
}
