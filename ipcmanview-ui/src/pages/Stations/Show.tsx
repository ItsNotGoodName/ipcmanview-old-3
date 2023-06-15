import { CreateQueryResult } from "@tanstack/solid-query";
import {
  RiMediaImageFill,
  RiMediaVideoFill,
  RiSystemAlertFill,
  RiSystemRefreshFill,
} from "solid-icons/ri";
import {
  Component,
  createEffect,
  createMemo,
  createSignal,
  For,
  Match,
  on,
  ParentComponent,
  Show,
  Switch,
  untrack,
} from "solid-js";

import {
  useCameraDetail,
  useCameraLicenses,
  useCameras,
  useCameraSoftware,
  useCamerasTotal,
  useDeleteCamera,
  useInfiniteFiles,
  useFilesTotal,
  useFiles,
  useScansActive,
  useScansCompleted,
  useScansPending,
  useCameraShow,
  HookFileFilter,
  HookFileQuery,
} from "~/data/station/hooks";
import ErrorText from "~/ui/ErrorText";
import { createPaging, formatDateTime } from "~/data/utils";
import Button from "~/ui/Button";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import { useStationApi } from "~/data/station";

const FilesViewer: Component = () => {
  const [filter, setFilter] = createSignal<HookFileFilter>({});

  const limit = 20;
  const [query, setQuery] = createSignal<HookFileQuery>({ limit });
  const [estimate, setEstimate] = createSignal(0);
  createEffect(
    on(query, () => {
      if (query().before) {
        setEstimate(estimate() - limit);
      } else if (query().after) {
        setEstimate(estimate() + limit);
      } else {
        setEstimate(0);
      }
    })
  );

  const api = useStationApi();
  const filesTotal = useFilesTotal(api, filter);

  const files = useFiles(api, filter, query);
  const [selectedFileId, setSelectedFileId] = createSignal<number>();
  const selectedFile = createMemo(() => {
    if (files.data) {
      for (let file of files.data!.files) {
        if (file.id == selectedFileId()) {
          return file;
        }
      }
    }
  });
  const selectedUrl = () => {
    const file = selectedFile();
    if (file) {
      return api.fileUrl(file.camera_id, file.file_path);
    }
    return "";
  };
  createEffect(() => {
    if (files.data && files.data.files.length > 0) {
      for (let file of files.data.files) {
        if (file.id == selectedFileId()) {
          return;
        }
      }
      if (untrack(query).before)
        setSelectedFileId(files.data.files[files.data.files.length - 1].id);
      else setSelectedFileId(files.data.files[0].id);
    }
  });
  const [imageLoading, setImageLoading] = createSignal(false);
  let image: HTMLImageElement;
  createEffect(() => {
    if (selectedUrl() && !image.complete) {
      setImageLoading(true);
    }
  });

  const cameras = useCameras(api);

  return (
    <div class="flex flex-col gap-4 2xl:flex-row">
      <div class="flex-1">
        <Card>
          <div class="flex h-96 justify-center lg:h-[calc(80vh)]">
            <img
              ref={image!}
              onLoad={() => {
                setImageLoading(false);
              }}
              class="h-full object-scale-down"
              classList={{ blur: imageLoading() }}
              src={selectedUrl()}
            />
          </div>
        </Card>
      </div>
      <div class="flex flex-col gap-4 md:flex-row">
        <div class="flex flex-1 flex-col gap-4 2xl:w-64">
          <div class="join flex shadow">
            <div class="join-item tooltip tooltip-bottom" data-tip="First">
              <Button
                disabled={!(query().after || query().before)}
                loading={files.isLoading}
                onClick={() => {
                  setQuery({
                    ...query(),
                    before: undefined,
                    after: undefined,
                  });
                }}
              >
                ««
              </Button>
            </div>
            <div class="join-item tooltip tooltip-bottom" data-tip="Previous">
              <Button
                disabled={!files.data?.has_before}
                loading={files.isLoading}
                onClick={() => {
                  setQuery({
                    ...query(),
                    before: files.data?.before,
                    after: undefined,
                  });
                }}
              >
                «
              </Button>
            </div>
            <div
              class="join-item tooltip tooltip-bottom flex-1"
              data-tip="Total Files"
            >
              <Button class="w-full">
                <Switch>
                  <Match when={filesTotal.isLoading}>
                    <div class="loading" />
                  </Match>
                  <Match when={filesTotal.isError}>
                    <RiSystemAlertFill class="h-6 w-6 fill-error" />
                  </Match>
                  <Match when={filesTotal.data}>{filesTotal.data?.total}</Match>
                </Switch>
              </Button>
            </div>
            <div class="join-item tooltip tooltip-bottom" data-tip="Next">
              <Button
                disabled={!files.data?.has_after}
                loading={files.isLoading}
                onClick={() => {
                  setQuery({
                    ...query(),
                    after: files.data?.after,
                    before: undefined,
                  });
                }}
              >
                »
              </Button>
            </div>
          </div>
          <Card>
            <CardHeader
              right={<div class="inline-flex items-center">{estimate()}</div>}
            >
              Files
            </CardHeader>
            <ul class="menu menu-sm overflow-x-auto bg-base-200">
              <Show when={files.data}>
                <For each={files.data!.files}>
                  {(file) => (
                    <li>
                      <div
                        class="flex justify-between"
                        classList={{ active: file.id == selectedFileId() }}
                        onClick={[setSelectedFileId, file.id]}
                      >
                        <div>{formatDateTime(file.start_time)}</div>
                        <div class="flex items-center">
                          <Switch>
                            <Match when={file.kind == "jpg"}>
                              <RiMediaImageFill class="fill-success" />
                            </Match>
                            <Match when={file.kind == "dav"}>
                              <RiMediaVideoFill class="fill-error" />
                            </Match>
                          </Switch>
                        </div>
                      </div>
                    </li>
                  )}
                </For>
              </Show>
            </ul>
          </Card>
        </div>
        <div class="flex flex-1 flex-col gap-4 2xl:w-64">
          <Card>
            <CardHeader>Files Filter</CardHeader>
            <CardBody>
              <div class="flex flex-col gap-2">
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">Start Time</span>
                  </label>
                  <input
                    class="input-bordered input input-sm"
                    onChange={(e) => {
                      let date: Date = new Date(e.currentTarget.value);
                      if (isNaN(date.getTime())) {
                        return;
                      }
                      setFilter({
                        ...filter(),
                        start: date,
                      });
                    }}
                    type="datetime-local"
                  />
                </div>
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">End Time</span>
                  </label>
                  <input
                    class="input-bordered input input-sm"
                    onChange={(e) => {
                      let date: Date = new Date(e.currentTarget.value);
                      if (isNaN(date.getTime())) {
                        return;
                      }
                      setFilter({
                        ...filter(),
                        end: date,
                      });
                    }}
                    type="datetime-local"
                  />
                </div>
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">Kind</span>
                  </label>
                  <select
                    multiple
                    class="select-bordered select select-sm"
                    onChange={(e) => {
                      const kinds = [];
                      for (const i of e.target.selectedOptions) {
                        kinds.push(i.value);
                      }
                      setFilter({ ...filter(), kinds: kinds });
                    }}
                  >
                    <option value="jpg">jpg</option>
                    <option value="dav">dav</option>
                  </select>
                </div>
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">Cameras</span>
                  </label>
                  <select
                    multiple
                    class="select-bordered select select-sm"
                    onChange={(e) => {
                      const ids = [];
                      for (const i of e.target.selectedOptions) {
                        ids.push(parseInt(i.value));
                      }
                      setFilter({ ...filter(), camera_ids: ids });
                    }}
                  >
                    <Show when={cameras.data}>
                      <For each={cameras.data!}>
                        {(camera) => (
                          <option value={camera.id}>{camera.ip}</option>
                        )}
                      </For>
                    </Show>
                  </select>
                </div>
              </div>
            </CardBody>
          </Card>
        </div>
      </div>
    </div>
  );
};

const StationShow: Component = () => {
  const api = useStationApi();

  const cameras = useCameras(api);
  const camerasTotal = useCamerasTotal(api);
  const scansPending = useScansPending(api);
  const scansActive = useScansActive(api);

  const filesFilter = () => {
    return {} as HookFileFilter;
  };
  const files = useInfiniteFiles(api, filesFilter, () => {
    return { limit: 10 };
  });
  const filesTotal = useFilesTotal(api, filesFilter);

  const [scansCompletedPage, setScansCompletedPage] = createSignal(1);
  const [scansCompletedPerPage, setScansCompletedPerPage] = createSignal(5);
  const scansCompleted = useScansCompleted(
    api,
    scansCompletedPage,
    scansCompletedPerPage
  );
  const scansCompletedPaging = createPaging(scansCompleted);

  return (
    <div class="flex flex-col gap-2">
      <FilesViewer />
      <div class="divider" />
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
              onChange={(e) =>
                setScansCompletedPerPage(parseInt(e.target.value))
              }
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
            const cameraShow = useCameraShow(api, cameraId);
            const cameraDetail = useCameraDetail(api, cameraId);
            const cameraSoftware = useCameraSoftware(api, cameraId);
            const cameraLicenses = useCameraLicenses(api, cameraId);

            const deleteCamera = useDeleteCamera(api);

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
                    <Show when={deleteCamera.error}>
                      {(e) => <ErrorText>{e().message}</ErrorText>}
                    </Show>
                  </div>
                </JsonCard>
                <JsonCard
                  title={"Show Camera " + camera.id}
                  query={cameraShow}
                />
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
