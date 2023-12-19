export async function fetchJson<T>(url: RequestInfo, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`${url}`, init)).json()
}

export async function fetchJsonFromBrawlhub<T>(endpoint: string, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`$https://api.brawlhub.net/${endpoint}`, init)).json()
}

export async function fetchJsonFromPublic<T>(endpoint: string, init?: RequestInit | undefined): Promise<T> {
  return (await fetch(`https://api.brawlhub.net/${endpoint}`, init)).json()
}
