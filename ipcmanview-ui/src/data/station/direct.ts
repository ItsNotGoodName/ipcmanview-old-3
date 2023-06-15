import { QueryKey } from "@tanstack/solid-query";
import { Accessor } from "solid-js";

import { StationApi } from ".";

export class DirectStationApi implements StationApi {
  constructor(readonly url: Accessor<string>) {}

  async send<T>(uri: string, reqOptions?: RequestInit): Promise<T> {
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
