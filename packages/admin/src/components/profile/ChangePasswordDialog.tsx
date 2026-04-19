import { useState } from 'react'
import {
  Alert,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Snackbar,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import { isApiError } from '../../http'
import { useChangeMyPasswordMutation } from '../../query'

interface ChangePasswordDialogProps {
  open: boolean
  onClose: () => void
}

export function ChangePasswordDialog({ open, onClose }: ChangePasswordDialogProps) {
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [toast, setToast] = useState(false)
  const mutation = useChangeMyPasswordMutation()

  const errorMessage = mutation.error
    ? isApiError(mutation.error)
      ? mutation.error.message
      : 'Unable to change password.'
    : null

  const handleClose = () => {
    setCurrentPassword('')
    setNewPassword('')
    mutation.reset()
    onClose()
  }

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
          mutation.reset()
          setToast(true)
          onClose()
        },
      },
    )
  }

  return (
    <>
      <Dialog open={open} onClose={mutation.isPending ? undefined : handleClose} fullWidth maxWidth='sm'>
        <DialogTitle>Change Password</DialogTitle>
        <DialogContent dividers sx={{ p: 3 }}>
          <Stack spacing={3}>
            <Typography variant='body2' color='text.secondary'>
              Updating your password will invalidate previously issued JWTs.
            </Typography>

            {errorMessage ? <Alert severity='error'>{errorMessage}</Alert> : null}

            <TextField
              label='Current password'
              type='password'
              autoComplete='current-password'
              value={currentPassword}
              onChange={(event) => setCurrentPassword(event.target.value)}
              fullWidth
            />
            <TextField
              label='New password'
              type='password'
              value={newPassword}
              onChange={(event) => setNewPassword(event.target.value)}
              helperText='At least 8 characters.'
              fullWidth
            />
          </Stack>
        </DialogContent>
        <DialogActions sx={{ px: 3, py: 2 }}>
          <Button onClick={handleClose} disabled={mutation.isPending}>
            Cancel
          </Button>
          <Button variant='contained' onClick={handleSubmit} disabled={mutation.isPending}>
            {mutation.isPending ? 'Updating...' : 'Update password'}
          </Button>
        </DialogActions>
      </Dialog>
      <Snackbar
        open={toast}
        autoHideDuration={2400}
        onClose={() => setToast(false)}
        message='Password updated successfully.'
      />
    </>
  )
}
