import { z } from 'zod'

const paginationSchema = z.object({
  page: z.number().int().positive(),
  pageSize: z.number().int().positive(),
  total: z.number().int().nonnegative(),
})

const totpCodeSchema = z.string().trim().regex(/^\d{6}$/, 'TOTP code must be 6 digits.')
const optionalTextSchema = z.string().trim().min(1).optional()
const nullableTextSchema = z.string().trim().min(1).nullable()
const codeSchema = z
  .string()
  .trim()
  .min(2)
  .max(64)
  .regex(/^[a-z0-9][a-z0-9-]*$/, 'Code may only contain lowercase letters, numbers, and dashes.')

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
  keyword: optionalTextSchema,
  page: z.number().int().positive().optional(),
  pageSize: z.number().int().positive().max(100).optional(),
})

export const listAdminUsersResponseSchema = z.object({
  items: z.array(adminUserSchema),
  pagination: paginationSchema,
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

export const userPoolApplicationSummarySchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
  isActive: z.boolean(),
})

export const profileFieldSchema = z.object({
  id: z.uuid(),
  userPoolId: z.uuid(),
  fieldKey: z.string().min(1),
  fieldName: z.string().min(1),
  fieldType: z.string().min(1),
  isRequired: z.boolean(),
  isUnique: z.boolean(),
  isSearchable: z.boolean(),
  sortOrder: z.number().int(),
  options: z.unknown().nullable(),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const userPoolSummarySchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
  description: z.string().nullable(),
  isActive: z.boolean(),
  allowSelfSignup: z.boolean(),
  applicationCount: z.number().int().nonnegative(),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const userPoolDetailSchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
  description: z.string().nullable(),
  isActive: z.boolean(),
  allowSelfSignup: z.boolean(),
  profileFields: z.array(profileFieldSchema),
  applications: z.array(userPoolApplicationSummarySchema),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const listUserPoolsParamsSchema = z.object({
  keyword: optionalTextSchema,
  isActive: z.boolean().optional(),
  page: z.number().int().positive().optional(),
  pageSize: z.number().int().positive().max(100).optional(),
})

export const listUserPoolsResponseSchema = z.object({
  items: z.array(userPoolSummarySchema),
  pagination: paginationSchema,
})

export const createUserPoolPayloadSchema = z.object({
  code: codeSchema,
  name: z.string().trim().min(1).max(64),
  description: nullableTextSchema.optional(),
  allowSelfSignup: z.boolean().optional(),
})

export const updateUserPoolPayloadSchema = z.object({
  name: z.string().trim().min(1).max(64).optional(),
  description: nullableTextSchema.optional(),
  allowSelfSignup: z.boolean().optional(),
})

export const createProfileFieldPayloadSchema = z.object({
  fieldKey: z
    .string()
    .trim()
    .min(1)
    .max(64)
    .regex(/^[a-z0-9_]+$/, 'Field key may only contain lowercase letters, numbers, and underscores.'),
  fieldName: z.string().trim().min(1).max(64),
  fieldType: z.enum(['string', 'text', 'number', 'boolean', 'date', 'datetime', 'select', 'multi_select']),
  isRequired: z.boolean().optional(),
  isUnique: z.boolean().optional(),
  isSearchable: z.boolean().optional(),
  sortOrder: z.number().int().optional(),
  options: z.unknown().nullable().optional(),
})

export const updateProfileFieldPayloadSchema = z.object({
  fieldName: z.string().trim().min(1).max(64).optional(),
  isRequired: z.boolean().optional(),
  isSearchable: z.boolean().optional(),
  sortOrder: z.number().int().optional(),
  options: z.unknown().nullable().optional(),
})

export const profileFieldListResponseSchema = z.object({
  items: z.array(profileFieldSchema),
})

export const applicationUserPoolSchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
})

export const redirectUriSchema = z.object({
  id: z.uuid(),
  applicationId: z.uuid(),
  redirectUri: z.string().min(1),
  isPrimary: z.boolean(),
  createdAt: z.string(),
})

export const applicationSummarySchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
  description: z.string().nullable(),
  applicationType: z.enum(['confidential', 'public']),
  clientId: z.string().min(1),
  userPool: applicationUserPoolSchema,
  homepageUrl: z.string().nullable(),
  isActive: z.boolean(),
  redirectUriCount: z.number().int().nonnegative(),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const applicationDetailSchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  name: z.string().min(1),
  description: z.string().nullable(),
  applicationType: z.enum(['confidential', 'public']),
  clientId: z.string().min(1),
  userPool: applicationUserPoolSchema,
  homepageUrl: z.string().nullable(),
  isActive: z.boolean(),
  redirectUris: z.array(redirectUriSchema),
  createdAt: z.string(),
  updatedAt: z.string(),
})

export const listApplicationsParamsSchema = z.object({
  keyword: optionalTextSchema,
  userPoolId: z.uuid().optional(),
  isActive: z.boolean().optional(),
  page: z.number().int().positive().optional(),
  pageSize: z.number().int().positive().max(100).optional(),
})

export const listApplicationsResponseSchema = z.object({
  items: z.array(applicationSummarySchema),
  pagination: paginationSchema,
})

export const createRedirectUriPayloadSchema = z.object({
  redirectUri: z.string().trim().url(),
  isPrimary: z.boolean().optional(),
})

export const updateRedirectUriPayloadSchema = z.object({
  isPrimary: z.boolean(),
})

export const createApplicationPayloadSchema = z.object({
  code: codeSchema,
  name: z.string().trim().min(1).max(64),
  description: nullableTextSchema.optional(),
  applicationType: z.enum(['confidential', 'public']),
  userPoolId: z.uuid(),
  homepageUrl: z.string().trim().url().nullable().optional(),
  redirectUris: z.array(createRedirectUriPayloadSchema).min(1),
})

export const updateApplicationPayloadSchema = z.object({
  name: z.string().trim().min(1).max(64).optional(),
  description: nullableTextSchema.optional(),
  homepageUrl: z.string().trim().url().nullable().optional(),
})

export const createApplicationResponseSchema = z.object({
  id: z.uuid(),
  code: z.string().min(1),
  clientId: z.string().min(1),
  clientSecret: z.string().nullable(),
})

export const redirectUriListResponseSchema = z.object({
  items: z.array(redirectUriSchema),
})

export const resetApplicationSecretPayloadSchema = z.object({
  currentAdminPassword: z.string().min(8),
})

export const resetApplicationSecretResponseSchema = z.object({
  clientId: z.string().min(1),
  clientSecret: z.string().min(1),
  secretRotatedAt: z.string().nullable(),
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
export type AdminLoginAuthenticatedResponse = z.infer<typeof adminLoginAuthenticatedResponseSchema>
export type AdminLoginPendingTotpResponse = z.infer<typeof adminLoginPendingTotpResponseSchema>
export type AdminLoginResponse = z.infer<typeof adminLoginResponseSchema>
export type AdminLoginTotpPayload = z.infer<typeof adminLoginTotpPayloadSchema>
export type ListAdminUsersParams = z.infer<typeof listAdminUsersParamsSchema>
export type ListAdminUsersResponse = z.infer<typeof listAdminUsersResponseSchema>
export type CreateAdminUserPayload = z.infer<typeof createAdminUserPayloadSchema>
export type UpdateAdminUserPayload = z.infer<typeof updateAdminUserPayloadSchema>
export type UserPoolSummary = z.infer<typeof userPoolSummarySchema>
export type UserPoolDetail = z.infer<typeof userPoolDetailSchema>
export type ListUserPoolsParams = z.infer<typeof listUserPoolsParamsSchema>
export type ListUserPoolsResponse = z.infer<typeof listUserPoolsResponseSchema>
export type CreateUserPoolPayload = z.infer<typeof createUserPoolPayloadSchema>
export type UpdateUserPoolPayload = z.infer<typeof updateUserPoolPayloadSchema>
export type ProfileField = z.infer<typeof profileFieldSchema>
export type CreateProfileFieldPayload = z.infer<typeof createProfileFieldPayloadSchema>
export type UpdateProfileFieldPayload = z.infer<typeof updateProfileFieldPayloadSchema>
export type ApplicationUserPool = z.infer<typeof applicationUserPoolSchema>
export type RedirectUri = z.infer<typeof redirectUriSchema>
export type ApplicationSummary = z.infer<typeof applicationSummarySchema>
export type ApplicationDetail = z.infer<typeof applicationDetailSchema>
export type ListApplicationsParams = z.infer<typeof listApplicationsParamsSchema>
export type ListApplicationsResponse = z.infer<typeof listApplicationsResponseSchema>
export type CreateApplicationPayload = z.infer<typeof createApplicationPayloadSchema>
export type UpdateApplicationPayload = z.infer<typeof updateApplicationPayloadSchema>
export type CreateApplicationResponse = z.infer<typeof createApplicationResponseSchema>
export type CreateRedirectUriPayload = z.infer<typeof createRedirectUriPayloadSchema>
export type UpdateRedirectUriPayload = z.infer<typeof updateRedirectUriPayloadSchema>
export type ResetApplicationSecretPayload = z.infer<typeof resetApplicationSecretPayloadSchema>
export type ResetApplicationSecretResponse = z.infer<typeof resetApplicationSecretResponseSchema>
export type ChangeMyPasswordPayload = z.infer<typeof changeMyPasswordPayloadSchema>
export type TotpStatusResponse = z.infer<typeof totpStatusResponseSchema>
export type TotpSetupResponse = z.infer<typeof totpSetupResponseSchema>
export type ConfirmTotpPayload = z.infer<typeof confirmTotpPayloadSchema>
export type DisableTotpPayload = z.infer<typeof disableTotpPayloadSchema>
export type TotpUpdateResponse = z.infer<typeof totpUpdateResponseSchema>
export type SuccessResponse = z.infer<typeof successResponseSchema>
