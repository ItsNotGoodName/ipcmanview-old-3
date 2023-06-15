import { createQuery, CreateQueryResult } from "@tanstack/solid-query";
import PocketBase, { ClientResponseError } from "pocketbase";

import { StationRecord } from "./records";

export const useStations = (
  pb: PocketBase
): CreateQueryResult<Array<StationRecord>, ClientResponseError> =>
  createQuery(
    () => ["stations"],
    () => pb.collection("stations").getFullList()
  );
