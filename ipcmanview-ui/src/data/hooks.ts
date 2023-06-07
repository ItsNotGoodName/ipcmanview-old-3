import PocketBase, { ClientResponseError } from "pocketbase";
import { Accessor } from "solid-js";
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
  FilesQuery,
  FilesResult,
  FilesFilter,
  InfiniteFilesQuery,
  PageResult,
  ScanActive,
  ScanCompleted,
  ScanPending,
  ShowCamera,
} from "./models";
import { StationRecord } from "./records";
import { searchParamsFromObject, stationUrl } from "./utils";

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, ClientResponseError> =>
  createQuery(
    () => ["stations"],
    () => pb.collection("stations").getFullList()
  );

export const useCameras = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, ClientResponseError> =>
  createQuery(
    () => [stationId(), "/cameras"],
    () => pb.send(stationUrl(stationId()) + "/cameras", {})
  );

export const useCamerasTotal = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, ClientResponseError> =>
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
): CreateQueryResult<ShowCamera, ClientResponseError> =>
  createQuery(
    () => [stationId(), "/cameras/", cameraId()],
    () => pb.send(stationUrl(stationId()) + "/cameras/" + cameraId(), {})
  );

export const useCameraDetail = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<CameraDetail, ClientResponseError> =>
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
): CreateQueryResult<CameraSoftware, ClientResponseError> =>
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
): CreateQueryResult<Array<CameraLicense>, ClientResponseError> =>
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
): CreateQueryResult<Array<ScanPending>, ClientResponseError> =>
  createQuery(
    () => [stationId(), "/scans/pending"],
    () => pb.send(stationUrl(stationId()) + "/scans/pending", {})
  );

export const useScansActive = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanActive>, ClientResponseError> =>
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

export const useFiles = (
  pb: PocketBase,
  stationId: Accessor<string>,
  filter: Accessor<FilesFilter>,
  query: Accessor<FilesQuery>
) =>
  createQuery<FilesResult, ClientResponseError>(
    () => [stationId(), "/files", filter(), query()],
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
  createInfiniteQuery<FilesResult, ClientResponseError>({
    queryKey: () => [stationId(), "/files", filter(), query(), "infinite"],
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
  filter: Accessor<FilesFilter>
) =>
  createQuery<{ total: number }, ClientResponseError>(
    () => [stationId(), "/files-total", filter()],
    () =>
      pb.send(
        stationUrl(stationId()) +
          "/files-total?" +
          searchParamsFromObject(filter()),
        {}
      )
  );
