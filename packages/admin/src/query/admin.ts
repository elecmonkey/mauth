import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import {
  changeMyPassword,
  confirmMyTotp,
  createAdminUser,
  createApplication,
  createProfileField,
  createRedirectUri,
  createUserPool,
  deleteAdminUser,
  deleteApplication,
  deleteProfileField,
  deleteRedirectUri,
  deleteUserPool,
  disableApplication,
  disableMyTotp,
  disableUserPool,
  enableApplication,
  enableUserPool,
  fetchAdminUsers,
  fetchApplicationDetail,
  fetchApplications,
  fetchMyTotpStatus,
  fetchProfileFields,
  fetchRedirectUris,
  fetchUserPoolDetail,
  fetchUserPools,
  loginAdmin,
  loginAdminTotp,
  resetApplicationSecret,
  setupMyTotp,
  updateAdminUser,
  updateApplication,
  updateProfileField,
  updateRedirectUri,
  updateUserPool,
} from '../api'
import type {
  AdminLoginPayload,
  AdminLoginTotpPayload,
  ChangeMyPasswordPayload,
  ConfirmTotpPayload,
  CreateAdminUserPayload,
  CreateApplicationPayload,
  CreateProfileFieldPayload,
  CreateRedirectUriPayload,
  CreateUserPoolPayload,
  DisableTotpPayload,
  ListAdminUsersParams,
  ListApplicationsParams,
  ListUserPoolsParams,
  ResetApplicationSecretPayload,
  UpdateAdminUserPayload,
  UpdateApplicationPayload,
  UpdateProfileFieldPayload,
  UpdateRedirectUriPayload,
  UpdateUserPoolPayload,
} from '../types'

export const adminQueryKeys = {
  all: ['admin'] as const,
  users: (params: ListAdminUsersParams) => ['admin', 'users', params] as const,
  userPools: (params: ListUserPoolsParams) => ['admin', 'user-pools', params] as const,
  userPoolDetail: (id: string) => ['admin', 'user-pool-detail', id] as const,
  profileFields: (userPoolId: string) => ['admin', 'profile-fields', userPoolId] as const,
  applications: (params: ListApplicationsParams) => ['admin', 'applications', params] as const,
  applicationDetail: (id: string) => ['admin', 'application-detail', id] as const,
  redirectUris: (applicationId: string) => ['admin', 'redirect-uris', applicationId] as const,
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

export function useUserPoolsQuery(params: ListUserPoolsParams) {
  return useQuery({
    queryKey: adminQueryKeys.userPools(params),
    queryFn: () => fetchUserPools(params),
  })
}

export function useUserPoolDetailQuery(userPoolId: string | null) {
  return useQuery({
    queryKey: adminQueryKeys.userPoolDetail(userPoolId ?? 'none'),
    queryFn: () => fetchUserPoolDetail(userPoolId ?? ''),
    enabled: Boolean(userPoolId),
  })
}

export function useProfileFieldsQuery(userPoolId: string | null) {
  return useQuery({
    queryKey: adminQueryKeys.profileFields(userPoolId ?? 'none'),
    queryFn: () => fetchProfileFields(userPoolId ?? ''),
    enabled: Boolean(userPoolId),
  })
}

export function useCreateUserPoolMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (payload: CreateUserPoolPayload) => createUserPool(payload),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useUpdateUserPoolMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ id, payload }: { id: string; payload: UpdateUserPoolPayload }) =>
      updateUserPool(id, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(variables.id) }),
      ])
    },
  })
}

export function useEnableUserPoolMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => enableUserPool(id),
    onSuccess: async (_data, id) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(id) }),
      ])
    },
  })
}

export function useDisableUserPoolMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => disableUserPool(id),
    onSuccess: async (_data, id) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(id) }),
      ])
    },
  })
}

export function useDeleteUserPoolMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => deleteUserPool(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useCreateProfileFieldMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ userPoolId, payload }: { userPoolId: string; payload: CreateProfileFieldPayload }) =>
      createProfileField(userPoolId, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(variables.userPoolId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.profileFields(variables.userPoolId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
      ])
    },
  })
}

export function useUpdateProfileFieldMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      userPoolId,
      fieldId,
      payload,
    }: {
      userPoolId: string
      fieldId: string
      payload: UpdateProfileFieldPayload
    }) => updateProfileField(userPoolId, fieldId, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(variables.userPoolId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.profileFields(variables.userPoolId) }),
      ])
    },
  })
}

export function useDeleteProfileFieldMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ userPoolId, fieldId }: { userPoolId: string; fieldId: string }) =>
      deleteProfileField(userPoolId, fieldId),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.userPoolDetail(variables.userPoolId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.profileFields(variables.userPoolId) }),
      ])
    },
  })
}

export function useApplicationsQuery(params: ListApplicationsParams) {
  return useQuery({
    queryKey: adminQueryKeys.applications(params),
    queryFn: () => fetchApplications(params),
  })
}

export function useApplicationDetailQuery(applicationId: string | null) {
  return useQuery({
    queryKey: adminQueryKeys.applicationDetail(applicationId ?? 'none'),
    queryFn: () => fetchApplicationDetail(applicationId ?? ''),
    enabled: Boolean(applicationId),
  })
}

export function useRedirectUrisQuery(applicationId: string | null) {
  return useQuery({
    queryKey: adminQueryKeys.redirectUris(applicationId ?? 'none'),
    queryFn: () => fetchRedirectUris(applicationId ?? ''),
    enabled: Boolean(applicationId),
  })
}

export function useCreateApplicationMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (payload: CreateApplicationPayload) => createApplication(payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useUpdateApplicationMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ id, payload }: { id: string; payload: UpdateApplicationPayload }) =>
      updateApplication(id, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(variables.id) }),
      ])
    },
  })
}

export function useEnableApplicationMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => enableApplication(id),
    onSuccess: async (_data, id) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(id) }),
      ])
    },
  })
}

export function useDisableApplicationMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => disableApplication(id),
    onSuccess: async (_data, id) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(id) }),
      ])
    },
  })
}

export function useDeleteApplicationMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => deleteApplication(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.all })
    },
  })
}

export function useResetApplicationSecretMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ id, payload }: { id: string; payload: ResetApplicationSecretPayload }) =>
      resetApplicationSecret(id, payload),
    onSuccess: async (_data, variables) => {
      await queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(variables.id) })
    },
  })
}

export function useCreateRedirectUriMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      applicationId,
      payload,
    }: {
      applicationId: string
      payload: CreateRedirectUriPayload
    }) => createRedirectUri(applicationId, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(variables.applicationId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.redirectUris(variables.applicationId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.all }),
      ])
    },
  })
}

export function useUpdateRedirectUriMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({
      applicationId,
      redirectUriId,
      payload,
    }: {
      applicationId: string
      redirectUriId: string
      payload: UpdateRedirectUriPayload
    }) => updateRedirectUri(applicationId, redirectUriId, payload),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(variables.applicationId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.redirectUris(variables.applicationId) }),
      ])
    },
  })
}

export function useDeleteRedirectUriMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ applicationId, redirectUriId }: { applicationId: string; redirectUriId: string }) =>
      deleteRedirectUri(applicationId, redirectUriId),
    onSuccess: async (_data, variables) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.applicationDetail(variables.applicationId) }),
        queryClient.invalidateQueries({ queryKey: adminQueryKeys.redirectUris(variables.applicationId) }),
      ])
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
