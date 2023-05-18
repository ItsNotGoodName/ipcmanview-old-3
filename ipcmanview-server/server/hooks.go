package server

import (
	"encoding/json"
	"net/url"

	validation "github.com/go-ozzo/ozzo-validation/v4"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/core"
	"github.com/pocketbase/pocketbase/models"
)

func parseCameras(r *models.Record) error {
	var cameras []int
	if err := json.Unmarshal([]byte(r.GetString("cameras")), &cameras); err != nil {
		errs := make(validation.Errors)
		errs["cameras"] = err
		return apis.NewBadRequestError("", errs)
	}
	if cameras == nil {
		cameras = make([]int, 0)
	}

	r.Set("cameras", cameras)

	return nil
}

func OnPermission(app *pocketbase.PocketBase) {
	app.OnRecordBeforeCreateRequest("permissions").Add(func(e *core.RecordCreateEvent) error {
		return parseCameras(e.Record)
	})

	app.OnRecordBeforeUpdateRequest("permissions").Add(func(e *core.RecordUpdateEvent) error {
		if e.Record.Get("cameras") == nil {
			return nil
		}

		return parseCameras(e.Record)
	})
}

func OnStation(app *pocketbase.PocketBase) {
	app.OnRecordBeforeCreateRequest("stations").Add(func(e *core.RecordCreateEvent) error {
		// Default name to hostname when empty
		if e.Record.GetString("name") == "" {
			urL, err := url.Parse(e.Record.GetString("url"))
			if err != nil {
				return err
			}

			e.Record.Set("name", urL.Hostname())
		}

		return nil
	})
}
