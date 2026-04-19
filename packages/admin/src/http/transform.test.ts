import { describe, expect, it } from 'vitest'
import { toCamelCase, toSnakeCase } from './transform'

describe('transform helpers', () => {
  it('converts snake_case payloads into camelCase', () => {
    const input = {
      access_token: 'token',
      token_type: 'Bearer',
      nested_item: {
        last_login_at: '2026-01-01T00:00:00Z',
      },
      items: [{ is_active: true }],
    }

    expect(toCamelCase(input)).toEqual({
      accessToken: 'token',
      tokenType: 'Bearer',
      nestedItem: {
        lastLoginAt: '2026-01-01T00:00:00Z',
      },
      items: [{ isActive: true }],
    })
  })

  it('converts camelCase payloads into snake_case', () => {
    const input = {
      accessToken: 'token',
      pageSize: 20,
      nestedItem: {
        isActive: false,
      },
    }

    expect(toSnakeCase(input)).toEqual({
      access_token: 'token',
      page_size: 20,
      nested_item: {
        is_active: false,
      },
    })
  })
})
