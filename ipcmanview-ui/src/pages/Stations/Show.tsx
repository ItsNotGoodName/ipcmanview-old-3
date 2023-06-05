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
} from "~/data/hooks";
import { FileFilter } from "~/data/models";
import { usePb } from "~/data/pb";
import InputError from "~/ui/InputError";
import { createPaging } from "~/data/utils";

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
  const scansCompleted = useScansCompleted(
    pb,
    stationId,
    scansCompletedPage,
    scansCompletedPerPage
  );
  const scansCompletedPaging = createPaging(scansCompleted);

  return (
    <div class="grid grid-cols-1 gap-2 lg:grid-cols-2 2xl:grid-cols-3">
      <JsonCard title="Cameras" query={cameras} />
      <JsonCard title="Cameras Total" query={camerasTotal} />

      <JsonCard title="Pending Scans" query={scansPending} />
      <JsonCard title="Active Scans" query={scansActive} />
      <JsonCard title="Completed Scans" query={scansCompleted}>
        <div class="flex gap-2 p-2">
          <input
            class="input-bordered input flex-1"
            type="number"
            onChange={(e) => setScansCompletedPerPage(parseInt(e.target.value))}
            value={scansCompletedPerPage()}
          />
          <div class="join">
            <Button
              class="join-item"
              onClick={() => setScansCompletedPage((prev) => prev - 1)}
              disabled={!scansCompletedPaging().has_previous}
            >
              Back
            </Button>
            <Button class="join-item">{scansCompletedPage()}</Button>
            <Button
              class="join-item"
              onClick={() => setScansCompletedPage((prev) => prev + 1)}
              disabled={!scansCompletedPaging().has_next}
            >
              Next
            </Button>
          </div>
        </div>
      </JsonCard>
      <JsonCard title="Files Total" query={filesTotal} />
      <JsonCard title="Files" query={files}>
        <div class="join p-2">
          <Button
            class="join-item"
            onClick={() => files.fetchPreviousPage()}
            disabled={!files.hasPreviousPage}
          >
            Back
          </Button>
          <Button class="join-item">{files.data?.pages.length}</Button>
          <Button
            class="join-item"
            onClick={() => files.fetchNextPage()}
            disabled={!files.hasNextPage}
          >
            Next
          </Button>
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
                <div class="flex flex-col">
                  <Button
                    class="w-full"
                    loading={deleteCamera.isLoading}
                    onClick={() => deleteCamera.mutate(cameraId())}
                  >
                    Delete Camera
                  </Button>
                  <InputError error={deleteCamera.error?.message} />
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
      right={
        <Show when={props.query}>
          <button class="rounded" onClick={() => props.query!.refetch()}>
            <RiSystemRefreshFill
              class="inline-flex h-full w-6"
              classList={{
                "animate-spin": props.query!.isRefetching,
              }}
            />
          </button>
        </Show>
      }
    >
      {props.title}
    </CardHeader>
    <div class="flex max-h-96 flex-col">
      <div>{props.children}</div>
      <Show when={props.children && props.query}>
        <hr class="border-base-300" />
      </Show>
      <Show when={props.query}>
        <div class="overflow-auto">
          <pre>{JSON.stringify(props.query!.data, null, 2)}</pre>
        </div>
      </Show>
    </div>
  </Card>
);

export default StationShow;
