import { createQuery, CreateQueryResult } from "@tanstack/solid-query";
import { Accessor } from "solid-js";
import { Camera, ScanPending } from "./models";
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

export const useCameras = (
  stationId: Accessor<string>
): CreateQueryResult<Array<Camera>, PbError> => {
  return createQuery(
    () => [stationId(), "/cameras"],
    () => pb.send(stationUrl(stationId(), "/cameras"), {})
  );
};

export const useScansPending = (
  stationId: Accessor<string>
): CreateQueryResult<Array<ScanPending>, PbError> => {
  return createQuery(
    () => [stationId(), "/scans/pending"],
    () => pb.send(stationUrl(stationId(), "/scans/pending"), {})
  );
};

type CamerasTotal = {
  total: number;
};

export const useCamerasTotal = (
  stationId: Accessor<string>
): CreateQueryResult<CamerasTotal, PbError> => {
  return createQuery(
    () => [stationId(), "/cameras-total"],
    () => pb.send(stationUrl(stationId(), "/cameras-total"), {})
  );
};

export const useAuthRefresh = (refetchOnWindowFocus: boolean) => {
  return createQuery(
    () => ["authRefresh"],
    () => pb.collection("users").authRefresh(),
    { refetchInterval: 10 * (60 * 1000), refetchOnWindowFocus }
  );
};

// export const useCameraDelete = (
//   stationId: Accessor<string>
// ): CreateMutationResult<unknown, PbError, number> => {
//   const queryClient = useQueryClient();
//   return createMutation({
//     onSuccess: () => {
//       queryClient.invalidateQueries([stationId(), "cameras"]);
//       queryClient.invalidateQueries([stationId(), "cameras-total"]);
//     },
//     mutationFn: (cameraId: number) =>
//       pb.send(stationUrl(stationId(), `/cameras/${cameraId}`), {
//         method: "DELETE",
//       }),
//   });
// };
