import PocketBase, { ClientResponseError } from "pocketbase";
import { Accessor, createMemo } from "solid-js";
import {
  createInfiniteQuery,
  createMutation,
  useQueryClient,
  createQuery,
  CreateQueryResult,
} from "@tanstack/solid-query";

import {
  Camera,
  CameraDetail,
  CameraLicense,
  CameraSoftware,
  CamerasTotal,
  CreateCameraMutation,
  FileFilter,
  FileResult,
  PageResult,
  ScanActive,
  ScanCompleted,
  ScanPending,
  ShowCamera,
} from "./models";
import { PbError, StationRecord } from "./records";
import { paramsFromObject, STATIONS_URI } from "./utils";

export type BackAndNext = { has_previous: boolean; has_next: boolean };

export function withBackAndNext<T, U = unknown>(
  query: CreateQueryResult<PageResult<T>, U>
): [CreateQueryResult<PageResult<T>, U>, Accessor<BackAndNext>] {
  return [
    query,
    createMemo(() => {
      let has_previous = false;
      let has_next = false;
      if (query.data && !query.isPreviousData) {
        if (query.data.page > 1) {
          has_previous = true;
        }

        if (query.data.page < query.data.total_pages) {
          has_next = true;
        }
      }
      return { has_previous, has_next };
    }),
  ];
}

function stationUrl(stationId: string): string {
  return STATIONS_URI + "/" + stationId;
}

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, PbError> =>
  createQuery(
    () => ["stations"],
    () => pb.collection("stations").getFullList()
  );

export const useCameras = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, PbError> =>
  createQuery(
    () => [stationId(), "/cameras"],
    () => pb.send(stationUrl(stationId()) + "/cameras", {})
  );

export const useCamerasTotal = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, PbError> =>
  createQuery(
    () => [stationId(), "/cameras-total"],
    () => pb.send(stationUrl(stationId()) + "/cameras-total", {})
  );

export const useCreateCamera = (
  pb: PocketBase,
  stationId: Accessor<string>
) => {
  const queryClient = useQueryClient();
  return createMutation({
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [stationId(), "/cameras"] });
      queryClient.invalidateQueries({
        queryKey: [stationId(), "/cameras-total"],
      });
    },
    mutationFn: (data: CreateCameraMutation) =>
      pb.send(stationUrl(stationId()) + "/cameras", {
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
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [stationId(), "/cameras"] });
      queryClient.invalidateQueries({
        queryKey: [stationId(), "/cameras-total"],
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
): CreateQueryResult<ShowCamera, PbError> =>
  createQuery(
    () => [stationId(), "/cameras/", cameraId()],
    () => pb.send(stationUrl(stationId()) + "/cameras/" + cameraId(), {})
  );

export const useCameraDetail = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<CameraDetail, PbError> =>
  createQuery(
    () => [stationId(), "/cameras/", cameraId(), "/detail"],
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
): CreateQueryResult<CameraSoftware, PbError> =>
  createQuery(
    () => [stationId(), "/cameras/", cameraId(), "/software"],
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
): CreateQueryResult<Array<CameraLicense>, PbError> =>
  createQuery(
    () => [stationId(), "/cameras/", cameraId(), "/licenses"],
    () =>
      pb.send(
        stationUrl(stationId()) + "/cameras/" + cameraId() + "/licenses",
        {}
      )
  );

export const useScansPending = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanPending>, PbError> =>
  createQuery(
    () => [stationId(), "/scans/pending"],
    () => pb.send(stationUrl(stationId()) + "/scans/pending", {})
  );

export const useScansActive = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanActive>, PbError> =>
  createQuery(
    () => [stationId(), "/scans/active"],
    () => pb.send(stationUrl(stationId()) + "/scans/active", {})
  );

export const useScansCompleted = (
  pb: PocketBase,
  stationId: Accessor<string>,
  page: Accessor<number>,
  perPage: Accessor<number>
): CreateQueryResult<PageResult<ScanCompleted>> =>
  createQuery(
    () => [stationId(), "/scans/completed", page(), perPage()],
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

// TODO: do not cache previous pages and also implement going backwards
export const useFiles = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<FileFilter>
) => {
  const params = () => paramsFromObject(filter());
  return createInfiniteQuery<FileResult, PbError>({
    queryKey: () => [stationId(), "/files", params().toString()],
    queryFn: ({ pageParam }) => {
      let p = params();
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
};

export const useFilesTotal = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<FileFilter>
) => {
  const params = () => paramsFromObject(filter());
  return createQuery(
    () => [stationId(), "/files-total", params.toString()],
    () => pb.send(stationUrl(stationId()) + "/files-total?" + params(), {})
  );
};
