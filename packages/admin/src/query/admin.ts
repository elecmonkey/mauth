import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import {
  changeMyPassword,
  confirmMyTotp,
  createAdminUser,
  deleteAdminUser,
  fetchAdminUsers,
  fetchMyTotpStatus,
  loginAdminTotp,
  loginAdmin,
  setupMyTotp,
  disableMyTotp,
  updateAdminUser,
} from '../api'
import type {
  AdminLoginPayload,
  AdminLoginTotpPayload,
  ChangeMyPasswordPayload,
  ConfirmTotpPayload,
  CreateAdminUserPayload,
  DisableTotpPayload,
  ListAdminUsersParams,
  UpdateAdminUserPayload,
} from '../types/index'

export const adminQueryKeys = {
  all: ['admin'] as const,
  users: (params: ListAdminUsersParams) => ['admin', 'users', params] as const,
  totpStatus: () => ['admin', 'totp-status'] as const,
}

export function useAdminLoginMutation() {
  return useMutation({
    mutationFn: (payload: AdminLoginPayload) => loginAdmin(payload),
  })
}

export function useAdminLoginTotpMutation() {
  return useMutation({
    mutationFn: (payload: AdminLoginTotpPayload) => loginAdminTotp(payload),
  })
}

export function useAdminUsersQuery(params: ListAdminUsersParams) {
  return useQuery({
    queryKey: adminQueryKeys.users(params),
    queryFn: () => fetchAdminUsers(params),
  })
}

export function useCreateAdminUserMutation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (payload: CreateAdminUserPayload) => createAdminUser(payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useUpdateAdminUserMutation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, payload }: { id: string; payload: UpdateAdminUserPayload }) =>
      updateAdminUser(id, payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useDeleteAdminUserMutation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) => deleteAdminUser(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useChangeMyPasswordMutation() {
  return useMutation({
    mutationFn: (payload: ChangeMyPasswordPayload) => changeMyPassword(payload),
  })
}

export function useMyTotpStatusQuery() {
  return useQuery({
    queryKey: adminQueryKeys.totpStatus(),
    queryFn: fetchMyTotpStatus,
  })
}

export function useSetupMyTotpMutation() {
  return useMutation({
    mutationFn: setupMyTotp,
  })
}

export function useConfirmMyTotpMutation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (payload: ConfirmTotpPayload) => confirmMyTotp(payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.totpStatus() })
    },
  })
}

export function useDisableMyTotpMutation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (payload: DisableTotpPayload) => disableMyTotp(payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.totpStatus() })
    },
  })
}
