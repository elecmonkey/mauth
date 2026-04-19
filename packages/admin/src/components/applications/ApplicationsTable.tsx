import DeleteRoundedIcon from '@mui/icons-material/DeleteRounded'
import EditRoundedIcon from '@mui/icons-material/EditRounded'
import PauseCircleRoundedIcon from '@mui/icons-material/PauseCircleRounded'
import PlayCircleRoundedIcon from '@mui/icons-material/PlayCircleRounded'
import {
  Box,
  Button,
  Chip,
  Paper,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  Typography,
} from '@mui/material'
import type { ApplicationSummary } from '../../types'

interface ApplicationsTableProps {
  items: ApplicationSummary[]
  selectedId: string | null
  busy: boolean
  onSelect: (item: ApplicationSummary) => void
  onEdit: (item: ApplicationSummary) => void
  onToggleActive: (item: ApplicationSummary) => void
  onDelete: (item: ApplicationSummary) => void
}

export function ApplicationsTable({
  items,
  selectedId,
  busy,
  onSelect,
  onEdit,
  onToggleActive,
  onDelete,
}: ApplicationsTableProps) {
  return (
    <Paper sx={{ overflow: 'hidden' }}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Code</TableCell>
            <TableCell>Type</TableCell>
            <TableCell>User Pool</TableCell>
            <TableCell>Status</TableCell>
            <TableCell align="right">Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {items.map((item) => (
            <TableRow
              key={item.id}
              hover
              selected={item.id === selectedId}
              onClick={() => onSelect(item)}
              sx={{ cursor: 'pointer' }}
            >
              <TableCell>
                <Typography sx={{ fontWeight: 600 }}>{item.name}</Typography>
                <Typography variant="body2" color="text.secondary">
                  {item.clientId}
                </Typography>
              </TableCell>
              <TableCell>{item.code}</TableCell>
              <TableCell>{item.applicationType}</TableCell>
              <TableCell>{item.userPool.name}</TableCell>
              <TableCell>
                <Chip
                  size="small"
                  color={item.isActive ? 'success' : 'default'}
                  label={item.isActive ? 'Active' : 'Disabled'}
                />
              </TableCell>
              <TableCell align="right" onClick={(event) => event.stopPropagation()}>
                <Stack direction="row" spacing={1} sx={{ justifyContent: 'flex-end' }}>
                  <Button size="small" startIcon={<EditRoundedIcon />} onClick={() => onEdit(item)}>
                    Edit
                  </Button>
                  <Button
                    size="small"
                    startIcon={
                      item.isActive ? <PauseCircleRoundedIcon /> : <PlayCircleRoundedIcon />
                    }
                    disabled={busy}
                    onClick={() => onToggleActive(item)}
                  >
                    {item.isActive ? 'Disable' : 'Enable'}
                  </Button>
                  <Button
                    size="small"
                    color="error"
                    startIcon={<DeleteRoundedIcon />}
                    disabled={busy}
                    onClick={() => onDelete(item)}
                  >
                    Delete
                  </Button>
                </Stack>
              </TableCell>
            </TableRow>
          ))}
          {items.length === 0 ? (
            <TableRow>
              <TableCell colSpan={6}>
                <Box sx={{ py: 6, textAlign: 'center', color: 'text.secondary' }}>
                  No applications found.
                </Box>
              </TableCell>
            </TableRow>
          ) : null}
        </TableBody>
      </Table>
    </Paper>
  )
}
