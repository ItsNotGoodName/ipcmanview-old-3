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
