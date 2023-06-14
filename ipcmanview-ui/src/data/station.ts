import { QueryKey } from "@tanstack/solid-query";
import PocketBase, { SendOptions } from "pocketbase";
import { Accessor } from "solid-js";

import { stationUrl } from "./utils";

export interface StationApi {
  send<T = any>(uri: string, reqOptions?: SendOptions): Promise<T>;
  wrapKey(key: QueryKey): QueryKey;
  unwrapKey(key: QueryKey): QueryKey | null;
  fileUrl(cameraId: number, filePath: string): string;
}

export class StationApiPb implements StationApi {
  constructor(readonly pb: PocketBase, readonly stationId: Accessor<string>) {}

  send<T>(url: string, reqOptions?: SendOptions): Promise<T> {
    return this.pb.send<T>(
      stationUrl(this.stationId()) + url,
      reqOptions || {}
    );
  }

  wrapKey(key: QueryKey): QueryKey {
    return ["t*#5T", this.stationId(), key];
  }

  unwrapKey(key: QueryKey): QueryKey | null {
    if (key[0] === "t*#5T" && key[1] === this.stationId()) {
      return key[2] as QueryKey;
    }

    return null;
  }

  fileUrl(cameraId: number, filePath: string): string {
    return (
      import.meta.env.VITE_BACKEND_URL +
      stationUrl(this.stationId()) +
      "/cameras/" +
      cameraId +
      "/fs/" +
      filePath
    );
  }
}

export class StationApiDirect implements StationApi {
  constructor(readonly url: Accessor<string>) {}

  async send<T>(uri: string, reqOptions?: SendOptions): Promise<T> {
    // TODO: handle error and normalize error
    const res = await fetch(this.url + "/api" + uri, reqOptions);
    return await res.json();
  }

  wrapKey(key: QueryKey): QueryKey {
    return ["B2Dt@", this.url, key];
  }

  unwrapKey(key: QueryKey): QueryKey | null {
    if (key[0] === "B2Dt@" && key[1] === this.url) {
      return key[2] as QueryKey;
    }

    return null;
  }

  fileUrl(cameraId: number, filePath: string): string {
    return this.url() + "/api/cameras/" + cameraId + "/fs/" + filePath;
  }
}
