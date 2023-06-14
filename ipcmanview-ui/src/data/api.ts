import { QueryKey } from "@tanstack/solid-query";
import PocketBase, { SendOptions } from "pocketbase";
import { Accessor } from "solid-js";

import { stationUrl } from "./utils";

export interface StationApi {
  send<T = any>(uri: string, reqOptions?: SendOptions): Promise<T>;
  key(key: QueryKey): QueryKey;
  getKey(key: QueryKey): QueryKey | null;
}

export class ProxyStationApi implements StationApi {
  constructor(readonly pb: PocketBase, readonly stationId: Accessor<string>) {}

  send<T>(url: string, reqOptions?: SendOptions): Promise<T> {
    return this.pb.send<T>(
      stationUrl(this.stationId()) + url,
      reqOptions || {}
    );
  }

  key(key: QueryKey): QueryKey {
    return ["station-proxy", this.stationId(), key];
  }

  getKey(key: QueryKey): QueryKey | null {
    if (key[0] === "station-proxy" && key[1] === this.stationId()) {
      return key[2] as QueryKey;
    }

    return null;
  }
}

export class DirectStationApi implements StationApi {
  constructor(readonly url: Accessor<string>) {}

  async send<T>(uri: string, reqOptions?: SendOptions): Promise<T> {
    // TODO: handle error and normalize error
    const res = await fetch(this.url + "/api" + uri, reqOptions);
    return await res.json();
  }

  key(key: QueryKey): QueryKey {
    return ["station-direct", this.url, key];
  }

  getKey(key: QueryKey): QueryKey | null {
    if (key[0] === "station-direct" && key[1] === this.url) {
      return key[2] as QueryKey;
    }

    return null;
  }
}
