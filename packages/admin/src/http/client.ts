import type { ZodType } from 'zod'
import { clearSession, getAccessToken } from '../stores/auth'
import { toCamelCase, toSnakeCase } from './transform'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://127.0.0.1:5610'
const API_TIMEOUT = Number(import.meta.env.VITE_API_TIMEOUT) || 10000

export class ApiError extends Error {
  status: number
  payload?: unknown

  constructor(message: string, status: number, payload?: unknown) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.payload = payload
  }
}

export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError
}

interface RequestOptions<T> {
  path: string
  method?: 'GET' | 'POST' | 'PATCH' | 'PUT' | 'DELETE'
  query?: Record<string, unknown>
  body?: unknown
  headers?: Record<string, string>
  auth?: boolean
  responseSchema?: ZodType<T>
}

function buildUrl(path: string, query?: Record<string, unknown>) {
  const url = new URL(path, API_BASE_URL)
  if (!query) return url.toString()

  const normalizedQuery = toSnakeCase(query) as Record<string, unknown>
  Object.entries(normalizedQuery).forEach(([key, value]) => {
    if (value === undefined || value === null || value === '') return
    url.searchParams.set(key, String(value))
  })

  return url.toString()
}

function parseJson(text: string): unknown {
  if (!text) return null

  try {
    return JSON.parse(text)
  } catch {
    return text
  }
}

export async function request<T>({
  path,
  method = 'GET',
  query,
  body,
  headers = {},
  auth = true,
  responseSchema,
}: RequestOptions<T>): Promise<T> {
  const controller = new AbortController()
  const timeoutId = window.setTimeout(() => controller.abort(), API_TIMEOUT)

  try {
    const token = auth ? getAccessToken() : null
    const response = await fetch(buildUrl(path, query), {
      method,
      headers: {
        Accept: 'application/json',
        ...(body ? { 'Content-Type': 'application/json' } : {}),
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...headers,
      },
      body: body ? JSON.stringify(toSnakeCase(body)) : undefined,
      signal: controller.signal,
    })

    const rawText = await response.text()
    const rawData = parseJson(rawText)
    const data = rawData && typeof rawData === 'object' ? toCamelCase(rawData) : rawData

    if (!response.ok) {
      if (response.status === 401) {
        clearSession()
      }

      const message =
        typeof data === 'object' &&
        data !== null &&
        'error' in data &&
        typeof data.error === 'string'
          ? data.error
          : 'Request failed'

      throw new ApiError(message, response.status, data)
    }

    if (!responseSchema) {
      return data as T
    }

    return responseSchema.parse(data)
  } catch (error) {
    if (error instanceof ApiError) {
      throw error
    }

    if (error instanceof Error && error.name === 'AbortError') {
      throw new ApiError('Request timeout', 408)
    }

    if (error instanceof Error) {
      throw new ApiError(error.message, 0)
    }

    throw new ApiError('Unknown request error', 0)
  } finally {
    window.clearTimeout(timeoutId)
  }
}
