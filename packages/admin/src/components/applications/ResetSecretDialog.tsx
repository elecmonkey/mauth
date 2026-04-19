import { useState } from 'react'
import {
  Alert,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Stack,
  TextField,
} from '@mui/material'

interface ResetSecretDialogProps {
  open: boolean
  submitting: boolean
  revealedSecret: string | null
  onClose: () => void
  onSubmit: (currentAdminPassword: string) => void
}

export function ResetSecretDialog({
  open,
  submitting,
  revealedSecret,
  onClose,
  onSubmit,
}: ResetSecretDialogProps) {
  const [password, setPassword] = useState('')

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogTitle>Reset Application Secret</DialogTitle>
      <DialogContent sx={{ px: 3, pt: 1.5, pb: 0 }}>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <Alert severity="warning">
            Resetting the secret will invalidate the current confidential client secret.
          </Alert>
          {revealedSecret ? (
            <Alert severity="success">
              New secret: <strong>{revealedSecret}</strong>
            </Alert>
          ) : null}
          <TextField
            label="Current Admin Password"
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 3, pt: 2 }}>
        <Button onClick={onClose}>Close</Button>
        <Button onClick={() => onSubmit(password)} variant="contained" disabled={submitting}>
          Reset Secret
        </Button>
      </DialogActions>
    </Dialog>
  )
}
