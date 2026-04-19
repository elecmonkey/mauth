import DeleteRoundedIcon from '@mui/icons-material/DeleteRounded'
import EditRoundedIcon from '@mui/icons-material/EditRounded'
import {
  Box,
  Button,
  Paper,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
  Typography,
} from '@mui/material'
import type { AdminUser } from '../../types'

interface AdminUsersTableProps {
  items: AdminUser[]
  currentUserId?: string
  deleting: boolean
  onEdit: (user: AdminUser) => void
  onDelete: (id: string) => void
}

export function AdminUsersTable({
  items,
  currentUserId,
  deleting,
  onEdit,
  onDelete,
}: AdminUsersTableProps) {
  return (
    <Paper sx={{ overflow: 'hidden' }}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Nickname</TableCell>
            <TableCell>Email</TableCell>
            <TableCell>Status</TableCell>
            <TableCell>2FA</TableCell>
            <TableCell>Last Login</TableCell>
            <TableCell align="right">Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {items.map((user) => (
            <TableRow key={user.id} hover>
              <TableCell>
                <Typography sx={{ fontWeight: 600 }}>{user.nickname}</Typography>
              </TableCell>
              <TableCell>{user.email}</TableCell>
              <TableCell>{user.isActive ? 'Active' : 'Disabled'}</TableCell>
              <TableCell>{user.totpEnabled ? 'Enabled' : 'Disabled'}</TableCell>
              <TableCell>
                {user.lastLoginAt ? new Date(user.lastLoginAt).toLocaleString() : '-'}
              </TableCell>
              <TableCell align="right">
                <Stack direction="row" spacing={1} sx={{ justifyContent: 'flex-end' }}>
                  <Button
                    size="small"
                    startIcon={<EditRoundedIcon />}
                    onClick={() => onEdit(user)}
                    sx={{ px: 1.5, py: 0.625 }}
                  >
                    Edit
                  </Button>
                  <Button
                    size="small"
                    color="error"
                    startIcon={<DeleteRoundedIcon />}
                    disabled={currentUserId === user.id || deleting}
                    onClick={() => onDelete(user.id)}
                    sx={{ px: 1.5, py: 0.625 }}
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
                  No admin users found.
                </Box>
              </TableCell>
            </TableRow>
          ) : null}
        </TableBody>
      </Table>
    </Paper>
  )
}
