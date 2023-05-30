import PocketBase from "pocketbase";
import { Accessor, createMemo } from "solid-js";
import { createQuery, CreateQueryResult } from "@tanstack/solid-query";

import {
  Camera,
  CameraDetail,
  CameraLicense,
  CameraSoftware,
  CamerasTotal,
  PageResult,
  ScanActive,
  ScanCompleted,
  ScanPending,
  ShowCamera,
} from "./models";
import { PbError, StationRecord } from "./records";
import { STATIONS_URI } from "./utils";

export type BackAndNext = { has_back: boolean; has_next: boolean };

export function withBackAndNext<T>(
  query: CreateQueryResult<PageResult<T>, unknown>
): [CreateQueryResult<PageResult<T>, unknown>, Accessor<BackAndNext>] {
  return [
    query,
    createMemo(() => {
      let has_back = false;
      let has_next = false;
      if (query.data && !query.isPreviousData) {
        if (query.data.page > 1) {
          has_back = true;
        }

        if (query.data.page < query.data.total_pages) {
          has_next = true;
        }
      }
      return { has_back, has_next };
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

export const useShowCamera = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<ShowCamera, PbError> =>
  createQuery(
    () => [stationId(), "/cameras", cameraId()],
    () => pb.send(stationUrl(stationId()) + "/cameras/" + cameraId(), {})
  );

export const useCameraDetail = (
  pb: PocketBase,
  stationId: Accessor<string>,
  cameraId: Accessor<number>
): CreateQueryResult<CameraDetail, PbError> =>
  createQuery(
    () => [stationId(), "/cameras", cameraId(), "detail"],
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
    () => [stationId(), "/cameras", cameraId(), "software"],
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
    () => [stationId(), "/cameras", cameraId(), "licenses"],
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

export const useCamerasTotal = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, PbError> =>
  createQuery(
    () => [stationId(), "/cameras-total"],
    () => pb.send(stationUrl(stationId()) + "/cameras-total", {})
  );

export const useAuthRefresh = (
  pb: PocketBase,
  {
    refetchOnWindowFocus,
  }: {
    refetchOnWindowFocus: boolean;
  }
) =>
  createQuery(
    () => ["authRefresh"],
    () => pb.collection("users").authRefresh(),
    { refetchInterval: 10 * (60 * 1000), refetchOnWindowFocus }
  );
