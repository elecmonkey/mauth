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

export interface UserPoolFormValues {
  code: string
  name: string
  description: string
  allowSelfSignup: boolean
}

interface UserPoolFormDialogProps {
  open: boolean
  mode: 'create' | 'edit'
  initialValues: UserPoolFormValues
  submitting: boolean
  onClose: () => void
  onSubmit: (values: UserPoolFormValues) => void
}

export function UserPoolFormDialog({
  open,
  mode,
  initialValues,
  submitting,
  onClose,
  onSubmit,
}: UserPoolFormDialogProps) {
  const [values, setValues] = useState(initialValues)

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth='sm'>
      <DialogTitle>{mode === 'create' ? 'Create User Pool' : 'Edit User Pool'}</DialogTitle>
      <DialogContent sx={{ px: 3, pt: 1.5, pb: 0 }}>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <TextField
            label='Code'
            disabled={mode === 'edit'}
            value={values.code}
            onChange={(event) => setValues((prev) => ({ ...prev, code: event.target.value }))}
            helperText='Stable identifier. Lowercase letters, numbers, and dashes only.'
          />
          <TextField
            label='Name'
            value={values.name}
            onChange={(event) => setValues((prev) => ({ ...prev, name: event.target.value }))}
          />
          <TextField
            label='Description'
            value={values.description}
            multiline
            minRows={3}
            onChange={(event) =>
              setValues((prev) => ({ ...prev, description: event.target.value }))
            }
          />
          <FormControlLabel
            control={
              <Switch
                checked={values.allowSelfSignup}
                onChange={(event) =>
                  setValues((prev) => ({ ...prev, allowSelfSignup: event.target.checked }))
                }
              />
            }
            label='Allow self signup'
          />
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 3, pt: 2 }}>
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={() => onSubmit(values)} variant='contained' disabled={submitting}>
          {mode === 'create' ? 'Create' : 'Save'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
