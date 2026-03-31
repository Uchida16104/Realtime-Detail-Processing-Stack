const backendUrl = import.meta.env.VITE_BACKEND_URL || 'http://localhost:8080'

export async function postJson<T>(path: string, body: unknown): Promise<T> {
  const response = await fetch(`${backendUrl}${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
  })

  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Request failed: ${response.status}`)
  }

  return response.json() as Promise<T>
}

export function getBackendUrl(): string {
  return backendUrl
}
