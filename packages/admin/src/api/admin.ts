import {
  adminLoginPayloadSchema,
  adminLoginResponseSchema,
  changeMyPasswordPayloadSchema,
  createAdminUserPayloadSchema,
  listAdminUsersParamsSchema,
  listAdminUsersResponseSchema,
  successResponseSchema,
  updateAdminUserPayloadSchema,
  adminUserSchema,
  type AdminLoginPayload,
  type ChangeMyPasswordPayload,
  type CreateAdminUserPayload,
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
