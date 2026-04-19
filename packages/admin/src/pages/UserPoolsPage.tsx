import { useState } from 'react'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import RefreshRoundedIcon from '@mui/icons-material/RefreshRounded'
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Chip,
  Divider,
  List,
  ListItem,
  ListItemText,
  Snackbar,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import {
  ProfileFieldFormDialog,
  type ProfileFieldFormValues,
} from '../components/user-pools/ProfileFieldFormDialog'
import {
  UserPoolFormDialog,
  type UserPoolFormValues,
} from '../components/user-pools/UserPoolFormDialog'
import { UserPoolsTable } from '../components/user-pools/UserPoolsTable'
import { isApiError } from '../http'
import {
  useCreateProfileFieldMutation,
  useCreateUserPoolMutation,
  useDeleteProfileFieldMutation,
  useDeleteUserPoolMutation,
  useDisableUserPoolMutation,
  useEnableUserPoolMutation,
  useUpdateProfileFieldMutation,
  useUpdateUserPoolMutation,
  useUserPoolDetailQuery,
  useUserPoolsQuery,
} from '../query'
import type { CreateProfileFieldPayload, ProfileField, UserPoolSummary } from '../types'

const emptyPoolForm: UserPoolFormValues = {
  code: '',
  name: '',
  description: '',
  allowSelfSignup: false,
}

const emptyFieldForm: ProfileFieldFormValues = {
  fieldKey: '',
  fieldName: '',
  fieldType: 'string',
  isRequired: false,
  isUnique: false,
  isSearchable: false,
  sortOrder: 0,
  optionsText: '',
}

function optionsToText(value: unknown) {
  if (!Array.isArray(value)) return ''
  return value.map((item) => String(item)).join('\n')
}

function textToOptions(fieldType: string, optionsText: string) {
  if (fieldType !== 'select' && fieldType !== 'multi_select') return null
  return optionsText
    .split('\n')
    .map((item) => item.trim())
    .filter(Boolean)
}

export function UserPoolsPage() {
  const [keyword, setKeyword] = useState('')
  const [selectedId, setSelectedId] = useState<string | null>(null)
  const [toast, setToast] = useState<string | null>(null)
  const [poolDialogOpen, setPoolDialogOpen] = useState(false)
  const [fieldDialogOpen, setFieldDialogOpen] = useState(false)
  const [poolMode, setPoolMode] = useState<'create' | 'edit'>('create')
  const [fieldMode, setFieldMode] = useState<'create' | 'edit'>('create')
  const [editingPool, setEditingPool] = useState<UserPoolSummary | null>(null)
  const [editingField, setEditingField] = useState<ProfileField | null>(null)

  const listQuery = useUserPoolsQuery({
    keyword: keyword || undefined,
    page: 1,
    pageSize: 50,
  })
  const detailQuery = useUserPoolDetailQuery(selectedId)
  const createPoolMutation = useCreateUserPoolMutation()
  const updatePoolMutation = useUpdateUserPoolMutation()
  const enablePoolMutation = useEnableUserPoolMutation()
  const disablePoolMutation = useDisableUserPoolMutation()
  const deletePoolMutation = useDeleteUserPoolMutation()
  const createFieldMutation = useCreateProfileFieldMutation()
  const updateFieldMutation = useUpdateProfileFieldMutation()
  const deleteFieldMutation = useDeleteProfileFieldMutation()

  const error =
    listQuery.error ??
    detailQuery.error ??
    createPoolMutation.error ??
    updatePoolMutation.error ??
    enablePoolMutation.error ??
    disablePoolMutation.error ??
    deletePoolMutation.error ??
    createFieldMutation.error ??
    updateFieldMutation.error ??
    deleteFieldMutation.error
  const errorMessage = error ? (isApiError(error) ? error.message : 'Request failed.') : null

  const poolInitialValues = editingPool
    ? {
        code: editingPool.code,
        name: editingPool.name,
        description: editingPool.description ?? '',
        allowSelfSignup: editingPool.allowSelfSignup,
      }
    : emptyPoolForm

  const fieldInitialValues = editingField
    ? {
        fieldKey: editingField.fieldKey,
        fieldName: editingField.fieldName,
        fieldType: editingField.fieldType,
        isRequired: editingField.isRequired,
        isUnique: editingField.isUnique,
        isSearchable: editingField.isSearchable,
        sortOrder: editingField.sortOrder,
        optionsText: optionsToText(editingField.options),
      }
    : emptyFieldForm

  const openCreatePool = () => {
    createPoolMutation.reset()
    updatePoolMutation.reset()
    setPoolMode('create')
    setEditingPool(null)
    setPoolDialogOpen(true)
  }

  const openEditPool = (item: UserPoolSummary) => {
    createPoolMutation.reset()
    updatePoolMutation.reset()
    setPoolMode('edit')
    setEditingPool(item)
    setPoolDialogOpen(true)
  }

  const closePoolDialog = () => {
    setPoolDialogOpen(false)
    setEditingPool(null)
  }

  const openCreateField = () => {
    setFieldMode('create')
    setEditingField(null)
    setFieldDialogOpen(true)
  }

  const openEditField = (item: ProfileField) => {
    setFieldMode('edit')
    setEditingField(item)
    setFieldDialogOpen(true)
  }

  const handleSubmitPool = (values: UserPoolFormValues) => {
    if (poolMode === 'create') {
      createPoolMutation.mutate(
        {
          code: values.code,
          name: values.name,
          description: values.description.trim() || null,
          allowSelfSignup: values.allowSelfSignup,
        },
        {
          onSuccess: (created) => {
            setSelectedId(created.id)
            setToast('User pool created.')
            setPoolDialogOpen(false)
          },
          onError: () => {
            setToast('Failed to create user pool.')
          },
        },
      )
      return
    }

    if (!editingPool) return
    updatePoolMutation.mutate(
      {
        id: editingPool.id,
        payload: {
          name: values.name,
          description: values.description.trim() || null,
          allowSelfSignup: values.allowSelfSignup,
        },
      },
      {
        onSuccess: () => {
          setToast('User pool updated.')
          setPoolDialogOpen(false)
        },
        onError: () => {
          setToast('Failed to update user pool.')
        },
      },
    )
  }

  const handleTogglePool = (item: UserPoolSummary) => {
    const mutation = item.isActive ? disablePoolMutation : enablePoolMutation
    mutation.mutate(item.id, {
      onSuccess: () => setToast(`User pool ${item.isActive ? 'disabled' : 'enabled'}.`),
    })
  }

  const handleDeletePool = (item: UserPoolSummary) => {
    if (!window.confirm(`Delete user pool "${item.name}"?`)) return
    deletePoolMutation.mutate(item.id, {
      onSuccess: () => {
        if (selectedId === item.id) setSelectedId(null)
        setToast('User pool deleted.')
      },
    })
  }

  const handleSubmitField = (values: ProfileFieldFormValues) => {
    if (!selectedId) return
    const options = textToOptions(values.fieldType, values.optionsText)

    if (fieldMode === 'create') {
      createFieldMutation.mutate(
        {
          userPoolId: selectedId,
          payload: {
            fieldKey: values.fieldKey,
            fieldName: values.fieldName,
            fieldType: values.fieldType as CreateProfileFieldPayload['fieldType'],
            isRequired: values.isRequired,
            isUnique: values.isUnique,
            isSearchable: values.isSearchable,
            sortOrder: values.sortOrder,
            options,
          },
        },
        {
          onSuccess: () => {
            setToast('Profile field created.')
            setFieldDialogOpen(false)
          },
        },
      )
      return
    }

    if (!editingField) return
    updateFieldMutation.mutate(
      {
        userPoolId: selectedId,
        fieldId: editingField.id,
        payload: {
          fieldName: values.fieldName,
          isRequired: values.isRequired,
          isSearchable: values.isSearchable,
          sortOrder: values.sortOrder,
          options,
        },
      },
      {
        onSuccess: () => {
          setToast('Profile field updated.')
          setFieldDialogOpen(false)
        },
      },
    )
  }

  const handleDeleteField = (field: ProfileField) => {
    if (!selectedId) return
    if (!window.confirm(`Delete profile field "${field.fieldName}"?`)) return
    deleteFieldMutation.mutate(
      { userPoolId: selectedId, fieldId: field.id },
      {
        onSuccess: () => setToast('Profile field deleted.'),
      },
    )
  }

  const busy =
    enablePoolMutation.isPending || disablePoolMutation.isPending || deletePoolMutation.isPending

  return (
    <Stack spacing={3}>
      <Stack
        direction={{ xs: 'column', md: 'row' }}
        spacing={2}
        sx={{ justifyContent: 'space-between' }}
      >
        <TextField
          label="Search user pools"
          value={keyword}
          onChange={(event) => setKeyword(event.target.value)}
          sx={{ minWidth: { xs: '100%', md: 320 } }}
        />
        <Stack direction="row" spacing={1.5}>
          <Button
            variant="outlined"
            startIcon={<RefreshRoundedIcon />}
            onClick={() => listQuery.refetch()}
          >
            Refresh
          </Button>
          <Button variant="contained" startIcon={<AddRoundedIcon />} onClick={openCreatePool}>
            New User Pool
          </Button>
        </Stack>
      </Stack>

      {errorMessage ? <Alert severity="error">{errorMessage}</Alert> : null}

      <Stack direction={{ xs: 'column', xl: 'row' }} spacing={3} sx={{ alignItems: 'stretch' }}>
        <Box sx={{ flex: 1.2 }}>
          <UserPoolsTable
            items={listQuery.data?.items ?? []}
            selectedId={selectedId}
            busy={busy}
            onSelect={(item) => setSelectedId(item.id)}
            onEdit={openEditPool}
            onToggleActive={handleTogglePool}
            onDelete={handleDeletePool}
          />
        </Box>

        <Card sx={{ flex: 1, minWidth: 0 }}>
          <CardContent>
            {detailQuery.data ? (
              <Stack spacing={2.5}>
                <Stack spacing={1}>
                  <Typography variant="h6">{detailQuery.data.name}</Typography>
                  <Stack direction="row" spacing={1} useFlexGap sx={{ flexWrap: 'wrap' }}>
                    <Chip
                      size="small"
                      color={detailQuery.data.isActive ? 'success' : 'default'}
                      label={detailQuery.data.isActive ? 'Active' : 'Disabled'}
                    />
                    <Chip
                      size="small"
                      variant="outlined"
                      label={
                        detailQuery.data.allowSelfSignup
                          ? 'Self Signup Enabled'
                          : 'Self Signup Closed'
                      }
                    />
                  </Stack>
                  <Typography variant="body2" color="text.secondary">
                    Code: {detailQuery.data.code}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {detailQuery.data.description || 'No description.'}
                  </Typography>
                </Stack>

                <Divider />

                <Stack
                  direction="row"
                  sx={{ justifyContent: 'space-between', alignItems: 'center' }}
                >
                  <Typography variant="subtitle1" sx={{ fontWeight: 700 }}>
                    Profile Fields
                  </Typography>
                  <Button size="small" startIcon={<AddRoundedIcon />} onClick={openCreateField}>
                    Add Field
                  </Button>
                </Stack>

                <List disablePadding>
                  {detailQuery.data.profileFields.map((field) => (
                    <ListItem
                      key={field.id}
                      divider
                      secondaryAction={
                        <Stack direction="row" spacing={1}>
                          <Button size="small" onClick={() => openEditField(field)}>
                            Edit
                          </Button>
                          <Button
                            size="small"
                            color="error"
                            onClick={() => handleDeleteField(field)}
                          >
                            Delete
                          </Button>
                        </Stack>
                      }
                    >
                      <ListItemText
                        primary={`${field.fieldName} (${field.fieldKey})`}
                        secondary={`${field.fieldType} · order ${field.sortOrder}${field.isRequired ? ' · required' : ''}${field.isSearchable ? ' · searchable' : ''}${field.isUnique ? ' · unique' : ''}`}
                      />
                    </ListItem>
                  ))}
                  {detailQuery.data.profileFields.length === 0 ? (
                    <Typography variant="body2" color="text.secondary">
                      No profile fields configured yet.
                    </Typography>
                  ) : null}
                </List>

                <Divider />

                <Typography variant="subtitle1" sx={{ fontWeight: 700 }}>
                  Bound Applications
                </Typography>
                <List disablePadding>
                  {detailQuery.data.applications.map((application) => (
                    <ListItem key={application.id} divider>
                      <ListItemText
                        primary={application.name}
                        secondary={`${application.code} · ${application.isActive ? 'Active' : 'Disabled'}`}
                      />
                    </ListItem>
                  ))}
                  {detailQuery.data.applications.length === 0 ? (
                    <Typography variant="body2" color="text.secondary">
                      No applications bound to this user pool.
                    </Typography>
                  ) : null}
                </List>
              </Stack>
            ) : (
              <Box sx={{ py: 6, color: 'text.secondary', textAlign: 'center' }}>
                Select a user pool to inspect and manage its fields.
              </Box>
            )}
          </CardContent>
        </Card>
      </Stack>

      {poolDialogOpen ? (
        <UserPoolFormDialog
          key={`user-pool-${poolMode}-${editingPool?.id ?? 'new'}`}
          open
          mode={poolMode}
          initialValues={poolInitialValues}
          submitting={createPoolMutation.isPending || updatePoolMutation.isPending}
          onClose={closePoolDialog}
          onSubmit={handleSubmitPool}
        />
      ) : null}

      <ProfileFieldFormDialog
        key={`profile-field-${fieldMode}-${editingField?.id ?? 'new'}`}
        open={fieldDialogOpen}
        mode={fieldMode}
        initialValues={fieldInitialValues}
        submitting={createFieldMutation.isPending || updateFieldMutation.isPending}
        onClose={() => setFieldDialogOpen(false)}
        onSubmit={handleSubmitField}
      />

      <Snackbar
        open={Boolean(toast)}
        autoHideDuration={2400}
        onClose={() => setToast(null)}
        message={toast}
      />
    </Stack>
  )
}
