import { useState, type ComponentProps } from 'react'
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import ShieldRoundedIcon from '@mui/icons-material/ShieldRounded'
import { useNavigate } from 'react-router'
import { isApiError } from '../http'
import { useAdminLoginMutation, useAdminLoginTotpMutation } from '../query'
import { setSession } from '../stores/auth'
import type { AdminLoginPendingTotpResponse } from '../types'

type FormSubmitEvent = Parameters<NonNullable<ComponentProps<'form'>['onSubmit']>>[0]

export function LoginPage() {
  const navigate = useNavigate()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [totpCode, setTotpCode] = useState('')
  const [pendingTotp, setPendingTotp] = useState<AdminLoginPendingTotpResponse | null>(null)

  const loginMutation = useAdminLoginMutation()
  const totpMutation = useAdminLoginTotpMutation()

  const activeError = pendingTotp ? totpMutation.error : loginMutation.error
  const errorMessage = activeError
    ? isApiError(activeError)
      ? activeError.message
      : 'Login failed, please try again.'
    : null

  const isPending = loginMutation.isPending || totpMutation.isPending

  const handleCredentialsSubmit = () => {
    totpMutation.reset()
    loginMutation.mutate(
      { email, password },
      {
        onSuccess: (response) => {
          if (response.status === 'authenticated') {
            setSession(response.accessToken, response.user)
            navigate('/dashboard', { replace: true })
            return
          }

          setPendingTotp(response)
          setTotpCode('')
        },
      },
    )
  }

  const handleTotpSubmit = () => {
    if (!pendingTotp) return

    loginMutation.reset()
    totpMutation.mutate(
      {
        challengeToken: pendingTotp.challengeToken,
        totpCode,
      },
      {
        onSuccess: (response) => {
          setSession(response.accessToken, response.user)
          navigate('/dashboard', { replace: true })
        },
      },
    )
  }

  const handleBackToCredentials = () => {
    setPendingTotp(null)
    setTotpCode('')
    setPassword('')
    loginMutation.reset()
    totpMutation.reset()
  }

  const handleSubmit = (event: FormSubmitEvent) => {
    event.preventDefault()

    if (pendingTotp) {
      handleTotpSubmit()
      return
    }

    handleCredentialsSubmit()
  }

  const helperMessage = pendingTotp
    ? `Enter the 6-digit code from your authenticator app. This challenge expires in ${Math.floor(
        pendingTotp.challengeExpiresIn / 60,
      )} minutes.`
    : 'Sign in with your admin email and password.'

  return (
    <Box
      sx={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        px: 2,
        background:
          'linear-gradient(180deg, rgba(25,118,210,0.10) 0%, rgba(245,247,251,1) 45%)',
      }}
    >
      <Card sx={{ width: '100%', maxWidth: 440, boxShadow: 6 }}>
        <CardContent component='form' onSubmit={handleSubmit} sx={{ p: 4 }}>
          <Stack spacing={3}>
            <Stack spacing={1} sx={{ alignItems: 'center' }}>
              <ShieldRoundedIcon color='primary' sx={{ fontSize: 44 }} />
              <Typography variant='h5' sx={{ fontWeight: 700 }}>
                MAuth Admin
              </Typography>
              <Typography variant='body2' color='text.secondary' sx={{ textAlign: 'center' }}>
                {helperMessage}
              </Typography>
            </Stack>

            {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}

            {pendingTotp ? (
              <>
                <Alert severity='info'>
                  Second factor required for <strong>{pendingTotp.user.email}</strong>.
                </Alert>
                <TextField
                  label='Authenticator code'
                  autoComplete='one-time-code'
                  value={totpCode}
                  onChange={(event) => setTotpCode(event.target.value)}
                  inputMode='numeric'
                  placeholder='123456'
                  fullWidth
                />
                <Stack direction='row' spacing={1.5}>
                  <Button
                    type='button'
                    variant='outlined'
                    size='large'
                    onClick={handleBackToCredentials}
                    disabled={isPending}
                    sx={{ flex: 1 }}
                  >
                    Back
                  </Button>
                  <Button
                    type='submit'
                    variant='contained'
                    size='large'
                    disabled={isPending}
                    startIcon={isPending ? <CircularProgress color='inherit' size={18} /> : null}
                    sx={{ flex: 1.4 }}
                  >
                    {isPending ? 'Verifying...' : 'Verify and sign in'}
                  </Button>
                </Stack>
              </>
            ) : (
              <>
                <TextField
                  label='Email'
                  type='email'
                  autoComplete='email'
                  value={email}
                  onChange={(event) => setEmail(event.target.value)}
                  fullWidth
                />
                <TextField
                  label='Password'
                  type='password'
                  autoComplete='current-password'
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                  fullWidth
                />

                <Button
                  type='submit'
                  variant='contained'
                  size='large'
                  disabled={isPending}
                  startIcon={isPending ? <CircularProgress color='inherit' size={18} /> : null}
                >
                  {isPending ? 'Signing in...' : 'Sign in'}
                </Button>
              </>
            )}
          </Stack>
        </CardContent>
      </Card>
    </Box>
  )
}
