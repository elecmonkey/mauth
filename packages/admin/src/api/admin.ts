import {
  adminLoginAuthenticatedResponseSchema,
  adminLoginPayloadSchema,
  adminLoginResponseSchema,
  adminLoginTotpPayloadSchema,
  changeMyPasswordPayloadSchema,
  confirmTotpPayloadSchema,
  createAdminUserPayloadSchema,
  disableTotpPayloadSchema,
  listAdminUsersParamsSchema,
  listAdminUsersResponseSchema,
  successResponseSchema,
  totpSetupResponseSchema,
  totpStatusResponseSchema,
  totpUpdateResponseSchema,
  updateAdminUserPayloadSchema,
  adminUserSchema,
  type AdminLoginPayload,
  type AdminLoginTotpPayload,
  type ChangeMyPasswordPayload,
  type ConfirmTotpPayload,
  type CreateAdminUserPayload,
  type DisableTotpPayload,
  type ListAdminUsersParams,
  type UpdateAdminUserPayload,
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
