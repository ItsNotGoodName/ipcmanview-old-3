import { createQuery, CreateQueryResult } from "@tanstack/solid-query";
import PocketBase, { ClientResponseError } from "pocketbase";
import {
  Accessor,
  createContext,
  createEffect,
  createSignal,
  JSX,
  Match,
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
  const [auth, setAuth] = createSignal<PbAuth>({
    token: pb.authStore.token,
    model: pb.authStore.model as any,
    isValid: pb.authStore.isValid,
  });

  createEffect(() => {
    return pb.authStore.onChange(() => {
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
    });
  });

  const authRefresh = createQuery(
    () => ["authRefresh"],
    () => {
      if (!pb.authStore.isValid) {
        return null;
      }
      return pb
        .collection("users")
        .authRefresh()
        .catch((e: ClientResponseError) => {
          if (e.status == 401) {
            pb.authStore.clear();
            return null;
          } else throw e;
        });
    },
    {
      refetchInterval: 10 * (60 * 1000),
      refetchOnWindowFocus: false,
    }
  );

  const store: PbContextType = {
    pb,
    pbUser: {
      user: () => auth().model as UserRecord,
      updateUser: (user: UserRecord) => {
        setAuth((prev) => {
          prev.model = user;
          return prev;
        });
      },
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
  updateUser: (user: UserRecord) => void;
};

export function usePb(): PocketBase {
  const result = useContext(PbContext);
  if (!result) throw new Error("usePb must be used within a PbProvider");

  return result.pb;
}

export function usePbUser(): [PbUser, CreateQueryResult<unknown>] {
  const result = useContext(PbContext);
  if (!result || !untrack(result.pbUserValid))
    throw new Error("useUser must be child of a PbProvider");

  return [
    result.pbUser,
    createQuery(
      () => ["authRefresh"],
      () => result.pb.collection("users").authRefresh()
    ),
  ];
}
