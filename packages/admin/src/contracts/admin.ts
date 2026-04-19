import { z } from 'zod'

export const adminUserSchema = z.object({
  id: z.uuid(),
  email: z.email(),
  nickname: z.string().min(1),
  isActive: z.boolean(),
  totpEnabled: z.boolean(),
  lastLoginAt: z.string().nullable(),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const adminLoginPayloadSchema = z.object({
  email: z.email(),
  password: z.string().min(8),
})

const totpCodeSchema = z.string().trim().regex(/^\d{6}$/, 'TOTP code must be 6 digits.')

export const adminLoginAuthenticatedResponseSchema = z.object({
  status: z.literal('authenticated'),
  requiresTotp: z.literal(false),
  accessToken: z.string().min(1),
  tokenType: z.literal('Bearer'),
  expiresIn: z.number().int().positive(),
  challengeToken: z.null(),
  challengeExpiresIn: z.null(),
  user: adminUserSchema,
})

export const adminLoginPendingTotpResponseSchema = z.object({
  status: z.literal('pending_totp'),
  requiresTotp: z.literal(true),
  accessToken: z.null(),
  tokenType: z.null(),
  expiresIn: z.null(),
  challengeToken: z.string().min(1),
  challengeExpiresIn: z.number().int().positive(),
  user: adminUserSchema,
})

export const adminLoginResponseSchema = z.discriminatedUnion('status', [
  adminLoginAuthenticatedResponseSchema,
  adminLoginPendingTotpResponseSchema,
])

export const adminLoginTotpPayloadSchema = z.object({
  challengeToken: z.string().min(1),
  totpCode: totpCodeSchema,
})

export const totpStatusResponseSchema = z.object({
  enabled: z.boolean(),
})

export const totpSetupResponseSchema = z.object({
  secret: z.string().min(1),
  otpauthUri: z.string().min(1),
  qrSvg: z.string().min(1),
})

export const confirmTotpPayloadSchema = z.object({
  totpCode: totpCodeSchema,
})

export const disableTotpPayloadSchema = z.object({
  currentPassword: z.string().min(8),
  totpCode: totpCodeSchema,
})

export const totpUpdateResponseSchema = z.object({
  success: z.literal(true),
  user: adminUserSchema,
})

export const listAdminUsersParamsSchema = z.object({
  keyword: z.string().trim().min(1).optional(),
  page: z.number().int().positive().optional(),
  pageSize: z.number().int().positive().max(100).optional(),
})

export const listAdminUsersResponseSchema = z.object({
  items: z.array(adminUserSchema),
  pagination: z.object({
    page: z.number().int().positive(),
    pageSize: z.number().int().positive(),
    total: z.number().int().nonnegative(),
  }),
})

export const createAdminUserPayloadSchema = z.object({
  email: z.email(),
  nickname: z.string().trim().min(1),
  password: z.string().min(8),
  isActive: z.boolean().optional(),
})

export const updateAdminUserPayloadSchema = z.object({
  nickname: z.string().trim().min(1).optional(),
  isActive: z.boolean().optional(),
})

export const changeMyPasswordPayloadSchema = z.object({
  currentPassword: z.string().min(8),
  newPassword: z.string().min(8),
})

export const successResponseSchema = z.object({
  success: z.boolean(),
})

export type AdminUser = z.infer<typeof adminUserSchema>
export type AdminLoginPayload = z.infer<typeof adminLoginPayloadSchema>
export type AdminLoginAuthenticatedResponse = z.infer<
  typeof adminLoginAuthenticatedResponseSchema
>
export type AdminLoginPendingTotpResponse = z.infer<typeof adminLoginPendingTotpResponseSchema>
export type AdminLoginResponse = z.infer<typeof adminLoginResponseSchema>
export type AdminLoginTotpPayload = z.infer<typeof adminLoginTotpPayloadSchema>
export type ListAdminUsersParams = z.infer<typeof listAdminUsersParamsSchema>
export type ListAdminUsersResponse = z.infer<typeof listAdminUsersResponseSchema>
export type CreateAdminUserPayload = z.infer<typeof createAdminUserPayloadSchema>
export type UpdateAdminUserPayload = z.infer<typeof updateAdminUserPayloadSchema>
export type ChangeMyPasswordPayload = z.infer<typeof changeMyPasswordPayloadSchema>
export type TotpStatusResponse = z.infer<typeof totpStatusResponseSchema>
export type TotpSetupResponse = z.infer<typeof totpSetupResponseSchema>
export type ConfirmTotpPayload = z.infer<typeof confirmTotpPayloadSchema>
export type DisableTotpPayload = z.infer<typeof disableTotpPayloadSchema>
export type TotpUpdateResponse = z.infer<typeof totpUpdateResponseSchema>
export type SuccessResponse = z.infer<typeof successResponseSchema>
