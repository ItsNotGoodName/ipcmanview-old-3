import {
  FieldValues,
  FormError,
  FormErrors,
  FormStore,
  reset,
  ResponseData,
} from "@modular-forms/solid";
import { CreateMutationResult } from "@tanstack/solid-query";
import { ClientResponseError } from "pocketbase";
import { Accessor, createMemo } from "solid-js";

export function formatDateTime(date: string): string {
  let d = new Date(date);
  return d.toLocaleDateString() + " " + d.toLocaleTimeString();
}

export const STATIONS_URI = "/app/stations";
export const ADMIN_PANEL_URL = import.meta.env.VITE_BACKEND_URL + "/_/";

export function paramsFromObject(obj: Record<string, any>): URLSearchParams {
  const s = new URLSearchParams();
  for (let k of Object.keys(obj)) {
    if (Array.isArray(obj[k])) {
      for (let v of obj[k]) {
        s.append(k, v.toString());
      }
    } else if (obj[k] instanceof Date) {
      s.append(k, (obj[k] as Date).toISOString());
    } else {
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
): [(data: TVariables) => void, Accessor<FormError<TFieldValues> | null>] {
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
    createMemo(() => {
      return parseErrors<TFieldValues>(mutationResult.error);
    }),
  ];
}

function parseErrors<T extends FieldValues>(
  err: ClientResponseError | null
): FormError<T> | null {
  if (!err) {
    return null;
  }

  let keys = Object.keys(err.response.data) as Array<keyof T>;
  if (keys.length > 0) {
    let newFieldErrors: FormErrors<T> = {};
    for (const key of keys) {
      //@ts-ignore
      newFieldErrors[key] = err.response.data[key].message;
    }
    return new FormError("", newFieldErrors);
  }

  return new FormError(err.message || "");
}
