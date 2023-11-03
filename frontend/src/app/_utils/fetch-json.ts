export async function fetchJson<T>(url: RequestInfo, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`${url}`, init)).json()
}
