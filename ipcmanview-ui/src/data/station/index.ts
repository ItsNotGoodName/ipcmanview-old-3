import { QueryKey } from "@tanstack/solid-query";
import { SendOptions } from "pocketbase";
import { createContext, useContext } from "solid-js";

export interface StationApi {
  send<T = any>(uri: string, reqOptions?: SendOptions): Promise<T>;
  wrapKey(key: QueryKey): QueryKey;
  unwrapKey(key: QueryKey): QueryKey | null;
  fileUrl(cameraId: number, filePath: string): string;
}

export type StationContextType = {
  api: StationApi;
};

export const StationContext = createContext<StationContextType>();

export function useStationApi(): StationApi {
  const result = useContext(StationContext);
  if (!result)
    throw new Error("useStationApi must be used within StationContext");

  return result.api;
}
