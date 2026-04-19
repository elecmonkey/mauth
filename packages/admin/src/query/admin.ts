import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import {
  changeMyPassword,
  createAdminUser,
  deleteAdminUser,
  fetchAdminUsers,
  loginAdmin,
  updateAdminUser,
} from '../api'
import type {
  AdminLoginPayload,
  ChangeMyPasswordPayload,
  CreateAdminUserPayload,
  ListAdminUsersParams,
  UpdateAdminUserPayload,
} from '../types/index'

export const adminQueryKeys = {
  all: ['admin'] as const,
  users: (params: ListAdminUsersParams) => ['admin', 'users', params] as const,
}

export function useAdminLoginMutation() {
  return useMutation({
    mutationFn: (payload: AdminLoginPayload) => loginAdmin(payload),
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
