import { useState } from 'react'
import { Button, Dialog, DialogActions, DialogContent, DialogTitle, MenuItem, Stack, TextField } from '@mui/material'
import type { UserPoolSummary } from '../../types'

export interface ApplicationFormValues {
  code: string
  name: string
  description: string
  applicationType: 'confidential' | 'public'
  userPoolId: string
  homepageUrl: string
  redirectUrisText: string
}

interface ApplicationFormDialogProps {
  open: boolean
  mode: 'create' | 'edit'
  initialValues: ApplicationFormValues
  userPools: UserPoolSummary[]
  submitting: boolean
  onClose: () => void
  onSubmit: (values: ApplicationFormValues) => void
}

export function ApplicationFormDialog({
  open,
  mode,
  initialValues,
  userPools,
  submitting,
  onClose,
  onSubmit,
}: ApplicationFormDialogProps) {
  const [values, setValues] = useState(initialValues)

  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth='sm'>
      <DialogTitle>{mode === 'create' ? 'Create Application' : 'Edit Application'}</DialogTitle>
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
          <TextField
            select
            label='Application Type'
            disabled={mode === 'edit'}
            value={values.applicationType}
            onChange={(event) =>
              setValues((prev) => ({
                ...prev,
                applicationType: event.target.value as 'confidential' | 'public',
              }))
            }
          >
            <MenuItem value='confidential'>confidential</MenuItem>
            <MenuItem value='public'>public</MenuItem>
          </TextField>
          <TextField
            select
            label='User Pool'
            disabled={mode === 'edit'}
            value={values.userPoolId}
            onChange={(event) =>
              setValues((prev) => ({ ...prev, userPoolId: event.target.value }))
            }
          >
            {userPools.map((item) => (
              <MenuItem key={item.id} value={item.id}>
                {item.name}
              </MenuItem>
            ))}
          </TextField>
          <TextField
            label='Homepage URL'
            value={values.homepageUrl}
            onChange={(event) =>
              setValues((prev) => ({ ...prev, homepageUrl: event.target.value }))
            }
          />
          {mode === 'create' ? (
            <TextField
              label='Redirect URIs'
              multiline
              minRows={3}
              value={values.redirectUrisText}
              onChange={(event) =>
                setValues((prev) => ({ ...prev, redirectUrisText: event.target.value }))
              }
              helperText='One URI per line. The first URI will be used as primary by default.'
            />
          ) : null}
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
