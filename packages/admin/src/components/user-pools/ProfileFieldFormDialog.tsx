import { useState } from 'react'
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControlLabel,
  MenuItem,
  Stack,
  Switch,
  TextField,
} from '@mui/material'

export interface ProfileFieldFormValues {
  fieldKey: string
  fieldName: string
  fieldType: string
  isRequired: boolean
  isUnique: boolean
  isSearchable: boolean
  sortOrder: number
  optionsText: string
}

interface ProfileFieldFormDialogProps {
  open: boolean
  mode: 'create' | 'edit'
  initialValues: ProfileFieldFormValues
  submitting: boolean
  onClose: () => void
  onSubmit: (values: ProfileFieldFormValues) => void
}

const fieldTypes = [
  'string',
  'text',
  'number',
  'boolean',
  'date',
  'datetime',
  'select',
  'multi_select',
]

export function ProfileFieldFormDialog({
  open,
  mode,
  initialValues,
  submitting,
  onClose,
  onSubmit,
}: ProfileFieldFormDialogProps) {
  const [values, setValues] = useState(initialValues)
  const needsOptions = values.fieldType === 'select' || values.fieldType === 'multi_select'

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogTitle>{mode === 'create' ? 'Create Profile Field' : 'Edit Profile Field'}</DialogTitle>
      <DialogContent sx={{ px: 3, pt: 1.5, pb: 0 }}>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <TextField
            label="Field Key"
            disabled={mode === 'edit'}
            value={values.fieldKey}
            onChange={(event) => setValues((prev) => ({ ...prev, fieldKey: event.target.value }))}
            helperText="Stable field key. Lowercase letters, numbers, and underscores only."
          />
          <TextField
            label="Field Name"
            value={values.fieldName}
            onChange={(event) => setValues((prev) => ({ ...prev, fieldName: event.target.value }))}
          />
          <TextField
            select
            label="Field Type"
            disabled={mode === 'edit'}
            value={values.fieldType}
            onChange={(event) => setValues((prev) => ({ ...prev, fieldType: event.target.value }))}
          >
            {fieldTypes.map((type) => (
              <MenuItem key={type} value={type}>
                {type}
              </MenuItem>
            ))}
          </TextField>
          <TextField
            label="Sort Order"
            type="number"
            value={values.sortOrder}
            onChange={(event) =>
              setValues((prev) => ({
                ...prev,
                sortOrder: Number(event.target.value) || 0,
              }))
            }
          />
          {needsOptions ? (
            <TextField
              label="Options"
              multiline
              minRows={3}
              value={values.optionsText}
              onChange={(event) =>
                setValues((prev) => ({
                  ...prev,
                  optionsText: event.target.value,
                }))
              }
              helperText="One option per line."
            />
          ) : null}
          <FormControlLabel
            control={
              <Switch
                checked={values.isRequired}
                onChange={(event) =>
                  setValues((prev) => ({
                    ...prev,
                    isRequired: event.target.checked,
                  }))
                }
              />
            }
            label="Required"
          />
          <FormControlLabel
            control={
              <Switch
                checked={values.isSearchable}
                onChange={(event) =>
                  setValues((prev) => ({
                    ...prev,
                    isSearchable: event.target.checked,
                  }))
                }
              />
            }
            label="Searchable"
          />
          <FormControlLabel
            control={
              <Switch
                checked={values.isUnique}
                disabled={mode === 'edit'}
                onChange={(event) =>
                  setValues((prev) => ({
                    ...prev,
                    isUnique: event.target.checked,
                  }))
                }
              />
            }
            label="Unique"
          />
        </Stack>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 3, pt: 2 }}>
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={() => onSubmit(values)} variant="contained" disabled={submitting}>
          {mode === 'create' ? 'Create' : 'Save'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
