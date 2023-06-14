import {
  FieldValues,
  FormError,
  FormErrors,
  FormStore,
  reset,
  ResponseData,
} from "@modular-forms/solid";
import { CreateMutationResult, CreateQueryResult } from "@tanstack/solid-query";
import { ClientResponseError } from "pocketbase";
import { Accessor, createMemo } from "solid-js";

export function formatDateTime(date: Date | string): string {
  let d = new Date(date);
  return d.toLocaleDateString() + " " + d.toLocaleTimeString();
}

export function nameToInitials(name: string): string {
  const words = name.split(" ");
  if (words.length < 1) return "?";

  if (words.length < 2) return words[0][0] ?? "?";

  return (words[0][0] ?? "?") + (words[1][0] ?? "");
}

export const STATIONS_URI = "/app/stations";
export const ADMIN_PANEL_URL = import.meta.env.VITE_BACKEND_URL + "/_/";

export function stationUrl(stationId: string): string {
  return STATIONS_URI + "/" + stationId;
}

export function fileUrl(
  stationId: string,
  cameraId: number,
  filePath: string
): string {
  return (
    import.meta.env.VITE_BACKEND_URL +
    stationUrl(stationId) +
    "/cameras/" +
    cameraId +
    "/fs/" +
    filePath
  );
}

export function searchParamsFromObject(
  obj: Record<string, any>
): URLSearchParams {
  const s = new URLSearchParams();
  for (let k of Object.keys(obj)) {
    if (Array.isArray(obj[k])) {
      for (let v of obj[k]) {
        s.append(k, v.toString());
      }
    } else if (obj[k] instanceof Date) {
      s.append(k, (obj[k] as Date).toISOString());
    } else if (typeof obj[k] !== "undefined") {
      s.append(k, obj[k].toString());
    }
  }
  return s;
}

export function createMutationForm<
  TFieldValues extends FieldValues,
  TVariables
>(
  mutationResult: CreateMutationResult<
    unknown,
    ClientResponseError,
    TVariables
  >,
  formStore: FormStore<TFieldValues, ResponseData>
): [
  (data: TVariables) => Promise<unknown>,
  Accessor<FormError<TFieldValues> | null>
] {
  return [
    async (d) => {
      try {
        return await mutationResult.mutateAsync(d, {
          onSuccess: () => {
            reset(formStore);
          },
        });
      } catch (e) {
        console.log(e);
      }
    },
    createMemo(() =>
      formErrorsFromMutation<TFieldValues>(mutationResult.error)
    ),
  ];
}

function formErrorsFromMutation<T extends FieldValues>(
  err: ClientResponseError | null
): FormError<T> | null {
  if (!err) {
    return null;
  }

  if (err.response.data) {
    let keys = Object.keys(err.response.data) as Array<keyof T>;
    if (keys.length > 0) {
      let newFieldErrors: FormErrors<T> = {};
      for (const key of keys) {
        //@ts-ignore
        newFieldErrors[key] = err.response.data[key].message;
      }
      return new FormError("", newFieldErrors);
    }
  }

  return new FormError(err.message || "");
}

export type PageResult<T> = {
  page: number;
  per_page: number;
  total_pages: number;
  total_items: number;
  items: T;
};

export type Paging = { has_previous: boolean; has_next: boolean };

export function createPaging<T, U = unknown>(
  query: CreateQueryResult<PageResult<T>, U>
): Accessor<Paging> {
  return createMemo(() => {
    let has_previous = false;
    let has_next = false;
    if (query.data && !query.isPreviousData) {
      if (query.data.page > 1) {
        has_previous = true;
      }

      if (query.data.page < query.data.total_pages) {
        has_next = true;
      }
    }
    return { has_previous, has_next };
  });
}
