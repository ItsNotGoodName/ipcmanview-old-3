import PocketBase, { ClientResponseError } from "pocketbase";
import { Accessor } from "solid-js";
import {
  createInfiniteQuery,
  createMutation,
  useQueryClient,
  createQuery,
  CreateQueryResult,
  Query,
} from "@tanstack/solid-query";

import {
  Camera,
  CameraDetail,
  CameraLicense,
  CameraSoftware,
  ScanActive,
  ScanPending,
  CameraShow,
  CreateCameraRequest,
  TotalQueryResult,
  UpdateCameraRequest,
  ScanCompletedPageResult,
  CameraFileQueryResult,
  CameraFileQuery,
} from "./models";
import { StationRecord } from "./records";
import { searchParamsFromObject } from "./utils";
import { StationApi } from "./api";

const q = {
  stations: ["stations"],
  cameras: ["cameras"],
  camerasTotal: ["camerasTotal"],
  showCameras: (cameraId: number) => ["cameras", cameraId, "showCameras"],
  cameraDetail: (cameraId: number) => ["cameras", cameraId, "cameraDetail"],
  cameraSoftware: (cameraId: number) => ["cameras", cameraId, "cameraSoftware"],
  cameraLicenses: (cameraId: number) => ["cameras", cameraId, "cameraLicenses"],
  scansPending: ["scansPending"],
  scansActive: ["scansActive"],
  scansCompleted: ["scansCompleted"],
  files: ["files"],
  filesTotal: ["filesTotal"],
};

const p = {
  camera: (api: StationApi, cameraId: number) => (query: Query) => {
    const key = api.getKey(query.queryKey);
    return key !== null && key[0] == "cameras" && key[1] == cameraId;
  },
  files: (api: StationApi) => (query: Query) => {
    const key = api.getKey(query.queryKey);
    return key !== null && (key[0] == "files" || key[0] == "filesTotal");
  },
};

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, ClientResponseError> =>
  createQuery(
    () => q.stations,
    () => pb.collection("stations").getFullList()
  );

export const useCameras = (
  api: StationApi
): CreateQueryResult<Array<Camera>, ClientResponseError> =>
  createQuery(
    () => api.key(q.cameras),
    () => api.send("/cameras")
  );

export const useCamerasTotal = (
  api: StationApi
): CreateQueryResult<TotalQueryResult, ClientResponseError> =>
  createQuery(
    () => api.key(q.camerasTotal),
    () => api.send("/cameras-total")
  );

export const useCreateCamera = (api: StationApi) => {
  const queryClient = useQueryClient();
  return createMutation({
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: api.key(q.cameras) });
      queryClient.invalidateQueries({ queryKey: api.key(q.camerasTotal) });
    },
    mutationFn: (data: CreateCameraRequest) =>
      api.send("/cameras", {
        method: "POST",
        body: JSON.stringify(data),
      }),
  });
};

export const useUpdateCamera = (api: StationApi) => {
  const queryClient = useQueryClient();
  return createMutation({
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: api.key(q.cameras) });
      queryClient.invalidateQueries({ queryKey: api.key(q.camerasTotal) });
      queryClient.invalidateQueries({ predicate: p.camera(api, variables.id) });
    },
    mutationFn: (data: UpdateCameraRequest) =>
      api.send("/cameras" + data.id, {
        method: "POST",
        body: JSON.stringify(data),
      }),
  });
};

export const useDeleteCamera = (api: StationApi) => {
  const queryClient = useQueryClient();
  return createMutation<unknown, ClientResponseError, number>({
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: api.key(q.cameras) });
      queryClient.invalidateQueries({ queryKey: api.key(q.camerasTotal) });
      queryClient.invalidateQueries({ predicate: p.camera(api, id) });
      queryClient.invalidateQueries({ predicate: p.files(api) });
    },
    mutationFn: (cameraId: number) =>
      api.send("/cameras/" + cameraId, {
        method: "DELETE",
      }),
  });
};

export const useShowCamera = (
  api: StationApi,
  cameraId: Accessor<number>
): CreateQueryResult<CameraShow, ClientResponseError> =>
  createQuery(
    () => api.key(q.showCameras(cameraId())),
    () => api.send("/cameras/" + cameraId())
  );

export const useCameraDetail = (
  api: StationApi,
  cameraId: Accessor<number>
): CreateQueryResult<CameraDetail, ClientResponseError> =>
  createQuery(
    () => api.key(q.cameraDetail(cameraId())),
    () => api.send("/cameras/" + cameraId() + "/detail")
  );

export const useCameraSoftware = (
  api: StationApi,
  cameraId: Accessor<number>
): CreateQueryResult<CameraSoftware, ClientResponseError> =>
  createQuery(
    () => api.key(q.cameraSoftware(cameraId())),
    () => api.send("/cameras/" + cameraId() + "/software")
  );

export const useCameraLicenses = (
  api: StationApi,
  cameraId: Accessor<number>
): CreateQueryResult<Array<CameraLicense>, ClientResponseError> =>
  createQuery(
    () => api.key(q.cameraLicenses(cameraId())),
    () => api.send("/cameras/" + cameraId() + "/licenses")
  );

export const useScansPending = (
  api: StationApi
): CreateQueryResult<Array<ScanPending>, ClientResponseError> =>
  createQuery(
    () => api.key(q.scansPending),
    () => api.send("/scans/pending")
  );

export const useScansActive = (
  api: StationApi
): CreateQueryResult<Array<ScanActive>, ClientResponseError> =>
  createQuery(
    () => api.key(q.scansActive),
    () => api.send("/scans/active")
  );

export const useScansCompleted = (
  api: StationApi,
  page: Accessor<number>,
  perPage: Accessor<number>
): CreateQueryResult<ScanCompletedPageResult> =>
  createQuery(
    () => api.key([...q.scansCompleted, page(), perPage()]),
    () =>
      api.send("/scans/completed?page=" + page() + "&per_page=" + perPage()),
    { keepPreviousData: true }
  );

export type HookFileFilter = Omit<
  CameraFileQuery,
  "limit" | "before" | "after"
>;

export type HookFileQuery = {
  limit?: number;
  before?: string;
  after?: string;
};

export type HookInfiniteFilesQuery = {
  limit?: number;
};

export const useFiles = (
  api: StationApi,
  filter: Accessor<HookFileFilter>,
  query: Accessor<HookFileQuery>
) =>
  createQuery<CameraFileQueryResult, ClientResponseError>(
    () => api.key([...q.files, filter(), query()]),
    () =>
      api.send("/files?" + searchParamsFromObject({ ...filter(), ...query() })),
    { keepPreviousData: true }
  );

// TODO: do not cache previous pages and also implement going backwards
export const useInfiniteFiles = (
  api: StationApi,
  filter: Accessor<HookFileFilter>,
  query: Accessor<HookInfiniteFilesQuery>
) =>
  createInfiniteQuery<CameraFileQueryResult, ClientResponseError>({
    queryKey: () => api.key([...q.files, filter(), query(), "infinite"]),
    queryFn: ({ pageParam }) => {
      let p = searchParamsFromObject({ ...filter(), ...query() });
      if (pageParam) {
        if (pageParam.isAfter) {
          p.set("after", pageParam.cursor);
        } else {
          p.set("before", pageParam.cursor);
        }
      }

      return api.send("/files?" + p);
    },
    staleTime: Infinity,
    getNextPageParam: (last) =>
      last.has_after ? { isAfter: true, cursor: last.after } : undefined,
    getPreviousPageParam: (first) =>
      first.has_before ? { isAfter: false, cursor: first.before } : undefined,
  });

export const useFilesTotal = (
  api: StationApi,
  filter: Accessor<HookFileFilter>
) =>
  createQuery<TotalQueryResult, ClientResponseError>(
    () => api.key([...q.filesTotal, filter()]),
    () => api.send("/files-total?" + searchParamsFromObject(filter()))
  );
