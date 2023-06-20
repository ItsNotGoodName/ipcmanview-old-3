import {
  createQuery,
  CreateQueryResult,
  QueryKey,
} from "@tanstack/solid-query";
import PocketBase, { SendOptions } from "pocketbase";
import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  Match,
  onCleanup,
  ParentComponent,
  Switch,
  untrack,
  useContext,
} from "solid-js";

import { PbAuth, StationRecord, UserRecord } from "./records";
import { StationApi, StationContext, StationContextType } from "~/data/station";

const STATIONS_URI = "/app/stations";

// PocketBase

type PbContextType = {
  pb: PocketBase;
  pbUser: PbUser;
  pbUserValid: Accessor<boolean>;
};

const PbContext = createContext<PbContextType>();

type PbContextProps = {
  loading: JSX.Element;
  login: JSX.Element;
};

export const PbProvider: ParentComponent<PbContextProps> = (props) => {
  const pb = new PocketBase(import.meta.env.VITE_BACKEND_URL + "/");

  pb.autoCancellation(false);

  pb.afterSend = (response, data) => {
    if (response.status == 401 && auth().isValid) {
      console.log("No longer authenticated.");
      pb.authStore.clear();
    }

    return data;
  };

  const [auth, setAuth] = createSignal<PbAuth>({
    token: pb.authStore.token,
    model: pb.authStore.model as any,
    isValid: pb.authStore.isValid,
  });

  onCleanup(
    pb.authStore.onChange(() => {
      setAuth({
        token: pb.authStore.token,
        model: pb.authStore.model as any,
        isValid: pb.authStore.isValid,
      });
      document.cookie =
        "pb_token=" +
        pb.authStore.token +
        `;Path=${STATIONS_URI}` +
        import.meta.env.VITE_COOKIE_ATTRIBUTES;
    })
  );

  const authRefresh = createQuery(
    () => ["authRefresh"],
    () => {
      if (!pb.authStore.isValid) {
        return null;
      }
      return pb.collection("users").authRefresh();
    },
    { refetchInterval: 10 * (60 * 1000) }
  );

  const store: PbContextType = {
    pb,
    pbUser: {
      user: () => auth().model as UserRecord,
      set: (user: UserRecord) => {
        setAuth((prev) => {
          prev.model = user;
          return prev;
        });
      },
      query: authRefresh,
    },
    pbUserValid: () => auth().isValid,
  };

  return (
    <PbContext.Provider value={store}>
      <Switch fallback={props.login}>
        <Match when={authRefresh.isLoading}>{props.loading}</Match>
        <Match when={store.pbUserValid()}>{props.children}</Match>
      </Switch>
    </PbContext.Provider>
  );
};

// StationApi

function stationUrl(stationId: string): string {
  return STATIONS_URI + "/" + stationId;
}

export class PbStationApi implements StationApi {
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

type PbUser = {
  user: Accessor<UserRecord>;
  set: (user: UserRecord) => void;
  query: CreateQueryResult<unknown, unknown>;
};

export function usePb(): PocketBase {
  const result = useContext(PbContext);
  if (!result) throw new Error("usePb must be used within a PbProvider");

  return result.pb;
}

export function usePbUser(): PbUser {
  const result = useContext(PbContext);
  if (!result || !untrack(result.pbUserValid))
    throw new Error("useUser must be child of a PbProvider");

  return result.pbUser;
}

export type PbStationApiContextProps = {
  stationId: string;
};

export const PbStationApiProvider: ParentComponent<PbStationApiContextProps> = (
  props
) => {
  const pb = usePb();
  const station = createQuery(
    () => ["stations", props.stationId],
    () => pb.collection("stations").getOne<StationRecord>(props.stationId)
  );

  const store: StationContextType = {
    api: new PbStationApi(pb, () => props.stationId),
    station,
  };

  return (
    <StationContext.Provider value={store}>
      {props.children}
    </StationContext.Provider>
  );
};
