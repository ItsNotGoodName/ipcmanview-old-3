station:
	cargo r

server:
	cd ipcmanview-server && go run --tags dev . serve

ui:
	cd ipcmanview-ui && npm run dev
