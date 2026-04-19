import { useMemo, useState } from 'react'
import { Alert, Button, Snackbar, Stack, TextField } from '@mui/material'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import RefreshRoundedIcon from '@mui/icons-material/RefreshRounded'
import {
  AdminUserFormDialog,
  type AdminUserFormValues,
} from '../components/admin-users/AdminUserFormDialog'
import { AdminUsersTable } from '../components/admin-users/AdminUsersTable'
import { isApiError } from '../http'
import {
  useAdminUsersQuery,
  useCreateAdminUserMutation,
  useDeleteAdminUserMutation,
  useUpdateAdminUserMutation,
} from '../query'
import { getStoredAdminUser } from '../stores/auth'
import type { AdminUser } from '../types/index'

type FormMode = 'create' | 'edit'

const emptyForm: AdminUserFormValues = {
  email: '',
  nickname: '',
  password: '',
  isActive: true,
}

export function AdminUsersPage() {
  const currentUser = getStoredAdminUser()
  const [keyword, setKeyword] = useState('')
  const [toast, setToast] = useState<string | null>(null)
  const [dialogOpen, setDialogOpen] = useState(false)
  const [mode, setMode] = useState<FormMode>('create')
  const [selectedUser, setSelectedUser] = useState<AdminUser | null>(null)
  const listQuery = useAdminUsersQuery({
    keyword: keyword || undefined,
    page: 1,
    pageSize: 50,
  })
  const createMutation = useCreateAdminUserMutation()
  const updateMutation = useUpdateAdminUserMutation()
  const deleteMutation = useDeleteAdminUserMutation()

  const errorMessage = useMemo(() => {
    const error =
      listQuery.error ?? createMutation.error ?? updateMutation.error ?? deleteMutation.error
    if (!error) return null
    if (isApiError(error)) return error.message
    return 'Request failed. Please try again.'
  }, [createMutation.error, deleteMutation.error, listQuery.error, updateMutation.error])

  const openCreateDialog = () => {
    setMode('create')
    setSelectedUser(null)
    setDialogOpen(true)
  }

  const openEditDialog = (user: AdminUser) => {
    setMode('edit')
    setSelectedUser(user)
    setDialogOpen(true)
  }

  const dialogInitialValues: AdminUserFormValues = selectedUser
    ? {
        email: selectedUser.email,
        nickname: selectedUser.nickname,
        password: '',
        isActive: selectedUser.isActive,
      }
    : emptyForm

  const handleSubmit = (values: AdminUserFormValues) => {
    if (mode === 'create') {
      createMutation.mutate(
        {
          email: values.email,
          nickname: values.nickname,
          password: values.password,
          isActive: values.isActive,
        },
        {
          onSuccess: () => {
            setToast('Admin user created.')
            setDialogOpen(false)
          },
        },
      )
      return
    }

    if (selectedUser) {
      updateMutation.mutate(
        {
          id: selectedUser.id,
          payload: {
            nickname: values.nickname,
            isActive: values.isActive,
          },
        },
        {
          onSuccess: () => {
            setToast('Admin user updated.')
            setDialogOpen(false)
          },
        },
      )
    }
  }

  const handleDelete = (id: string) => {
    deleteMutation.mutate(id, {
      onSuccess: () => {
        setToast('Admin user deleted.')
      },
    })
  }

  const isSubmitting = createMutation.isPending || updateMutation.isPending

  return (
    <Stack spacing={3}>
      <Stack
        direction={{ xs: 'column', md: 'row' }}
        spacing={2}
        sx={{ justifyContent: 'space-between' }}
      >
        <TextField
          label='Search by email or nickname'
          value={keyword}
          onChange={(event) => setKeyword(event.target.value)}
          sx={{ minWidth: { xs: '100%', md: 320 } }}
        />
        <Stack direction='row' spacing={1.5}>
          <Button variant='outlined' startIcon={<RefreshRoundedIcon />} onClick={() => listQuery.refetch()}>
            Refresh
          </Button>
          <Button variant='contained' startIcon={<AddRoundedIcon />} onClick={openCreateDialog}>
            New Admin
          </Button>
        </Stack>
      </Stack>

      {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}

      <AdminUsersTable
        items={listQuery.data?.items ?? []}
        currentUserId={currentUser?.id}
        deleting={deleteMutation.isPending}
        onEdit={openEditDialog}
        onDelete={handleDelete}
      />

      <AdminUserFormDialog
        key={`${mode}-${selectedUser?.id ?? 'new'}`}
        open={dialogOpen}
        mode={mode}
        initialValues={dialogInitialValues}
        submitting={isSubmitting}
        onClose={() => setDialogOpen(false)}
        onSubmit={handleSubmit}
      />

      <Snackbar open={Boolean(toast)} autoHideDuration={2400} onClose={() => setToast(null)} message={toast} />
    </Stack>
  )
}
