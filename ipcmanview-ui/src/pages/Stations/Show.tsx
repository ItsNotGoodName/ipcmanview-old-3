import { useParams } from "@solidjs/router";
import { CreateQueryResult } from "@tanstack/solid-query";
import { RiSystemRefreshFill } from "solid-icons/ri";
import { Component, createSignal, For, ParentComponent, Show } from "solid-js";
import Button from "~/ui/Button";
import { Card, CardHeader } from "~/ui/Card";

import {
  useCameraDetail,
  useCameraLicenses,
  useCameras,
  useCameraSoftware,
  useCamerasTotal,
  useDeleteCamera,
  useFiles,
  useFilesTotal,
  useScansActive,
  useScansCompleted,
  useScansPending,
  useShowCamera,
  withBackAndNext,
} from "~/data/hooks";
import { FileFilter } from "~/data/models";
import { usePb } from "~/data/pb";

const StationShow: Component = () => {
  const pb = usePb();

  const { stationId: stationIdParams } = useParams<{ stationId: string }>();
  const stationId = () => stationIdParams;

  const cameras = useCameras(pb, stationId);
  const camerasTotal = useCamerasTotal(pb, stationId);
  const scansPending = useScansPending(pb, stationId);
  const scansActive = useScansActive(pb, stationId);

  const filesFilter = () => {
    return {} satisfies FileFilter;
  };
  const files = useFiles(pb, stationId, filesFilter);
  const filesTotal = useFilesTotal(pb, stationId, filesFilter);

  const [scansCompletedPage, setScansCompletedPage] = createSignal(1);
  const [scansCompletedPerPage, setScansCompletedPerPage] = createSignal(5);
  const [scansCompleted, scansCompletedPaging] = withBackAndNext(
    useScansCompleted(pb, stationId, scansCompletedPage, scansCompletedPerPage)
  );

  return (
    <div class="grid grid-cols-1 gap-2 lg:grid-cols-2 2xl:grid-cols-3">
      <JsonCard title="Cameras" query={cameras} />
      <JsonCard title="Cameras Total" query={camerasTotal} />

      <JsonCard title="Pending Scans" query={scansPending} />
      <JsonCard title="Active Scans" query={scansActive} />
      <JsonCard title="Completed Scans" query={scansCompleted}>
        <div class="flex gap-2 border-b border-ship-600 p-2">
          <input
            class="flex-1 rounded"
            type="number"
            onChange={(e) => setScansCompletedPerPage(parseInt(e.target.value))}
            value={scansCompletedPerPage()}
          />
          <div class="flex w-40 justify-between gap-2">
            <button
              class="rounded bg-ship-500 p-2 text-ship-50"
              classList={{
                "opacity-80": !scansCompletedPaging().has_previous,
                "hover:bg-ship-600": scansCompletedPaging().has_previous,
              }}
              onClick={() => setScansCompletedPage((prev) => prev - 1)}
              disabled={!scansCompletedPaging().has_previous}
            >
              Back {scansCompletedPaging().has_previous ? 1 : 0}
            </button>
            <div class="my-auto text-xl">{scansCompletedPage()}</div>
            <button
              class="rounded bg-ship-500 p-2 text-ship-50"
              classList={{
                "opacity-80": !scansCompletedPaging().has_next,
                "hover:bg-ship-600": scansCompletedPaging().has_next,
              }}
              onClick={() => setScansCompletedPage((prev) => prev + 1)}
              disabled={!scansCompletedPaging().has_next}
            >
              Next {scansCompletedPaging().has_next ? 1 : 0}
            </button>
          </div>
        </div>
      </JsonCard>
      <JsonCard title="Files Total" query={filesTotal} />
      <JsonCard title="Files" query={files}>
        <div class="flex justify-between gap-2 border-b border-ship-600 p-2">
          <button
            class="rounded bg-ship-500 p-2 text-ship-50"
            classList={{
              "opacity-80": !files.hasPreviousPage,
              "hover:bg-ship-600": files.hasPreviousPage,
            }}
            onClick={() => files.fetchPreviousPage()}
            disabled={!files.hasPreviousPage}
          >
            Back
          </button>
          <button
            class="rounded bg-ship-500 p-2 text-ship-50"
            classList={{
              "opacity-80": !files.hasNextPage,
              "hover:bg-ship-600": files.hasNextPage,
            }}
            onClick={() => files.fetchNextPage()}
            disabled={!files.hasNextPage}
          >
            Next
          </button>
        </div>
      </JsonCard>

      <For each={cameras.data || []}>
        {(camera) => {
          const cameraId = () => camera.id;
          const showCamera = useShowCamera(pb, stationId, cameraId);
          const cameraDetail = useCameraDetail(pb, stationId, cameraId);
          const cameraSoftware = useCameraSoftware(pb, stationId, cameraId);
          const cameraLicenses = useCameraLicenses(pb, stationId, cameraId);

          const deleteCamera = useDeleteCamera(pb, stationId);

          return (
            <>
              <JsonCard title={"Camera Actions " + camera.id}>
                <Show when={deleteCamera.isError}>
                  {deleteCamera.error!.message}
                </Show>
                <div class="p-2">
                  <Button
                    loading={deleteCamera.isLoading}
                    onClick={() => deleteCamera.mutate(cameraId())}
                  >
                    Delete Camera
                  </Button>
                </div>
              </JsonCard>
              <JsonCard title={"Show Camera " + camera.id} query={showCamera} />
              <JsonCard
                title={"Camera " + camera.id + " Detail"}
                query={cameraDetail}
              />
              <JsonCard
                title={"Camera " + camera.id + " Software"}
                query={cameraSoftware}
              />
              <JsonCard
                title={"Camera " + camera.id + " Licenses"}
                query={cameraLicenses}
              />
            </>
          );
        }}
      </For>
    </div>
  );
};

const JsonCard: ParentComponent<{
  title: string;
  query?: CreateQueryResult<any, any>;
}> = (props) => (
  <Card>
    <CardHeader
      title={props.title}
      right={
        <Show when={props.query}>
          <button
            class="rounded hover:bg-ship-700"
            onClick={() => props.query!.refetch()}
          >
            <RiSystemRefreshFill
              class="inline-flex h-full w-6"
              classList={{
                "animate-spin": props.query!.isRefetching,
              }}
            />
          </button>
        </Show>
      }
    />
    <div class="flex max-h-96 flex-col">
      <div>{props.children}</div>
      <Show when={props.query}>
        <div class="overflow-auto">
          <pre>{JSON.stringify(props.query!.data, null, 2)}</pre>
        </div>
      </Show>
    </div>
  </Card>
);

export default StationShow;
