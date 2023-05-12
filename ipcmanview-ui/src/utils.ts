export function formatDateTime(date: string): string {
  let d = new Date(date);
  return d.toLocaleDateString() + " " + d.toLocaleTimeString();
}
