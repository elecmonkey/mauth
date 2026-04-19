import { useState } from 'react'
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControlLabel,
  Stack,
  Switch,
  TextField,
} from '@mui/material'

export interface AdminUserFormValues {
  email: string
  nickname: string
  password: string
  isActive: boolean
}

interface AdminUserFormDialogProps {
  open: boolean
  mode: 'create' | 'edit'
  initialValues: AdminUserFormValues
  submitting: boolean
  onClose: () => void
  onSubmit: (values: AdminUserFormValues) => void
}

export function AdminUserFormDialog({
  open,
  mode,
  initialValues,
  submitting,
  onClose,
  onSubmit,
}: AdminUserFormDialogProps) {
  const [values, setValues] = useState(initialValues)

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth='sm'>
      <DialogTitle>{mode === 'create' ? 'Create Admin User' : 'Edit Admin User'}</DialogTitle>
      <DialogContent sx={{ px: 3, pt: 1.5, pb: 0 }}>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <TextField
            label='Email'
            disabled={mode === 'edit'}
            value={values.email}
            onChange={(event) => setValues((prev) => ({ ...prev, email: event.target.value }))}
          />
          <TextField
            label='Nickname'
            value={values.nickname}
            onChange={(event) =>
              setValues((prev) => ({ ...prev, nickname: event.target.value }))
            }
          />
          {mode === 'create' ? (
            <TextField
              label='Password'
              type='password'
              value={values.password}
              onChange={(event) =>
                setValues((prev) => ({ ...prev, password: event.target.value }))
              }
            />
          ) : null}
          <FormControlLabel
            control={
              <Switch
                checked={values.isActive}
                onChange={(event) =>
                  setValues((prev) => ({ ...prev, isActive: event.target.checked }))
                }
              />
            }
            label='Active'
          />
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 3, pt: 2 }}>
        <Button onClick={onClose}>Cancel</Button>
        <Button
          onClick={() => onSubmit(values)}
          variant='contained'
          disabled={submitting}
          sx={{ px: 2.25 }}
        >
          {mode === 'create' ? 'Create' : 'Save'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
