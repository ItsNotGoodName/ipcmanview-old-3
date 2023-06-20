import { Component, For, Match, Show, Switch } from "solid-js";
import {
  createColumnHelper,
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";
import { styled } from "@macaron-css/solid";
import { useNavigate } from "@solidjs/router";

import { Card, CardBody, CardHeader, CardHeaderTitle } from "~/ui/Card";
import { StationRecord } from "~/data/pb/records";
import { useStations } from "~/data/pb/hooks";
import { useCamerasTotal } from "~/data/station/hooks";
import { PbStationApi, usePb } from "~/data/pb";
import { theme } from "~/ui/theme";
import { IconAlert, IconSpinner } from "~/ui/Icon";
import { LayoutDefault } from "~/ui/Layouts";

const Table = styled("table", {
  base: {
    display: "table",
    width: "100%",
    borderCollapse: "collapse",
  },
});

const Th = styled("th", {
  base: {
    textAlign: "left",
    textTransform: "uppercase",
    padding: `${theme.space[1]} ${theme.space[2]}`,
  },
});

const Tr = styled("tr", {
  base: {
    cursor: "pointer",
    ":hover": {
      backgroundColor: theme.color.Surface2,
    },
  },
});

const Td = styled("td", {
  base: {
    padding: `${theme.space[1]} ${theme.space[2]}`,
  },
});

export const Stations: Component = () => {
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
        const api = new PbStationApi(pb, () => info.row.original.id);
        const total = useCamerasTotal(api);

        return (
          <div>
            <Switch>
              <Match when={total.isLoading}>
                <IconSpinner />
              </Match>
              <Match when={total.isError}>
                <IconAlert />
              </Match>
              <Match when={total.isSuccess}>{total.data!.total}</Match>
            </Switch>
          </div>
        );
      },
    }),
  ];
  const table = createSolidTable({
    get data() {
      return stations.data || [];
    },
    columns: defaultColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  const navigate = useNavigate();

  return (
    <LayoutDefault>
      <Card>
        <CardHeader>
          <CardHeaderTitle>Stations</CardHeaderTitle>
          <Show when={stations.isFetching}>
            <IconSpinner />
          </Show>
        </CardHeader>
        <CardBody padding={false}>
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
                  <Tr
                    role="button"
                    onClick={() => {
                      navigate(`/stations/${row.original.id}`);
                    }}
                  >
                    <For each={row.getVisibleCells()}>
                      {(cell) => (
                        <Td>
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext()
                          )}
                        </Td>
                      )}
                    </For>
                  </Tr>
                )}
              </For>
            </tbody>
          </Table>
        </CardBody>
      </Card>
    </LayoutDefault>
  );
};
