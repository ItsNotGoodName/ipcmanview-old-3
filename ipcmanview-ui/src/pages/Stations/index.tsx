import { Component, For, Match, Show, Switch } from "solid-js";
import { A } from "@solidjs/router";
import {
  createColumnHelper,
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";
import { RiSystemAlertFill } from "solid-icons/ri";

import { useCamerasTotal, useStations } from "~/data/hooks";
import { usePb } from "~/data/pb";
import { Card, CardHeader } from "~/ui/Card";
import Spinner from "~/ui/Spinner";
import { StationRecord } from "~/data/records";

const StationList: Component = () => {
  const pb = usePb();
  const stations = useStations(pb);

  const columnHelper = createColumnHelper<StationRecord>();
  const defaultColumns = [
    columnHelper.accessor("name", {}),
    columnHelper.accessor("url", {}),
    columnHelper.display({
      id: "cameras",
      header: "cameras",
      cell: (info) => {
        const total = useCamerasTotal(pb, () => info.row.original.id);
        return (
          <Switch>
            <Match when={total.isLoading}>
              <Spinner />
            </Match>
            <Match when={total.isError}>
              <RiSystemAlertFill class="h-full w-6 fill-danger-100" />
            </Match>
            <Match when={total.isSuccess}>{total.data!.total}</Match>
          </Switch>
        );
      },
    }),

    columnHelper.display({
      id: "action",
      cell: (info) => (
        <A class="text-link" href={"/stations/" + info.row.original.id}>
          Open
        </A>
      ),
    }),
  ];
  const table = createSolidTable({
    get data() {
      return stations.data || [];
    },
    columns: defaultColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Card>
      <CardHeader
        title="Stations"
        right={
          <Show when={stations.isFetching}>
            <Spinner />
          </Show>
        }
      />
      <div class="overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <For each={table.getHeaderGroups()}>
              {(headerGroup) => (
                <tr class="bg-ship-600 text-ship-50">
                  <For each={headerGroup.headers}>
                    {(header) => (
                      <th class="p-2 text-left uppercase">
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
                <tr class="cursor-pointer even:bg-ship-50">
                  <For each={row.getVisibleCells()}>
                    {(cell) => (
                      <td class="p-1">
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
    </Card>
  );
};

export default StationList;
