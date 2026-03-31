export type StreamEvent = {
  kind: string
  status: string
  timestamp: string
  detail: Record<string, unknown>
}
