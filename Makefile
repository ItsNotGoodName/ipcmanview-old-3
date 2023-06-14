station:
	cargo r

server:
	cd ipcmanview-server && go run --tags dev . serve

ui:
	cd ipcmanview-ui && npm run dev

gen:
	cargo run --bin gen-openapi && cd ipcmanview-ui && npm run swag
