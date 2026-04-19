import {
  AppBar,
  Avatar,
  Box,
  Drawer,
  IconButton,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Stack,
  Toolbar,
  Tooltip,
  Typography,
} from '@mui/material'
import DashboardRoundedIcon from '@mui/icons-material/DashboardRounded'
import LockResetRoundedIcon from '@mui/icons-material/LockResetRounded'
import LogoutRoundedIcon from '@mui/icons-material/LogoutRounded'
import PeopleAltRoundedIcon from '@mui/icons-material/PeopleAltRounded'
import ShieldRoundedIcon from '@mui/icons-material/ShieldRounded'
import { Outlet, useLocation, useNavigate } from 'react-router'
import { clearSession, getStoredAdminUser } from '../stores/auth'

const drawerWidth = 248

const navItems = [
  { label: 'Dashboard', icon: <DashboardRoundedIcon />, path: '/dashboard' },
  { label: 'Admin Users', icon: <PeopleAltRoundedIcon />, path: '/admin-users' },
  { label: 'Change Password', icon: <LockResetRoundedIcon />, path: '/profile/password' },
]

export function AdminLayout() {
  const location = useLocation()
  const navigate = useNavigate()
  const currentUser = getStoredAdminUser()

  const handleLogout = () => {
    clearSession()
    navigate('/login', { replace: true })
  }

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh', backgroundColor: 'background.default' }}>
      <Drawer
        variant='permanent'
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          '& .MuiDrawer-paper': {
            width: drawerWidth,
            boxSizing: 'border-box',
            borderRightColor: 'divider',
          },
        }}
      >
        <Toolbar>
          <Stack direction='row' spacing={1.5} sx={{ alignItems: 'center' }}>
            <Avatar sx={{ bgcolor: 'primary.main' }}>
              <ShieldRoundedIcon fontSize='small' />
            </Avatar>
            <Box>
              <Typography variant='subtitle1' sx={{ fontWeight: 700 }}>
                MAuth Admin
              </Typography>
              <Typography variant='caption' color='text.secondary'>
                Backoffice Console
              </Typography>
            </Box>
          </Stack>
        </Toolbar>
        <Box sx={{ px: 1.5, pb: 1.5 }}>
          <List disablePadding>
            {navItems.map((item) => (
              <ListItemButton
                key={item.path}
                selected={location.pathname === item.path}
                onClick={() => navigate(item.path)}
                sx={{ borderRadius: 2, mb: 0.5 }}
              >
                <ListItemIcon sx={{ minWidth: 40 }}>{item.icon}</ListItemIcon>
                <ListItemText primary={item.label} />
              </ListItemButton>
            ))}
          </List>
        </Box>
      </Drawer>

      <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        <AppBar
          position='sticky'
          color='inherit'
          elevation={0}
          sx={{ borderBottom: 1, borderColor: 'divider', backgroundColor: 'background.paper' }}
        >
          <Toolbar sx={{ justifyContent: 'space-between' }}>
            <Box>
              <Typography variant='h6' sx={{ fontWeight: 700 }}>
                {navItems.find((item) => item.path === location.pathname)?.label ?? 'Admin'}
              </Typography>
              <Typography variant='body2' color='text.secondary'>
                Manage internal administrators for MAuth.
              </Typography>
            </Box>
            <Stack direction='row' spacing={2} sx={{ alignItems: 'center' }}>
              <Box sx={{ textAlign: 'right' }}>
                <Typography variant='subtitle2'>{currentUser?.nickname ?? 'Admin'}</Typography>
                <Typography variant='caption' color='text.secondary'>
                  {currentUser?.email ?? 'Unknown'}
                </Typography>
              </Box>
              <Tooltip title='Sign out'>
                <IconButton onClick={handleLogout}>
                  <LogoutRoundedIcon />
                </IconButton>
              </Tooltip>
            </Stack>
          </Toolbar>
        </AppBar>

        <Box component='main' sx={{ flex: 1, p: 3 }}>
          <Outlet />
        </Box>
      </Box>
    </Box>
  )
}
