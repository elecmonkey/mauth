import { useState } from 'react'
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
import { useAdminLoginMutation } from '../query'
import { setSession } from '../stores/auth'

export function LoginPage() {
  const navigate = useNavigate()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')

  const mutation = useAdminLoginMutation()

  const errorMessage = mutation.error
    ? isApiError(mutation.error)
      ? mutation.error.message
      : 'Login failed, please try again.'
    : null

  const handleSubmit = () => {
    mutation.mutate(
      { email, password },
      {
        onSuccess: (response) => {
          setSession(response.accessToken, response.user)
          navigate('/dashboard', { replace: true })
        },
      },
    )
  }

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
        <CardContent sx={{ p: 4 }}>
          <Stack spacing={3}>
            <Stack spacing={1} sx={{ alignItems: 'center' }}>
              <ShieldRoundedIcon color='primary' sx={{ fontSize: 44 }} />
              <Typography variant='h5' sx={{ fontWeight: 700 }}>
                MAuth Admin
              </Typography>
              <Typography variant='body2' color='text.secondary' sx={{ textAlign: 'center' }}>
                Sign in with your admin email and password.
              </Typography>
            </Stack>

            {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}

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
              variant='contained'
              size='large'
              onClick={handleSubmit}
              disabled={mutation.isPending}
              startIcon={mutation.isPending ? <CircularProgress color='inherit' size={18} /> : null}
            >
              {mutation.isPending ? 'Signing in...' : 'Sign in'}
            </Button>
          </Stack>
        </CardContent>
      </Card>
    </Box>
  )
}
