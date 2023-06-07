import { createQuery, CreateQueryResult } from "@tanstack/solid-query";
import PocketBase from "pocketbase";
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

import { PbAuth, UserRecord } from "./records";
import { STATIONS_URI } from "./utils";

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
