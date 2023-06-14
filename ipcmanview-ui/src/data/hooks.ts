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
  ShowCamera,
  CreateCamera,
  UpdateCamera,
  PageResultScanCompleted,
  Total,
  TotalFileFilterQuery,
  QueryCameraFileResult,
  FileFilterQuery,
} from "./models";
import { StationRecord } from "./records";
import { searchParamsFromObject, stationUrl } from "./utils";

const q = {
  stations: ["stations"],
  cameras: (stationId: string) => [stationId, "cameras"],
  camerasTotal: (stationId: string) => [stationId, "camerasTotal"],
  showCameras: (stationId: string, cameraId: number) => [
    stationId,
    "cameras",
    cameraId,
    "showCameras",
  ],
  cameraDetail: (stationId: string, cameraId: number) => [
    stationId,
    "cameras",
    cameraId,
    "cameraDetail",
  ],
  cameraSoftware: (stationId: string, cameraId: number) => [
    stationId,
    "cameras",
    cameraId,
    "cameraSoftware",
  ],
  cameraLicenses: (stationId: string, cameraId: number) => [
    stationId,
    "cameras",
    cameraId,
    "cameraLicenses",
  ],
  scansPending: (stationId: string) => [stationId, "scansPending"],
  scansActive: (stationId: string) => [stationId, "scansActive"],
  scansCompleted: (stationId: string) => [stationId, "scansCompleted"],
  files: (stationId: string) => [stationId, "files"],
  filesTotal: (stationId: string) => [stationId, "filesTotal"],
};

const p = {
  camera: (stationId: string, cameraId: number) => (query: Query) =>
    query.queryKey[0] == stationId &&
    query.queryKey[1] == "cameras" &&
    query.queryKey[2] == cameraId,
  files: (stationId: string) => (query: Query) =>
    query.queryKey[0] == stationId &&
    (query.queryKey[1] == "files" || query.queryKey[1] == "filesTotal"),
};

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, ClientResponseError> =>
  createQuery(
    () => q.stations,
    () => pb.collection("stations").getFullList()
  );

export const useCameras = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, ClientResponseError> =>
  createQuery(
    () => q.cameras(stationId()),
    () => pb.send(stationUrl(stationId()) + "/cameras", {})
  );

export const useCamerasTotal = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Total, ClientResponseError> =>
  createQuery(
    () => q.camerasTotal(stationId()),
    () => pb.send(stationUrl(stationId()) + "/cameras-total", {})
  );

export const useCreateCamera = (
  pb: PocketBase,
  stationId: Accessor<string>
) => {
  const queryClient = useQueryClient();
  return createMutation({
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: q.cameras(stationId()) });
      queryClient.invalidateQueries({ queryKey: q.camerasTotal(stationId()) });
    },
    mutationFn: (data: CreateCamera) =>
      pb.send(stationUrl(stationId()) + "/cameras", {
        method: "POST",
        body: JSON.stringify(data),
      }),
  });
};

export const useUpdateCamera = (
  pb: PocketBase,
  stationId: Accessor<string>
) => {
  const queryClient = useQueryClient();
  return createMutation({
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: q.cameras(stationId()) });
      queryClient.invalidateQueries({ queryKey: q.camerasTotal(stationId()) });
      queryClient.invalidateQueries({
        predicate: p.camera(stationId(), variables.id),
      });
    },
    mutationFn: (data: UpdateCamera) =>
      pb.send(stationUrl(stationId()) + "/cameras" + data.id, {
        method: "POST",
        body: JSON.stringify(data),
      }),
  });
};

export const useDeleteCamera = (
  pb: PocketBase,
  stationId: Accessor<string>
) => {
  const queryClient = useQueryClient();
  return createMutation<unknown, ClientResponseError, number>({
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: q.cameras(stationId()) });
      queryClient.invalidateQueries({ queryKey: q.camerasTotal(stationId()) });
      queryClient.invalidateQueries({
        predicate: p.camera(stationId(), id),
      });
      queryClient.invalidateQueries({
        predicate: p.files(stationId()),
      });
    },
    mutationFn: (cameraId: number) =>
      pb.send(stationUrl(stationId()) + "/cameras/" + cameraId, {
        method: "DELETE",
      }),
  });
};

export const useShowCamera = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<ShowCamera, ClientResponseError> =>
  createQuery(
    () => q.showCameras(stationId(), cameraId()),
    () => pb.send(stationUrl(stationId()) + "/cameras/" + cameraId(), {})
  );

export const useCameraDetail = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<CameraDetail, ClientResponseError> =>
  createQuery(
    () => q.cameraDetail(stationId(), cameraId()),
    () =>
      pb.send(
        stationUrl(stationId()) + "/cameras/" + cameraId() + "/detail",
        {}
      )
  );

export const useCameraSoftware = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<CameraSoftware, ClientResponseError> =>
  createQuery(
    () => q.cameraSoftware(stationId(), cameraId()),
    () =>
      pb.send(
        stationUrl(stationId()) + "/cameras/" + cameraId() + "/software",
        {}
      )
  );

export const useCameraLicenses = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<Array<CameraLicense>, ClientResponseError> =>
  createQuery(
    () => q.cameraLicenses(stationId(), cameraId()),
    () =>
      pb.send(
        stationUrl(stationId()) + "/cameras/" + cameraId() + "/licenses",
        {}
      )
  );

export const useScansPending = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanPending>, ClientResponseError> =>
  createQuery(
    () => q.scansPending(stationId()),
    () => pb.send(stationUrl(stationId()) + "/scans/pending", {})
  );

export const useScansActive = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanActive>, ClientResponseError> =>
  createQuery(
    () => q.scansActive(stationId()),
    () => pb.send(stationUrl(stationId()) + "/scans/active", {})
  );

export const useScansCompleted = (
  pb: PocketBase,
  stationId: Accessor<string>,
  page: Accessor<number>,
  perPage: Accessor<number>
): CreateQueryResult<PageResultScanCompleted> =>
  createQuery(
    () => [...q.scansCompleted(stationId()), page(), perPage()],
    () =>
      pb.send(
        stationUrl(stationId()) +
          "/scans/completed?page=" +
          page() +
          "&per_page=" +
          perPage(),
        {}
      ),
    { keepPreviousData: true }
  );

export type FilesFilter = Omit<FileFilterQuery, "limit" | "before" | "after">;

export type FilesQuery = {
  limit?: number;
  before?: string;
  after?: string;
};

export type InfiniteFilesQuery = {
  limit?: number;
};

export const useFiles = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<FilesFilter>,
  query: Accessor<FilesQuery>
) =>
  createQuery<QueryCameraFileResult, ClientResponseError>(
    () => [...q.files(stationId()), filter(), query()],
    () => {
      return pb.send(
        stationUrl(
          stationId() +
            "/files?" +
            searchParamsFromObject({ ...filter(), ...query() })
        ),
        {}
      );
    },
    { keepPreviousData: true }
  );

// TODO: do not cache previous pages and also implement going backwards
export const useInfiniteFiles = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<FilesFilter>,
  query: Accessor<InfiniteFilesQuery>
) =>
  createInfiniteQuery<QueryCameraFileResult, ClientResponseError>({
    queryKey: () => [...q.files(stationId()), filter(), query(), "infinite"],
    queryFn: ({ pageParam }) => {
      let p = searchParamsFromObject({ ...filter(), ...query() });
      if (pageParam) {
        if (pageParam.isAfter) {
          p.set("after", pageParam.cursor);
        } else {
          p.set("before", pageParam.cursor);
        }
      }

      return pb.send(stationUrl(stationId() + "/files?" + p), {});
    },
    staleTime: Infinity,
    getNextPageParam: (last) =>
      last.has_after ? { isAfter: true, cursor: last.after } : undefined,
    getPreviousPageParam: (first) =>
      first.has_before ? { isAfter: false, cursor: first.before } : undefined,
  });

export const useFilesTotal = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<TotalFileFilterQuery>
) =>
  createQuery<{ total: number }, ClientResponseError>(
    () => [...q.filesTotal(stationId()), filter()],
    () =>
      pb.send(
        stationUrl(stationId()) +
          "/files-total?" +
          searchParamsFromObject(filter()),
        {}
      )
  );
