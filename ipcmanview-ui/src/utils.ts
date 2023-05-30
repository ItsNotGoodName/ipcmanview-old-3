export function formatDateTime(date: string): string {
  let d = new Date(date);
  return d.toLocaleDateString() + " " + d.toLocaleTimeString();
}

export const STATIONS_URI = "/app/stations";
export const ADMIN_PANEL_URL = import.meta.env.VITE_BACKEND_URL + "/_/";
