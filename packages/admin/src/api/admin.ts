import {
  adminLoginAuthenticatedResponseSchema,
  adminLoginPayloadSchema,
  adminLoginResponseSchema,
  adminLoginTotpPayloadSchema,
  applicationDetailSchema,
  changeMyPasswordPayloadSchema,
  confirmTotpPayloadSchema,
  createAdminUserPayloadSchema,
  createApplicationPayloadSchema,
  createApplicationResponseSchema,
  createProfileFieldPayloadSchema,
  createRedirectUriPayloadSchema,
  createUserPoolPayloadSchema,
  disableTotpPayloadSchema,
  listAdminUsersParamsSchema,
  listAdminUsersResponseSchema,
  listApplicationsParamsSchema,
  listApplicationsResponseSchema,
  listUserPoolsParamsSchema,
  listUserPoolsResponseSchema,
  profileFieldListResponseSchema,
  profileFieldSchema,
  redirectUriListResponseSchema,
  redirectUriSchema,
  resetApplicationSecretPayloadSchema,
  resetApplicationSecretResponseSchema,
  successResponseSchema,
  totpSetupResponseSchema,
  totpStatusResponseSchema,
  totpUpdateResponseSchema,
  updateAdminUserPayloadSchema,
  updateApplicationPayloadSchema,
  updateProfileFieldPayloadSchema,
  updateRedirectUriPayloadSchema,
  updateUserPoolPayloadSchema,
  userPoolDetailSchema,
  userPoolSummarySchema,
  adminUserSchema,
  type AdminLoginPayload,
  type AdminLoginTotpPayload,
  type ChangeMyPasswordPayload,
  type ConfirmTotpPayload,
  type CreateAdminUserPayload,
  type CreateApplicationPayload,
  type CreateProfileFieldPayload,
  type CreateRedirectUriPayload,
  type CreateUserPoolPayload,
  type DisableTotpPayload,
  type ListAdminUsersParams,
  type ListApplicationsParams,
  type ListUserPoolsParams,
  type ResetApplicationSecretPayload,
  type UpdateAdminUserPayload,
  type UpdateApplicationPayload,
  type UpdateProfileFieldPayload,
  type UpdateRedirectUriPayload,
  type UpdateUserPoolPayload,
} from '../contracts'
import { request } from '../http'

export function loginAdmin(payload: AdminLoginPayload) {
  return request({
    path: '/admin/login',
    method: 'POST',
    auth: false,
    body: adminLoginPayloadSchema.parse(payload),
    responseSchema: adminLoginResponseSchema,
  })
}

export function loginAdminTotp(payload: AdminLoginTotpPayload) {
  return request({
    path: '/admin/login/totp',
    method: 'POST',
    auth: false,
    body: adminLoginTotpPayloadSchema.parse(payload),
    responseSchema: adminLoginAuthenticatedResponseSchema,
  })
}

export function fetchAdminUsers(params: ListAdminUsersParams = {}) {
  return request({
    path: '/admin/users',
    query: listAdminUsersParamsSchema.parse(params),
    responseSchema: listAdminUsersResponseSchema,
  })
}

export function createAdminUser(payload: CreateAdminUserPayload) {
  return request({
    path: '/admin/users',
    method: 'POST',
    body: createAdminUserPayloadSchema.parse(payload),
    responseSchema: adminUserSchema,
  })
}

export function updateAdminUser(id: string, payload: UpdateAdminUserPayload) {
  return request({
    path: `/admin/users/${id}`,
    method: 'PATCH',
    body: updateAdminUserPayloadSchema.parse(payload),
    responseSchema: adminUserSchema,
  })
}

export function deleteAdminUser(id: string) {
  return request({
    path: `/admin/users/${id}`,
    method: 'DELETE',
    responseSchema: successResponseSchema,
  })
}

export function fetchUserPools(params: ListUserPoolsParams = {}) {
  return request({
    path: '/admin/user-pools',
    query: listUserPoolsParamsSchema.parse(params),
    responseSchema: listUserPoolsResponseSchema,
  })
}

export function createUserPool(payload: CreateUserPoolPayload) {
  return request({
    path: '/admin/user-pools',
    method: 'POST',
    body: createUserPoolPayloadSchema.parse(payload),
    responseSchema: userPoolSummarySchema,
  })
}

export function fetchUserPoolDetail(id: string) {
  return request({
    path: `/admin/user-pools/${id}`,
    responseSchema: userPoolDetailSchema,
  })
}

export function updateUserPool(id: string, payload: UpdateUserPoolPayload) {
  return request({
    path: `/admin/user-pools/${id}`,
    method: 'PATCH',
    body: updateUserPoolPayloadSchema.parse(payload),
    responseSchema: userPoolSummarySchema,
  })
}

export function enableUserPool(id: string) {
  return request({
    path: `/admin/user-pools/${id}/enable`,
    method: 'POST',
    responseSchema: userPoolSummarySchema,
  })
}

export function disableUserPool(id: string) {
  return request({
    path: `/admin/user-pools/${id}/disable`,
    method: 'POST',
    responseSchema: userPoolSummarySchema,
  })
}

export function deleteUserPool(id: string) {
  return request({
    path: `/admin/user-pools/${id}`,
    method: 'DELETE',
    responseSchema: successResponseSchema,
  })
}

export function fetchProfileFields(userPoolId: string) {
  return request({
    path: `/admin/user-pools/${userPoolId}/profile-fields`,
    responseSchema: profileFieldListResponseSchema,
  })
}

export function createProfileField(userPoolId: string, payload: CreateProfileFieldPayload) {
  return request({
    path: `/admin/user-pools/${userPoolId}/profile-fields`,
    method: 'POST',
    body: createProfileFieldPayloadSchema.parse(payload),
    responseSchema: profileFieldSchema,
  })
}

export function updateProfileField(
  userPoolId: string,
  fieldId: string,
  payload: UpdateProfileFieldPayload,
) {
  return request({
    path: `/admin/user-pools/${userPoolId}/profile-fields/${fieldId}`,
    method: 'PATCH',
    body: updateProfileFieldPayloadSchema.parse(payload),
    responseSchema: profileFieldSchema,
  })
}

export function deleteProfileField(userPoolId: string, fieldId: string) {
  return request({
    path: `/admin/user-pools/${userPoolId}/profile-fields/${fieldId}`,
    method: 'DELETE',
    responseSchema: successResponseSchema,
  })
}

export function fetchApplications(params: ListApplicationsParams = {}) {
  return request({
    path: '/admin/applications',
    query: listApplicationsParamsSchema.parse(params),
    responseSchema: listApplicationsResponseSchema,
  })
}

export function createApplication(payload: CreateApplicationPayload) {
  return request({
    path: '/admin/applications',
    method: 'POST',
    body: createApplicationPayloadSchema.parse(payload),
    responseSchema: createApplicationResponseSchema,
  })
}

export function fetchApplicationDetail(id: string) {
  return request({
    path: `/admin/applications/${id}`,
    responseSchema: applicationDetailSchema,
  })
}

export function updateApplication(id: string, payload: UpdateApplicationPayload) {
  return request({
    path: `/admin/applications/${id}`,
    method: 'PATCH',
    body: updateApplicationPayloadSchema.parse(payload),
    responseSchema: applicationDetailSchema,
  })
}

export function enableApplication(id: string) {
  return request({
    path: `/admin/applications/${id}/enable`,
    method: 'POST',
    responseSchema: applicationDetailSchema,
  })
}

export function disableApplication(id: string) {
  return request({
    path: `/admin/applications/${id}/disable`,
    method: 'POST',
    responseSchema: applicationDetailSchema,
  })
}

export function deleteApplication(id: string) {
  return request({
    path: `/admin/applications/${id}`,
    method: 'DELETE',
    responseSchema: successResponseSchema,
  })
}

export function resetApplicationSecret(id: string, payload: ResetApplicationSecretPayload) {
  return request({
    path: `/admin/applications/${id}/reset-secret`,
    method: 'POST',
    body: resetApplicationSecretPayloadSchema.parse(payload),
    responseSchema: resetApplicationSecretResponseSchema,
  })
}

export function fetchRedirectUris(applicationId: string) {
  return request({
    path: `/admin/applications/${applicationId}/redirect-uris`,
    responseSchema: redirectUriListResponseSchema,
  })
}

export function createRedirectUri(applicationId: string, payload: CreateRedirectUriPayload) {
  return request({
    path: `/admin/applications/${applicationId}/redirect-uris`,
    method: 'POST',
    body: createRedirectUriPayloadSchema.parse(payload),
    responseSchema: redirectUriSchema,
  })
}

export function updateRedirectUri(
  applicationId: string,
  redirectUriId: string,
  payload: UpdateRedirectUriPayload,
) {
  return request({
    path: `/admin/applications/${applicationId}/redirect-uris/${redirectUriId}`,
    method: 'PATCH',
    body: updateRedirectUriPayloadSchema.parse(payload),
    responseSchema: redirectUriSchema,
  })
}

export function deleteRedirectUri(applicationId: string, redirectUriId: string) {
  return request({
    path: `/admin/applications/${applicationId}/redirect-uris/${redirectUriId}`,
    method: 'DELETE',
    responseSchema: successResponseSchema,
  })
}

export function changeMyPassword(payload: ChangeMyPasswordPayload) {
  return request({
    path: '/admin/me/password',
    method: 'POST',
    body: changeMyPasswordPayloadSchema.parse(payload),
    responseSchema: successResponseSchema,
  })
}

export function fetchMyTotpStatus() {
  return request({
    path: '/admin/me/totp',
    responseSchema: totpStatusResponseSchema,
  })
}

export function setupMyTotp() {
  return request({
    path: '/admin/me/totp/setup',
    method: 'POST',
    responseSchema: totpSetupResponseSchema,
  })
}

export function confirmMyTotp(payload: ConfirmTotpPayload) {
  return request({
    path: '/admin/me/totp/confirm',
    method: 'POST',
    body: confirmTotpPayloadSchema.parse(payload),
    responseSchema: totpUpdateResponseSchema,
  })
}

export function disableMyTotp(payload: DisableTotpPayload) {
  return request({
    path: '/admin/me/totp/disable',
    method: 'POST',
    body: disableTotpPayloadSchema.parse(payload),
    responseSchema: totpUpdateResponseSchema,
  })
}
