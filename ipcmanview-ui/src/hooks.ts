import { createQuery, CreateQueryResult } from "@tanstack/solid-query";
import { Accessor } from "solid-js";
import pb, { stationUrl } from "./pb";
import { PbError, StationRecord } from "./records";

export const useStations = (): CreateQueryResult<
  Array<StationRecord>,
  PbError
> => {
  return createQuery(
    () => ["stations"],
    () => pb.collection("stations").getFullList()
  );
};

export type Camera = {
  id: number;
  ip: string;
  username: string;
};

export const useCameras = (
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, PbError> => {
  return createQuery(
    () => [stationId(), "cameras"],
    () => pb.send(stationUrl(stationId(), "/cameras"), {})
  );
};

type CamerasTotal = {
  total: number;
};

export const useCamerasTotal = (
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, PbError> => {
  return createQuery(
    () => [stationId(), "cameras-total"],
    () => pb.send(stationUrl(stationId(), "/cameras-total"), {})
  );
};

const authRefreshRefetchInterval = 10 * (60 * 1000);
export const useAuthRefresh = (refetchOnWindowFocus: boolean) => {
  return createQuery(
    () => ["authRefresh"],
    () => pb.collection("users").authRefresh(),
    { refetchInterval: authRefreshRefetchInterval, refetchOnWindowFocus }
  );
};
