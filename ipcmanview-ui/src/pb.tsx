import PocketBase from "pocketbase";
import {
  Accessor,
  batch,
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
import { STATIONS_URL } from "./utils";

type PbContextType = {
  pb: PocketBase;
  userValid: Accessor<boolean>;
  user: Accessor<UserRecord>;
  updateUser: (user: UserRecord) => void;
};

const PbContext = createContext<PbContextType>();

type PbContextProps = {
  login: JSX.Element;
};

export const PbProvider: ParentComponent<PbContextProps> = (props) => {
  const pb = new PocketBase(import.meta.env.VITE_BACKEND_URL + "/");
  const [loading, setLoading] = createSignal(pb.authStore.isValid);
  const [auth, setAuth] = createSignal<PbAuth>({
    token: pb.authStore.token,
    model: pb.authStore.model as any,
    isValid: pb.authStore.isValid,
  });

  createEffect(() => {
    pb.authStore.onChange(() => {
      setAuth({
        token: pb.authStore.token,
        model: pb.authStore.model as any,
        isValid: pb.authStore.isValid,
      });
      document.cookie =
        "pb_token=" +
        pb.authStore.token +
        `;Path=${STATIONS_URL}` +
        import.meta.env.VITE_COOKIE_ATTRIBUTES;
    });
  });

  if (pb.authStore.isValid)
    // TODO: replace with useQuery and only clear store on PbError
    pb.collection("users")
      .authRefresh()
      .then(() => setLoading(false))
      .catch(() => {
        batch(() => {
          pb.authStore.clear();
          setLoading(false);
        });
      });

  const store: PbContextType = {
    pb,
    userValid: () => auth().isValid,
    user: () => auth().model as UserRecord,
    updateUser: (user: UserRecord) => {
      setAuth((prev) => {
        prev.model = user;
        return prev;
      });
    },
  };

  return (
    <PbContext.Provider value={store}>
      <Switch fallback={props.login}>
        <Match when={loading()}>Loading...</Match>
        <Match when={store.userValid()}>{props.children}</Match>
      </Switch>
    </PbContext.Provider>
  );
};

type User = Omit<PbContextType, "userValid" | "pb">;

export function usePb(): PocketBase {
  const result = useContext(PbContext);
  if (!result)
    throw new Error("usePb must be used within a PbProvider login prop");

  return result.pb;
}

export function useUser(): User {
  const result = useContext(PbContext);
  if (!result || !untrack(result.userValid))
    throw new Error("useUser must be used within a PbProvider");

  return result;
}
