import PocketBase from "pocketbase";
import { Accessor } from "solid-js";
import { createQuery, CreateQueryResult } from "@tanstack/solid-query";

import { Camera, CamerasTotal, ScanCompleted, ScanPending } from "./models";
import { PbError, StationRecord } from "./records";
import { STATIONS_URL } from "./utils";

function stationUrl(stationId: string): string {
  return STATIONS_URL + "/" + stationId;
}

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, PbError> => {
  return createQuery(
    () => ["stations"],
    () => pb.collection("stations").getFullList()
  );
};

export const useCameras = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, PbError> => {
  return createQuery(
    () => [stationId(), "/cameras"],
    () => pb.send(stationUrl(stationId()) + "/cameras", {})
  );
};

export const useScansPending = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanPending>, PbError> => {
  return createQuery(
    () => [stationId(), "/scans/pending"],
    () => pb.send(stationUrl(stationId()) + "/scans/pending", {})
  );
};

export const useScansCompleted = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanCompleted>, PbError> => {
  return createQuery(
    () => [stationId(), "/scans/completed"],
    () => pb.send(stationUrl(stationId()) + "/scans/completed", {})
  );
};

export const useCamerasTotal = (
  pb: PocketBase,
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, PbError> => {
  return createQuery(
    () => [stationId(), "/cameras-total"],
    () => pb.send(stationUrl(stationId()) + "/cameras-total", {})
  );
};

export const useAuthRefresh = (
  pb: PocketBase,
  {
    refetchOnWindowFocus,
  }: {
    refetchOnWindowFocus: boolean;
  }
) => {
  return createQuery(
    () => ["authRefresh"],
    () => pb.collection("users").authRefresh(),
    { refetchInterval: 10 * (60 * 1000), refetchOnWindowFocus }
  );
};
