import PocketBase from "pocketbase";
import { createSignal } from "solid-js";
import { UserRecord } from "./records";

const pb = new PocketBase(import.meta.env.VITE_BACKEND_URL + "/");

export const adminPageUrl = import.meta.env.VITE_BACKEND_URL + "/_/";

export const stationUrl = (stationId: string, path: string): string => {
  return "/app/stations/" + stationId + path;
};

type Auth = {
  token: string;
  model: UserRecord | null;
  isValid: boolean;
};

const [authStore, setAuthStore] = createSignal<Auth>({
  token: pb.authStore.token,
  model: pb.authStore.model as any,
  isValid: pb.authStore.isValid,
});

pb.authStore.onChange(() => {
  setAuthStore({
    token: pb.authStore.token,
    model: pb.authStore.model as any,
    isValid: pb.authStore.isValid,
  });
  document.cookie =
    "pb_token=" +
    pb.authStore.token +
    ";Path=/app/stations" +
    import.meta.env.VITE_COOKIE_ATTRIBUTES;
});

try {
  pb.authStore.isValid && (await pb.collection("users").authRefresh());
} catch (_) {
  pb.authStore.clear();
}

export const authStoreMutate = (user: UserRecord) => {
  setAuthStore((prev) => {
    prev.model = user;
    return prev;
  });
};

export default pb;
export { authStore };
