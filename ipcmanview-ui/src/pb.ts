import PocketBase, { Admin, Record } from "pocketbase";
import { createSignal } from "solid-js";

const pb = new PocketBase(import.meta.env.VITE_BACKEND_URL);

type Auth = {
  token: string;
  model: Record | Admin | null;
  isValid: boolean;
};

const [authStore, setAuthStore] = createSignal<Auth>({
  token: pb.authStore.token,
  model: pb.authStore.model,
  isValid: pb.authStore.isValid,
});

pb.authStore.onChange(() => {
  setAuthStore({
    token: pb.authStore.token,
    model: pb.authStore.model,
    isValid: pb.authStore.isValid,
  });
});

try {
  pb.authStore.isValid && (await pb.collection("users").authRefresh());
} catch (_) {
  pb.authStore.clear();
}

export { authStore };
export default pb;
