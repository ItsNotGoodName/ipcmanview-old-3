import PocketBase from "pocketbase";
import { createSignal } from "solid-js";

const pb = new PocketBase(import.meta.env.VITE_BACKEND_URL);

export type PbError = {
  code: number;
  message: string;
  data: {
    [string: string]: Omit<PbError, "data">;
  };
};

export type UserRecord = {
  avatar: string;
  collectionId: string;
  collectionName: string;
  created: string;
  email: string;
  emailVisibility: boolean;
  id: string;
  name: string;
  updated: string;
  username: string;
  verified: boolean;
};

export type StationRecord = {
  id: string;
  url: string;
  name: string;
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
});

try {
  pb.authStore.isValid && (await pb.collection("users").authRefresh());
} catch (_) {
  pb.authStore.clear();
}

export const authStoreEagerUpdate = (user: UserRecord) => {
  setAuthStore((prev) => {
    prev.model = user;
    return prev;
  });
};

export default pb;
export { authStore };
