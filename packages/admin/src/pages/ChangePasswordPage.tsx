import { useState } from 'react'
import {
  Alert,
  Button,
  Card,
  CardContent,
  Snackbar,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import { isApiError } from '../http'
import { useChangeMyPasswordMutation } from '../query'

export function ChangePasswordPage() {
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [toast, setToast] = useState(false)
  const mutation = useChangeMyPasswordMutation()

  const errorMessage = mutation.error
    ? isApiError(mutation.error)
      ? mutation.error.message
      : 'Unable to change password.'
    : null

  const handleSubmit = () => {
    mutation.mutate(
      {
        currentPassword,
        newPassword,
      },
      {
        onSuccess: () => {
          setCurrentPassword('')
          setNewPassword('')
          setToast(true)
        },
      },
    )
  }

  return (
    <>
      <Card sx={{ maxWidth: 640 }}>
        <CardContent>
          <Stack spacing={3}>
            <Stack spacing={0.5}>
              <Typography variant='h6' sx={{ fontWeight: 700 }}>
                Change Password
              </Typography>
              <Typography variant='body2' color='text.secondary'>
                Updating your password will invalidate previously issued JWTs.
              </Typography>
            </Stack>

            {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}

            <TextField
              label='Current password'
              type='password'
              value={currentPassword}
              onChange={(event) => setCurrentPassword(event.target.value)}
            />
            <TextField
              label='New password'
              type='password'
              value={newPassword}
              onChange={(event) => setNewPassword(event.target.value)}
              helperText='At least 8 characters.'
            />
            <Button variant='contained' onClick={handleSubmit} disabled={mutation.isPending}>
              {mutation.isPending ? 'Updating...' : 'Update password'}
            </Button>
          </Stack>
        </CardContent>
      </Card>
      <Snackbar open={toast} autoHideDuration={2400} onClose={() => setToast(false)} message='Password updated successfully.' />
    </>
  )
}
