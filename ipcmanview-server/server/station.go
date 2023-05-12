package server

import (
	"net/http/httputil"
	"net/url"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/pocketbase"
)

func stationProxy(app *pocketbase.PocketBase) echo.HandlerFunc {
	return func(c echo.Context) error {
		// Get station
		id := c.PathParam("id")
		station, err := app.Dao().FindRecordById("stations", id)
		if err != nil {
			return echo.ErrNotFound
		}

		// Parse station url
		target, err := url.Parse(station.GetString("url"))
		if err != nil {
			return echo.ErrInternalServerError
		}

		// Proxy request
		// TODO: fix panic from echo request when HTTP request is canceled -> http: superfluous response.WriteHeader call from github.com/labstack/echo/v5.(*Response).WriteHeader (response.go:64)
		path := "/" + c.PathParam("*")
		proxy := &httputil.ReverseProxy{
			Rewrite: func(r *httputil.ProxyRequest) {
				r.SetURL(target)
				r.Out.Host = r.In.Host
				r.Out.URL.Path = path
				r.Out.Header.Del("Cookie")
			},
		}
		proxy.ServeHTTP(c.Response().Writer, c.Request())

		return nil
	}
}
