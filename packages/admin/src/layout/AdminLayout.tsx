import { useState, type MouseEvent } from 'react'
import {
  AppBar,
  Avatar,
  Box,
  Button,
  Divider,
  Drawer,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  Stack,
  Toolbar,
  Typography,
} from '@mui/material'
import DashboardRoundedIcon from '@mui/icons-material/DashboardRounded'
import AppsRoundedIcon from '@mui/icons-material/AppsRounded'
import HubRoundedIcon from '@mui/icons-material/HubRounded'
import KeyboardArrowDownRoundedIcon from '@mui/icons-material/KeyboardArrowDownRounded'
import LockResetRoundedIcon from '@mui/icons-material/LockResetRounded'
import LogoutRoundedIcon from '@mui/icons-material/LogoutRounded'
import PeopleAltRoundedIcon from '@mui/icons-material/PeopleAltRounded'
import SecurityRoundedIcon from '@mui/icons-material/SecurityRounded'
import ShieldRoundedIcon from '@mui/icons-material/ShieldRounded'
import { Outlet, useLocation, useNavigate } from 'react-router'
import { ChangePasswordDialog } from '../components/profile/ChangePasswordDialog'
import { SecurityDialog } from '../components/profile/SecurityDialog'
import { clearSession, getStoredAdminUser } from '../stores/auth'

const drawerWidth = 248

const navItems = [
  { label: 'Dashboard', icon: <DashboardRoundedIcon />, path: '/dashboard' },
  { label: 'Admin Users', icon: <PeopleAltRoundedIcon />, path: '/admin-users' },
  { label: 'User Pools', icon: <HubRoundedIcon />, path: '/user-pools' },
  { label: 'Applications', icon: <AppsRoundedIcon />, path: '/applications' },
]

const pageMeta: Record<string, { title: string; description: string }> = {
  '/dashboard': {
    title: 'Dashboard',
    description: 'Manage internal administrators for MAuth.',
  },
  '/admin-users': {
    title: 'Admin Users',
    description: 'Create, edit, and disable backoffice administrators.',
  },
  '/user-pools': {
    title: 'User Pools',
    description: 'Manage user pool boundaries and profile field definitions.',
  },
  '/applications': {
    title: 'Applications',
    description: 'Manage registered applications, redirect URIs, and secrets.',
  },
}

export function AdminLayout() {
  const location = useLocation()
  const navigate = useNavigate()
  const currentUser = getStoredAdminUser()
  const [menuAnchorEl, setMenuAnchorEl] = useState<HTMLElement | null>(null)
  const [passwordDialogOpen, setPasswordDialogOpen] = useState(false)
  const [securityDialogOpen, setSecurityDialogOpen] = useState(false)

  const currentPage = pageMeta[location.pathname] ?? {
    title: 'Admin',
    description: 'Manage internal administrators for MAuth.',
  }
  const menuOpen = Boolean(menuAnchorEl)

  const handleOpenUserMenu = (event: MouseEvent<HTMLElement>) => {
    setMenuAnchorEl(event.currentTarget)
  }

  const handleCloseUserMenu = () => {
    setMenuAnchorEl(null)
  }

  const handleLogout = () => {
    handleCloseUserMenu()
    clearSession()
    navigate('/login', { replace: true })
  }

  const handleOpenSecurity = () => {
    handleCloseUserMenu()
    setSecurityDialogOpen(true)
  }

  const handleOpenChangePassword = () => {
    handleCloseUserMenu()
    setPasswordDialogOpen(true)
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
                sx={{ borderRadius: 1, mb: 0.5 }}
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
                {currentPage.title}
              </Typography>
              <Typography variant='body2' color='text.secondary'>
                {currentPage.description}
              </Typography>
            </Box>
            <Button
              color='inherit'
              onClick={handleOpenUserMenu}
              endIcon={<KeyboardArrowDownRoundedIcon />}
              sx={{
                px: 1.5,
                py: 0.75,
                textTransform: 'none',
                borderRadius: 1,
                minWidth: 0,
              }}
            >
              <Stack direction='row' spacing={1.5} sx={{ alignItems: 'center' }}>
                <Avatar sx={{ width: 34, height: 34, bgcolor: 'primary.main' }}>
                  {(currentUser?.nickname ?? 'A').slice(0, 1).toUpperCase()}
                </Avatar>
                <Box sx={{ textAlign: 'left' }}>
                  <Typography variant='subtitle2'>{currentUser?.nickname ?? 'Admin'}</Typography>
                  <Typography variant='caption' color='text.secondary'>
                    {currentUser?.email ?? 'Unknown'}
                  </Typography>
                </Box>
              </Stack>
            </Button>
            <Menu
              anchorEl={menuAnchorEl}
              open={menuOpen}
              onClose={handleCloseUserMenu}
              anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
              transformOrigin={{ vertical: 'top', horizontal: 'right' }}
            >
              <MenuItem onClick={handleOpenSecurity}>
                <ListItemIcon sx={{ minWidth: 36 }}>
                  <SecurityRoundedIcon fontSize='small' />
                </ListItemIcon>
                <ListItemText primary='Security' secondary='Two-factor authentication' />
              </MenuItem>
              <MenuItem onClick={handleOpenChangePassword}>
                <ListItemIcon sx={{ minWidth: 36 }}>
                  <LockResetRoundedIcon fontSize='small' />
                </ListItemIcon>
                <ListItemText primary='Change Password' />
              </MenuItem>
              <Divider />
              <MenuItem onClick={handleLogout}>
                <ListItemIcon sx={{ minWidth: 36 }}>
                  <LogoutRoundedIcon fontSize='small' />
                </ListItemIcon>
                <ListItemText primary='Sign out' />
              </MenuItem>
            </Menu>
          </Toolbar>
        </AppBar>

        <Box component='main' sx={{ flex: 1, p: 3 }}>
          <Outlet />
        </Box>
      </Box>
      <ChangePasswordDialog
        open={passwordDialogOpen}
        onClose={() => setPasswordDialogOpen(false)}
      />
      <SecurityDialog open={securityDialogOpen} onClose={() => setSecurityDialogOpen(false)} />
    </Box>
  )
}
