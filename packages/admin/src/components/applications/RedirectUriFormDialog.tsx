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

export interface RedirectUriFormValues {
  redirectUri: string
  isPrimary: boolean
}

interface RedirectUriFormDialogProps {
  open: boolean
  initialValues: RedirectUriFormValues
  submitting: boolean
  onClose: () => void
  onSubmit: (values: RedirectUriFormValues) => void
}

export function RedirectUriFormDialog({
  open,
  initialValues,
  submitting,
  onClose,
  onSubmit,
}: RedirectUriFormDialogProps) {
  const [values, setValues] = useState(initialValues)

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogTitle>Add Redirect URI</DialogTitle>
      <DialogContent sx={{ px: 3, pt: 1.5, pb: 0 }}>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <TextField
            label="Redirect URI"
            value={values.redirectUri}
            onChange={(event) =>
              setValues((prev) => ({
                ...prev,
                redirectUri: event.target.value,
              }))
            }
          />
          <FormControlLabel
            control={
              <Switch
                checked={values.isPrimary}
                onChange={(event) =>
                  setValues((prev) => ({
                    ...prev,
                    isPrimary: event.target.checked,
                  }))
                }
              />
            }
            label="Set as primary"
          />
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 3, pt: 2 }}>
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={() => onSubmit(values)} variant="contained" disabled={submitting}>
          Add
        </Button>
      </DialogActions>
    </Dialog>
  )
}
