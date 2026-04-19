import { useState } from 'react'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import ContentCopyRoundedIcon from '@mui/icons-material/ContentCopyRounded'
import KeyRoundedIcon from '@mui/icons-material/KeyRounded'
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
import { ApplicationFormDialog, type ApplicationFormValues } from '../components/applications/ApplicationFormDialog'
import { ApplicationsTable } from '../components/applications/ApplicationsTable'
import { RedirectUriFormDialog, type RedirectUriFormValues } from '../components/applications/RedirectUriFormDialog'
import { ResetSecretDialog } from '../components/applications/ResetSecretDialog'
import { isApiError } from '../http'
import {
  useApplicationDetailQuery,
  useApplicationsQuery,
  useCreateApplicationMutation,
  useCreateRedirectUriMutation,
  useDeleteApplicationMutation,
  useDeleteRedirectUriMutation,
  useDisableApplicationMutation,
  useEnableApplicationMutation,
  useResetApplicationSecretMutation,
  useUpdateApplicationMutation,
  useUpdateRedirectUriMutation,
  useUserPoolsQuery,
} from '../query'
import type { ApplicationSummary, RedirectUri } from '../types'

const emptyApplicationForm: ApplicationFormValues = {
  code: '',
  name: '',
  description: '',
  applicationType: 'confidential',
  userPoolId: '',
  homepageUrl: '',
  redirectUrisText: '',
}

const emptyRedirectUriForm: RedirectUriFormValues = {
  redirectUri: '',
  isPrimary: false,
}

export function ApplicationsPage() {
  const [keyword, setKeyword] = useState('')
  const [selectedId, setSelectedId] = useState<string | null>(null)
  const [toast, setToast] = useState<string | null>(null)
  const [revealedSecret, setRevealedSecret] = useState<string | null>(null)
  const [appDialogOpen, setAppDialogOpen] = useState(false)
  const [redirectUriDialogOpen, setRedirectUriDialogOpen] = useState(false)
  const [resetSecretDialogOpen, setResetSecretDialogOpen] = useState(false)
  const [appMode, setAppMode] = useState<'create' | 'edit'>('create')
  const [editingApp, setEditingApp] = useState<ApplicationSummary | null>(null)

  const userPoolsQuery = useUserPoolsQuery({ page: 1, pageSize: 100, isActive: true })
  const listQuery = useApplicationsQuery({ keyword: keyword || undefined, page: 1, pageSize: 50 })
  const detailQuery = useApplicationDetailQuery(selectedId)
  const createAppMutation = useCreateApplicationMutation()
  const updateAppMutation = useUpdateApplicationMutation()
  const enableAppMutation = useEnableApplicationMutation()
  const disableAppMutation = useDisableApplicationMutation()
  const deleteAppMutation = useDeleteApplicationMutation()
  const resetSecretMutation = useResetApplicationSecretMutation()
  const createRedirectMutation = useCreateRedirectUriMutation()
  const updateRedirectMutation = useUpdateRedirectUriMutation()
  const deleteRedirectMutation = useDeleteRedirectUriMutation()

  const error =
    listQuery.error ??
    detailQuery.error ??
    createAppMutation.error ??
    updateAppMutation.error ??
    enableAppMutation.error ??
    disableAppMutation.error ??
    deleteAppMutation.error ??
    resetSecretMutation.error ??
    createRedirectMutation.error ??
    updateRedirectMutation.error ??
    deleteRedirectMutation.error
  const errorMessage = error ? (isApiError(error) ? error.message : 'Request failed.') : null

  const appInitialValues = editingApp
    ? {
        code: editingApp.code,
        name: editingApp.name,
        description: editingApp.description ?? '',
        applicationType: editingApp.applicationType,
        userPoolId: editingApp.userPool.id,
        homepageUrl: editingApp.homepageUrl ?? '',
        redirectUrisText: '',
      }
    : {
        ...emptyApplicationForm,
        userPoolId: userPoolsQuery.data?.items[0]?.id ?? '',
      }

  const openCreateApp = () => {
    setAppMode('create')
    setEditingApp(null)
    setAppDialogOpen(true)
  }

  const openEditApp = (item: ApplicationSummary) => {
    setAppMode('edit')
    setEditingApp(item)
    setAppDialogOpen(true)
  }

  const handleSubmitApp = (values: ApplicationFormValues) => {
    if (appMode === 'create') {
      const redirectUris = values.redirectUrisText
        .split('\n')
        .map((item) => item.trim())
        .filter(Boolean)
        .map((redirectUri, index) => ({ redirectUri, isPrimary: index === 0 }))

      createAppMutation.mutate(
        {
          code: values.code,
          name: values.name,
          description: values.description.trim() || null,
          applicationType: values.applicationType,
          userPoolId: values.userPoolId,
          homepageUrl: values.homepageUrl.trim() || null,
          redirectUris,
        },
        {
          onSuccess: (created) => {
            setSelectedId(created.id)
            setRevealedSecret(created.clientSecret ?? null)
            setToast('Application created.')
            setAppDialogOpen(false)
          },
        },
      )
      return
    }

    if (!editingApp) return
    updateAppMutation.mutate(
      {
        id: editingApp.id,
        payload: {
          name: values.name,
          description: values.description.trim() || null,
          homepageUrl: values.homepageUrl.trim() || null,
        },
      },
      {
        onSuccess: () => {
          setToast('Application updated.')
          setAppDialogOpen(false)
        },
      },
    )
  }

  const handleToggleApp = (item: ApplicationSummary) => {
    const mutation = item.isActive ? disableAppMutation : enableAppMutation
    mutation.mutate(item.id, {
      onSuccess: () => setToast(`Application ${item.isActive ? 'disabled' : 'enabled'}.`),
    })
  }

  const handleDeleteApp = (item: ApplicationSummary) => {
    if (!window.confirm(`Delete application "${item.name}"?`)) return
    deleteAppMutation.mutate(item.id, {
      onSuccess: () => {
        if (selectedId === item.id) setSelectedId(null)
        setToast('Application deleted.')
      },
    })
  }

  const handleAddRedirectUri = (values: RedirectUriFormValues) => {
    if (!selectedId) return
    createRedirectMutation.mutate(
      {
        applicationId: selectedId,
        payload: values,
      },
      {
        onSuccess: () => {
          setToast('Redirect URI added.')
          setRedirectUriDialogOpen(false)
        },
      },
    )
  }

  const handlePromoteRedirectUri = (item: RedirectUri) => {
    if (!selectedId) return
    updateRedirectMutation.mutate(
      {
        applicationId: selectedId,
        redirectUriId: item.id,
        payload: { isPrimary: true },
      },
      {
        onSuccess: () => setToast('Primary redirect URI updated.'),
      },
    )
  }

  const handleDeleteRedirectUri = (item: RedirectUri) => {
    if (!selectedId) return
    if (!window.confirm(`Delete redirect URI "${item.redirectUri}"?`)) return
    deleteRedirectMutation.mutate(
      {
        applicationId: selectedId,
        redirectUriId: item.id,
      },
      {
        onSuccess: () => setToast('Redirect URI deleted.'),
      },
    )
  }

  const handleResetSecret = (currentAdminPassword: string) => {
    if (!selectedId) return
    resetSecretMutation.mutate(
      {
        id: selectedId,
        payload: { currentAdminPassword },
      },
      {
        onSuccess: (result) => {
          setRevealedSecret(result.clientSecret)
          setToast('Application secret reset.')
        },
      },
    )
  }

  const busy =
    enableAppMutation.isPending || disableAppMutation.isPending || deleteAppMutation.isPending

  return (
    <Stack spacing={3}>
      <Stack direction={{ xs: 'column', md: 'row' }} spacing={2} sx={{ justifyContent: 'space-between' }}>
        <TextField
          label='Search applications'
          value={keyword}
          onChange={(event) => setKeyword(event.target.value)}
          sx={{ minWidth: { xs: '100%', md: 320 } }}
        />
        <Stack direction='row' spacing={1.5}>
          <Button variant='outlined' startIcon={<RefreshRoundedIcon />} onClick={() => listQuery.refetch()}>
            Refresh
          </Button>
          <Button variant='contained' startIcon={<AddRoundedIcon />} onClick={openCreateApp}>
            New Application
          </Button>
        </Stack>
      </Stack>

      {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}
      {revealedSecret ? (
        <Alert
          severity='success'
          action={
            <Button
              color='inherit'
              size='small'
              startIcon={<ContentCopyRoundedIcon />}
              onClick={() => navigator.clipboard.writeText(revealedSecret)}
            >
              Copy
            </Button>
          }
        >
          Revealed client secret: <strong>{revealedSecret}</strong>
        </Alert>
      ) : null}

      <Stack direction={{ xs: 'column', xl: 'row' }} spacing={3} sx={{ alignItems: 'stretch' }}>
        <Box sx={{ flex: 1.2 }}>
          <ApplicationsTable
            items={listQuery.data?.items ?? []}
            selectedId={selectedId}
            busy={busy}
            onSelect={(item) => setSelectedId(item.id)}
            onEdit={openEditApp}
            onToggleActive={handleToggleApp}
            onDelete={handleDeleteApp}
          />
        </Box>

        <Card sx={{ flex: 1, minWidth: 0 }}>
          <CardContent>
            {detailQuery.data ? (
              <Stack spacing={2.5}>
                <Stack spacing={1}>
                  <Typography variant='h6'>{detailQuery.data.name}</Typography>
                  <Stack direction='row' spacing={1} useFlexGap sx={{ flexWrap: 'wrap' }}>
                    <Chip
                      size='small'
                      color={detailQuery.data.isActive ? 'success' : 'default'}
                      label={detailQuery.data.isActive ? 'Active' : 'Disabled'}
                    />
                    <Chip size='small' variant='outlined' label={detailQuery.data.applicationType} />
                  </Stack>
                  <Typography variant='body2' color='text.secondary'>
                    Client ID: {detailQuery.data.clientId}
                  </Typography>
                  <Typography variant='body2' color='text.secondary'>
                    User Pool: {detailQuery.data.userPool.name}
                  </Typography>
                  <Typography variant='body2' color='text.secondary'>
                    {detailQuery.data.description || 'No description.'}
                  </Typography>
                </Stack>

                <Divider />

                <Stack direction='row' sx={{ justifyContent: 'space-between', alignItems: 'center' }}>
                  <Typography variant='subtitle1' sx={{ fontWeight: 700 }}>
                    Redirect URIs
                  </Typography>
                  <Button size='small' startIcon={<AddRoundedIcon />} onClick={() => setRedirectUriDialogOpen(true)}>
                    Add Redirect URI
                  </Button>
                </Stack>
                <List disablePadding>
                  {detailQuery.data.redirectUris.map((item) => (
                    <ListItem
                      key={item.id}
                      divider
                      secondaryAction={
                        <Stack direction='row' spacing={1}>
                          {!item.isPrimary ? (
                            <Button size='small' onClick={() => handlePromoteRedirectUri(item)}>
                              Set Primary
                            </Button>
                          ) : null}
                          <Button size='small' color='error' onClick={() => handleDeleteRedirectUri(item)}>
                            Delete
                          </Button>
                        </Stack>
                      }
                    >
                      <ListItemText
                        primary={item.redirectUri}
                        secondary={item.isPrimary ? 'Primary redirect URI' : 'Secondary redirect URI'}
                      />
                    </ListItem>
                  ))}
                </List>

                <Divider />

                <Stack direction='row' spacing={1.5}>
                  {detailQuery.data.applicationType === 'confidential' ? (
                    <Button
                      variant='outlined'
                      startIcon={<KeyRoundedIcon />}
                      onClick={() => setResetSecretDialogOpen(true)}
                    >
                      Reset Secret
                    </Button>
                  ) : null}
                </Stack>
              </Stack>
            ) : (
              <Box sx={{ py: 6, color: 'text.secondary', textAlign: 'center' }}>
                Select an application to inspect its redirect URIs and security settings.
              </Box>
            )}
          </CardContent>
        </Card>
      </Stack>

      <ApplicationFormDialog
        key={`${appMode}-${editingApp?.id ?? 'new'}`}
        open={appDialogOpen}
        mode={appMode}
        initialValues={appInitialValues}
        userPools={userPoolsQuery.data?.items ?? []}
        submitting={createAppMutation.isPending || updateAppMutation.isPending}
        onClose={() => setAppDialogOpen(false)}
        onSubmit={handleSubmitApp}
      />

      <RedirectUriFormDialog
        key={selectedId ?? 'redirect-uri'}
        open={redirectUriDialogOpen}
        initialValues={emptyRedirectUriForm}
        submitting={createRedirectMutation.isPending}
        onClose={() => setRedirectUriDialogOpen(false)}
        onSubmit={handleAddRedirectUri}
      />

      <ResetSecretDialog
        open={resetSecretDialogOpen}
        submitting={resetSecretMutation.isPending}
        revealedSecret={revealedSecret}
        onClose={() => setResetSecretDialogOpen(false)}
        onSubmit={handleResetSecret}
      />

      <Snackbar open={Boolean(toast)} autoHideDuration={2400} onClose={() => setToast(null)} message={toast} />
    </Stack>
  )
}
