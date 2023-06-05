import { A } from "@solidjs/router";
import { Component, For, Match, Show, Switch } from "solid-js";
import { RiSystemAlertFill } from "solid-icons/ri";
import {
  createColumnHelper,
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";

import Spinner from "~/ui/Spinner";
import { Card, CardHeader } from "~/ui/Card";
import { StationRecord } from "~/data/records";
import { useCamerasTotal, useStations } from "~/data/hooks";
import { usePb } from "~/data/pb";

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
              <RiSystemAlertFill class="h-full w-6 fill-error" />
            </Match>
            <Match when={total.isSuccess}>{total.data!.total}</Match>
          </Switch>
        );
      },
    }),
    columnHelper.display({
      id: "action",
      cell: (info) => (
        <A
          class="no-animation btn-xs btn"
          href={"/stations/" + info.row.original.id}
        >
          details
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
        right={
          <Show when={stations.isFetching}>
            <Spinner />
          </Show>
        }
      >
        Stations
      </CardHeader>
      <div class="overflow-x-auto pb-2">
        <table class="table w-full">
          <thead>
            <For each={table.getHeaderGroups()}>
              {(headerGroup) => (
                <tr>
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
                <tr>
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
