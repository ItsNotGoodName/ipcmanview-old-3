//go:build !dev

package ui

import (
	"embed"

	"github.com/labstack/echo/v5"
)

//go:embed dist
var dist embed.FS

var FS = echo.MustSubFS(dist, "dist")
