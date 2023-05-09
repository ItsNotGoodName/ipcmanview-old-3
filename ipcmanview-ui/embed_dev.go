//go:build dev

package ui

import (
	"io/fs"
)

type empty struct{}

func (empty) Open(string) (fs.File, error) {
	return nil, fs.ErrNotExist
}

var FS = empty{}
