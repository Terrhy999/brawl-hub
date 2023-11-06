export async function fetchJson<T>(url: RequestInfo, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`${url}`, init)).json()
}

export async function fetchJsonFromBrawlhub<T>(endpoint: string, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`${process.env.URL}${endpoint}`, init)).json()
}

export async function fetchJsonFromPublic<T>(endpoint: string, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`${process.env.NEXT_PUBLIC_URL}${endpoint}`, init)).json()
}
