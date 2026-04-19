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

export const adminLoginResponseSchema = z.object({
  accessToken: z.string().min(1),
  tokenType: z.literal('Bearer'),
  expiresIn: z.number().int().positive(),
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
export type AdminLoginResponse = z.infer<typeof adminLoginResponseSchema>
export type ListAdminUsersParams = z.infer<typeof listAdminUsersParamsSchema>
export type ListAdminUsersResponse = z.infer<typeof listAdminUsersResponseSchema>
export type CreateAdminUserPayload = z.infer<typeof createAdminUserPayloadSchema>
export type UpdateAdminUserPayload = z.infer<typeof updateAdminUserPayloadSchema>
export type ChangeMyPasswordPayload = z.infer<typeof changeMyPasswordPayloadSchema>
export type SuccessResponse = z.infer<typeof successResponseSchema>
