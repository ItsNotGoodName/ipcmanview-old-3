import { A } from "@solidjs/router";
import { Component, For, Match, Show, Switch } from "solid-js";
import {
  createColumnHelper,
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";
import { styled } from "@macaron-css/solid";

import { Card, CardHeader } from "~/ui/Card";
import { StationRecord } from "~/data/records";
import { useCamerasTotal, useStations } from "~/data/hooks";
import { usePb } from "~/data/pb";
import { theme } from "~/ui/theme";
import { Row } from "~/ui/utility";
import { IconAlert, IconSpinner } from "~/ui/Icon";
import { StationApiPb } from "~/data/station";

const Overflow = styled("div", {
  base: {
    overflowX: "auto",
  },
});

const Table = styled("table", {
  base: {
    display: "table",
    width: "100%",
    padding: theme.space[2],
  },
});

const Th = styled("th", {
  base: {
    textAlign: "left",
    textTransform: "uppercase",
  },
});

const Stations: Component = () => {
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
        const api = new StationApiPb(pb, () => info.row.original.id);
        const total = useCamerasTotal(api);

        return (
          <Row>
            <Switch>
              <Match when={total.isLoading}>
                <IconSpinner />
              </Match>
              <Match when={total.isError}>
                <IconAlert />
              </Match>
              <Match when={total.isSuccess}>{total.data!.total}</Match>
            </Switch>
          </Row>
        );
      },
    }),
    columnHelper.display({
      id: "action",
      cell: (info) => <A href={"/stations/" + info.row.original.id}>details</A>,
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
      <CardHeader>
        Stations
        <Show when={stations.isFetching}>
          <IconSpinner />
        </Show>
      </CardHeader>
      <Overflow>
        <Table>
          <thead>
            <For each={table.getHeaderGroups()}>
              {(headerGroup) => (
                <tr>
                  <For each={headerGroup.headers}>
                    {(header) => (
                      <Th>
                        {header.isPlaceholder
                          ? null
                          : flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                      </Th>
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
                      <td>
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
        </Table>
      </Overflow>
    </Card>
  );
};

export default Stations;
