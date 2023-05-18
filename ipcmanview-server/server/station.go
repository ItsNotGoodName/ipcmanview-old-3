package server

import (
	"database/sql"
	"net/http/httputil"
	"net/url"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/dbx"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
	"github.com/pocketbase/pocketbase/models"
)

func stationAuth(app core.App) echo.MiddlewareFunc {
	const (
		ManagerRole = "manager"
		ViewerRole  = "viewer"
	)

	type Permissions struct {
		Role string
	}

	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			// Get user
			user, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)
			if user == nil {
				return apis.NewUnauthorizedError("", nil)
			}

			// Get permission record
			stationId := c.PathParam("id")
			var permission Permissions
			if err := app.
				DB().
				Select("role").
				From("permissions").
				Where(dbx.And(dbx.HashExp{"user": user.Id, "station": stationId})).
				Build().
				One(&permission); err != nil {
				if err == sql.ErrNoRows {
					return apis.NewNotFoundError("", nil)
				}

				return echo.ErrInternalServerError.WithInternal(err)
			}

			// Allow access to station based on permission
			if permission.Role == ManagerRole {
				return next(c)
			}
			if permission.Role == ViewerRole {
				if c.Request().Method == "GET" {
					return next(c)
				}

				return apis.NewForbiddenError("", nil)
			}

			return apis.NewNotFoundError("", nil)
		}
	}
}

func stationProxy(app *pocketbase.PocketBase) echo.HandlerFunc {
	const MountPoint = "/api/"

	return func(c echo.Context) error {
		// Get station
		stationId := c.PathParam("id")
		station, err := app.Dao().FindRecordById("stations", stationId)
		if err != nil {
			return apis.NewNotFoundError("", nil)
		}

		// Parse station url
		target, err := url.Parse(station.GetString("url"))
		if err != nil {
			return echo.ErrInternalServerError.WithInternal(err)
		}

		// Proxy request
		// TODO: fix panic from echo request when HTTP request is canceled -> http: superfluous response.WriteHeader call from github.com/labstack/echo/v5.(*Response).WriteHeader (response.go:64)
		// TODO: check if c.PathParam needs to be sanitized
		path := MountPoint + c.PathParam("*")
		proxy := &httputil.ReverseProxy{
			Rewrite: func(r *httputil.ProxyRequest) {
				r.SetURL(target)
				r.Out.Host = r.In.Host
				r.Out.URL.Path = path
				r.Out.Header.Del("Cookie")
				r.Out.Header.Del("Authorization")
			},
		}
		proxy.ServeHTTP(c.Response().Writer, c.Request())

		return nil
	}
}
