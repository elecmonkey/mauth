const isPlainObject = (value: unknown): value is Record<string, unknown> =>
  Object.prototype.toString.call(value) === '[object Object]'

const snakeCache = new Map<string, string>()
const camelCache = new Map<string, string>()

const camelize = (key: string): string => {
  if (camelCache.has(key)) return camelCache.get(key)!
  const result = key.replace(/[_-](\w)/g, (_, c: string) => c.toUpperCase())
  camelCache.set(key, result)
  return result
}

const snakify = (key: string): string => {
  if (snakeCache.has(key)) return snakeCache.get(key)!
  const result = key
    .replace(/([A-Z])/g, '_$1')
    .replace(/[^a-zA-Z0-9]+/g, '_')
    .replace(/_{2,}/g, '_')
    .toLowerCase()
    .replace(/^_+/, '')
    .replace(/_+$/, '')
  snakeCache.set(key, result)
  return result
}

const shouldTransform = (value: unknown): boolean => isPlainObject(value) || Array.isArray(value)

const transformKeys = (value: unknown, transformKey: (key: string) => string): unknown => {
  if (Array.isArray(value)) {
    return value.map((item) => transformKeys(item, transformKey))
  }

  if (!isPlainObject(value)) {
    return value
  }

  const result: Record<string, unknown> = {}
  Object.entries(value).forEach(([key, val]) => {
    const nextKey = transformKey(key)
    result[nextKey] = shouldTransform(val) ? transformKeys(val, transformKey) : val
  })
  return result
}

export const toCamelCase = <T>(value: T): T => transformKeys(value, camelize) as T

export const toSnakeCase = <T>(value: T): T => transformKeys(value, snakify) as T
