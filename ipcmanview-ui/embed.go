//go:build !dev

package ui

import (
	"embed"
	"fmt"
	"io/fs"
	"path/filepath"
)

//go:embed dist
var dist embed.FS

var FS = mustSubFS(dist, "dist")

// https://github.com/labstack/echo/blob/deb17d2388a74cd4133f46c2dedfb7601da1db0a/echo_fs.go#LL144C2-L144C2
func mustSubFS(currentFs fs.FS, fsRoot string) fs.FS {
	fsRoot = filepath.ToSlash(filepath.Clean(fsRoot)) // note: fs.FS operates only with slashes. `ToSlash` is necessary for Windows
	subFs, err := fs.Sub(currentFs, fsRoot)
	if err != nil {
		panic(fmt.Errorf("can not create sub FS, invalid root given, err: %w", err))
	}
	return subFs
}
