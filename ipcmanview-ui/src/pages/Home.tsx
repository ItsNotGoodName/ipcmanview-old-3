import {
  Component,
  For,
  Switch,
  Match,
  Show,
  createSignal,
  Index,
  createEffect,
} from "solid-js";
import { RiSystemAlertFill } from "solid-icons/ri";
import { useSearchParams } from "@solidjs/router";
import {
  createColumnHelper,
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";

import Spinner from "../components/Spinner";
import { StationRecord } from "../records";
import { useCameras, useCamerasTotal, useStations } from "../hooks";
import ActionButtons from "../components/ActionButtons";
import { Camera } from "../models";
import { usePb } from "../pb";
import { Card, CardBody, CardHeader } from "../components/Card";

const Home: Component = () => {
  const pb = usePb();
  const stations = useStations(pb);

  const [searchParams, setSearchParams] = useSearchParams<{
    station: string;
  }>();
  const [selectedStation, setSelectedStation] = createSignal(
    searchParams.station
  );
  createEffect(() => {
    setSearchParams({ station: selectedStation() });
  });

  return (
    <div class="flex flex-col gap-4 sm:flex-row">
      <div class="w-full sm:w-48">
        <Card>
          <CardHeader
            title="Stations"
            right={
              <Show when={stations.isFetching}>
                <Spinner />
              </Show>
            }
          />
          <div class="max-h-36 overflow-y-auto sm:max-h-96">
            <Switch>
              <Match when={stations.isLoading}>
                <CardBody>
                  <Spinner />
                </CardBody>
              </Match>
              <Match when={stations.isError}>
                <CardBody>
                  <div class="text-danger-100">{stations.error!.message}</div>
                </CardBody>
              </Match>
              <Match when={stations.data}>
                {(stations) => (
                  <StationsList
                    selected={selectedStation()}
                    onSelect={setSelectedStation}
                    stations={stations()}
                  />
                )}
              </Match>
            </Switch>
          </div>
        </Card>
      </div>
      <Show when={stations.data?.find((s) => s.id == selectedStation())}>
        {(s) => (
          <div class="flex-1">
            <StationCard station={s()} />
          </div>
        )}
      </Show>
    </div>
  );
};

type StationsListProps = {
  stations: StationRecord[];
  selected: string;
  onSelect: (id: string) => void;
};

const StationsList: Component<StationsListProps> = (props) => {
  return (
    <ul class="flex flex-col">
      <Index each={props.stations}>
        {(station) => {
          const pb = usePb();
          const total = useCamerasTotal(pb, () => station().id);

          return (
            <li
              class="flex cursor-pointer border-b border-ship-300 p-2 last:border-b-0 hover:bg-ship-200"
              classList={{
                "bg-ship-100 font-bold": station().id == props.selected,
              }}
              onClick={() => props.onSelect(station().id)}
            >
              <div class="flex-1 truncate">{station().name}</div>
              <div>
                <Switch>
                  <Match when={total.isLoading}>
                    <Spinner />
                  </Match>
                  <Match when={total.isError}>
                    <RiSystemAlertFill class="h-full w-6 fill-danger-100" />
                  </Match>
                  <Match when={total.isSuccess}>{total.data!.total}</Match>
                </Switch>
              </div>
            </li>
          );
        }}
      </Index>
    </ul>
  );
};

type StationCardProps = {
  station: StationRecord;
};

const StationCard: Component<StationCardProps> = (props) => {
  const pb = usePb();
  const cameras = useCameras(pb, () => props.station.id);

  const columnHelper = createColumnHelper<Camera>();

  const defaultColumns = [
    columnHelper.display({
      id: "actions",
      cell: (_) => {
        return (
          <div class="flex">
            <input
              class="m-auto h-6 w-6 rounded border-ship-300"
              type="checkbox"
            />
          </div>
        );
      },
      header: () => (
        <div class="flex">
          <input
            class="m-auto h-6 w-6 rounded border-ship-300"
            type="checkbox"
          />
        </div>
      ),
    }),
    columnHelper.accessor("ip", {}),
    columnHelper.accessor("username", {}),
  ];

  const table = createSolidTable({
    get data() {
      return cameras.data || [];
    },
    columns: defaultColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Card>
      <CardHeader
        title="Cameras"
        right={
          <Show when={cameras.isFetching}>
            <Spinner />
          </Show>
        }
      />
      <Switch>
        <Match when={cameras.isError}>
          <CardBody>
            <div class="text-danger-100">{cameras.error!.message}</div>
          </CardBody>
        </Match>
        <Match when={cameras.isSuccess}>
          <ActionButtons class="m-2" />
          <div class="overflow-x-auto">
            <table class="table w-full">
              <thead class="bg-ship-500 text-left uppercase text-ship-50">
                <For each={table.getHeaderGroups()}>
                  {(headerGroup) => (
                    <tr>
                      <For each={headerGroup.headers}>
                        {(header) => (
                          <th class="p-2">
                            {header.isPlaceholder
                              ? null
                              : flexRender(
                                  header.column.columnDef.header,
                                  header.getContext()
                                )}
                          </th>
                        )}
                      </For>
                    </tr>
                  )}
                </For>
              </thead>
              <tbody>
                <For each={table.getRowModel().rows}>
                  {(row) => (
                    <tr class="border-b border-ship-300 last:border-b-0">
                      <For each={row.getVisibleCells()}>
                        {(cell) => (
                          <td class="p-2">
                            {flexRender(
                              cell.column.columnDef.cell,
                              cell.getContext()
                            )}
                          </td>
                        )}
                      </For>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </div>
        </Match>
      </Switch>
    </Card>
  );
};

export default Home;
