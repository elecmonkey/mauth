import { useMemo, useState } from 'react'
import {
  Alert,
  Box,
  Button,
  Chip,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import { isApiError } from '../../http'
import {
  useConfirmMyTotpMutation,
  useDisableMyTotpMutation,
  useMyTotpStatusQuery,
  useSetupMyTotpMutation,
} from '../../query'
import { setStoredAdminUser } from '../../stores/auth'
import type { TotpSetupResponse } from '../../types'

interface SecurityDialogProps {
  open: boolean
  onClose: () => void
}

export function SecurityDialog({ open, onClose }: SecurityDialogProps) {
  const [setupData, setSetupData] = useState<TotpSetupResponse | null>(null)
  const [confirmCode, setConfirmCode] = useState('')
  const [disableCode, setDisableCode] = useState('')
  const [currentPassword, setCurrentPassword] = useState('')

  const statusQuery = useMyTotpStatusQuery()
  const setupMutation = useSetupMyTotpMutation()
  const confirmMutation = useConfirmMyTotpMutation()
  const disableMutation = useDisableMyTotpMutation()

  const enabled = statusQuery.data?.enabled ?? false
  const isBusy = setupMutation.isPending || confirmMutation.isPending || disableMutation.isPending

  const setupErrorMessage = setupMutation.error
    ? isApiError(setupMutation.error)
      ? setupMutation.error.message
      : 'Unable to prepare two-factor authentication.'
    : null

  const confirmErrorMessage = confirmMutation.error
    ? isApiError(confirmMutation.error)
      ? confirmMutation.error.message
      : 'Unable to enable two-factor authentication.'
    : null

  const disableErrorMessage = disableMutation.error
    ? isApiError(disableMutation.error)
      ? disableMutation.error.message
      : 'Unable to disable two-factor authentication.'
    : null

  const qrMarkup = useMemo(
    () => (setupData ? { __html: setupData.qrSvg } : undefined),
    [setupData],
  )

  const handleClose = () => {
    if (isBusy) return

    setSetupData(null)
    setConfirmCode('')
    setDisableCode('')
    setCurrentPassword('')
    setupMutation.reset()
    confirmMutation.reset()
    disableMutation.reset()
    onClose()
  }

  const handleStartSetup = () => {
    confirmMutation.reset()
    setupMutation.mutate(undefined, {
      onSuccess: (response) => {
        setSetupData(response)
        setConfirmCode('')
      },
    })
  }

  const handleConfirmSetup = () => {
    if (!setupData) return

    setupMutation.reset()
    confirmMutation.mutate(
      { totpCode: confirmCode },
      {
        onSuccess: (response) => {
          setStoredAdminUser(response.user)
          setSetupData(null)
          setConfirmCode('')
        },
      },
    )
  }

  const handleDisable = () => {
    disableMutation.mutate(
      {
        currentPassword,
        totpCode: disableCode,
      },
      {
        onSuccess: (response) => {
          setStoredAdminUser(response.user)
          setCurrentPassword('')
          setDisableCode('')
          setSetupData(null)
        },
      },
    )
  }

  return (
    <Dialog open={open} onClose={handleClose} fullWidth maxWidth='md'>
      <DialogTitle>Security</DialogTitle>
      <DialogContent dividers sx={{ p: 3 }}>
        <Stack spacing={3}>
          <Stack
            direction={{ xs: 'column', sm: 'row' }}
            spacing={1.5}
            sx={{ alignItems: { xs: 'flex-start', sm: 'center' }, justifyContent: 'space-between' }}
          >
            <Box>
              <Typography variant='h6' sx={{ fontWeight: 700 }}>
                Two-Factor Authentication
              </Typography>
              <Typography variant='body2' color='text.secondary'>
                Protect your admin account with a 6-digit code from an authenticator app.
              </Typography>
            </Box>
            <Chip
              color={enabled ? 'success' : 'default'}
              label={enabled ? 'Enabled' : 'Not enabled'}
              variant={enabled ? 'filled' : 'outlined'}
            />
          </Stack>

          {statusQuery.isLoading ? (
            <Stack direction='row' spacing={1.5} sx={{ alignItems: 'center' }}>
              <CircularProgress size={20} />
              <Typography variant='body2' color='text.secondary'>
                Loading security settings...
              </Typography>
            </Stack>
          ) : null}

          {statusQuery.error ? (
            <Alert severity='error'>
              {isApiError(statusQuery.error)
                ? statusQuery.error.message
                : 'Unable to load security settings.'}
            </Alert>
          ) : null}

          {!statusQuery.isLoading ? (
            enabled ? (
              <Alert severity='success'>
                Two-factor authentication is active for this administrator account.
              </Alert>
            ) : (
              <Alert severity='info'>
                Once enabled, signing in will require both your password and a code from your
                authenticator app.
              </Alert>
            )
          ) : null}

          {!enabled && !statusQuery.isLoading ? (
            <Stack direction='row' spacing={1.5}>
              <Button
                variant='contained'
                onClick={handleStartSetup}
                disabled={setupMutation.isPending}
                startIcon={
                  setupMutation.isPending ? <CircularProgress color='inherit' size={18} /> : null
                }
              >
                {setupMutation.isPending ? 'Preparing...' : 'Set up authenticator'}
              </Button>
              {setupData ? (
                <Button
                  variant='outlined'
                  onClick={() => setSetupData(null)}
                  disabled={confirmMutation.isPending}
                >
                  Hide setup
                </Button>
              ) : null}
            </Stack>
          ) : null}

          {setupErrorMessage ? <Alert severity='error'>{setupErrorMessage}</Alert> : null}

          {!enabled && setupData ? (
            <Stack spacing={3} sx={{ borderTop: '1px solid', borderColor: 'divider', pt: 3 }}>
              <Box>
                <Typography variant='subtitle1' sx={{ fontWeight: 700, mb: 0.5 }}>
                  Scan QR Code
                </Typography>
                <Typography variant='body2' color='text.secondary'>
                  Scan this QR code with Google Authenticator, 1Password, or another TOTP-compatible
                  app, then enter the 6-digit code below.
                </Typography>
              </Box>

              <Box
                sx={{
                  display: 'flex',
                  justifyContent: 'center',
                  border: '1px solid',
                  borderColor: 'divider',
                  bgcolor: 'background.paper',
                  p: 2,
                  '& svg': {
                    width: 220,
                    height: 220,
                    display: 'block',
                  },
                }}
                dangerouslySetInnerHTML={qrMarkup}
              />

              <TextField
                label='Setup secret'
                value={setupData.secret}
                fullWidth
                slotProps={{ htmlInput: { readOnly: true } }}
                helperText='Use this secret if your authenticator app cannot scan the QR code.'
              />

              <TextField
                label='otpauth URI'
                value={setupData.otpauthUri}
                fullWidth
                multiline
                minRows={2}
                slotProps={{ htmlInput: { readOnly: true } }}
              />

              {confirmErrorMessage ? <Alert severity='error'>{confirmErrorMessage}</Alert> : null}

              <Stack spacing={2}>
                <TextField
                  label='Authenticator code'
                  autoComplete='one-time-code'
                  value={confirmCode}
                  onChange={(event) => setConfirmCode(event.target.value)}
                  inputMode='numeric'
                  placeholder='123456'
                  helperText='Enter the current 6-digit code to finish enabling 2FA.'
                />
                <Button
                  variant='contained'
                  onClick={handleConfirmSetup}
                  disabled={confirmMutation.isPending}
                  startIcon={
                    confirmMutation.isPending ? <CircularProgress color='inherit' size={18} /> : null
                  }
                  sx={{ alignSelf: 'flex-start', px: 3 }}
                >
                  {confirmMutation.isPending ? 'Enabling...' : 'Enable two-factor authentication'}
                </Button>
              </Stack>
            </Stack>
          ) : null}

          {enabled && !statusQuery.isLoading ? (
            <Stack spacing={3} sx={{ borderTop: '1px solid', borderColor: 'divider', pt: 3 }}>
              <Box>
                <Typography variant='subtitle1' sx={{ fontWeight: 700, mb: 0.5 }}>
                  Disable Two-Factor Authentication
                </Typography>
                <Typography variant='body2' color='text.secondary'>
                  To disable 2FA, confirm your current password and provide a valid 6-digit
                  authenticator code.
                </Typography>
              </Box>

              {disableErrorMessage ? <Alert severity='error'>{disableErrorMessage}</Alert> : null}

              <TextField
                label='Current password'
                type='password'
                autoComplete='current-password'
                value={currentPassword}
                onChange={(event) => setCurrentPassword(event.target.value)}
              />
              <TextField
                label='Authenticator code'
                autoComplete='one-time-code'
                value={disableCode}
                onChange={(event) => setDisableCode(event.target.value)}
                inputMode='numeric'
                placeholder='123456'
              />
              <Button
                variant='outlined'
                color='error'
                onClick={handleDisable}
                disabled={disableMutation.isPending}
                startIcon={
                  disableMutation.isPending ? <CircularProgress color='inherit' size={18} /> : null
                }
                sx={{ alignSelf: 'flex-start', px: 3 }}
              >
                {disableMutation.isPending ? 'Disabling...' : 'Disable 2FA'}
              </Button>
            </Stack>
          ) : null}
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, py: 2 }}>
        <Button onClick={handleClose} disabled={isBusy}>
          Close
        </Button>
      </DialogActions>
    </Dialog>
  )
}
