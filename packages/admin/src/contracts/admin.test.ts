import { describe, expect, it } from 'vitest'
import {
  adminLoginResponseSchema,
  adminLoginTotpPayloadSchema,
  totpSetupResponseSchema,
} from './admin'

const adminUser = {
  id: '3ff7fb6f-0d60-4f7f-a7cf-cbe8f0f44f19',
  email: 'admin@example.com',
  nickname: 'Root Admin',
  isActive: true,
  totpEnabled: true,
  lastLoginAt: '2026-04-19T00:00:00Z',
  createdAt: '2026-04-01T00:00:00Z',
  updatedAt: '2026-04-19T00:00:00Z',
}

describe('admin contracts', () => {
  it('parses the authenticated login response', () => {
    const result = adminLoginResponseSchema.parse({
      status: 'authenticated',
      requiresTotp: false,
      accessToken: 'access-token',
      tokenType: 'Bearer',
      expiresIn: 3600,
      challengeToken: null,
      challengeExpiresIn: null,
      user: adminUser,
    })

    expect(result.status).toBe('authenticated')
    expect(result.accessToken).toBe('access-token')
  })

  it('parses the pending TOTP login response', () => {
    const result = adminLoginResponseSchema.parse({
      status: 'pending_totp',
      requiresTotp: true,
      accessToken: null,
      tokenType: null,
      expiresIn: null,
      challengeToken: 'challenge-token',
      challengeExpiresIn: 300,
      user: adminUser,
    })

    expect(result.status).toBe('pending_totp')
    expect(result.challengeToken).toBe('challenge-token')
  })

  it('requires a 6-digit TOTP code for setup confirmation', () => {
    expect(() =>
      adminLoginTotpPayloadSchema.parse({
        challengeToken: 'challenge-token',
        totpCode: '12345',
      }),
    ).toThrow()
  })

  it('parses QR-based setup responses', () => {
    const result = totpSetupResponseSchema.parse({
      secret: 'JBSWY3DPEHPK3PXP',
      otpauthUri: 'otpauth://totp/MAuth:admin@example.com?secret=JBSWY3DPEHPK3PXP',
      qrSvg: '<svg viewBox="0 0 220 220"></svg>',
    })

    expect(result.qrSvg).toContain('<svg')
  })
})
