import { createBrowserRouter, Navigate } from 'react-router'
import { AdminLayout } from '../layout/AdminLayout'
import { DashboardPage } from '../pages/DashboardPage'
import { LoginPage } from '../pages/LoginPage'
import { AdminUsersPage } from '../pages/AdminUsersPage'
import { RequireAuth } from './RequireAuth'

export const router = createBrowserRouter([
  {
    path: '/login',
    element: <LoginPage />,
  },
  {
    path: '/',
    element: (
      <RequireAuth>
        <AdminLayout />
      </RequireAuth>
    ),
    children: [
      { index: true, element: <Navigate to='/dashboard' replace /> },
      { path: 'dashboard', element: <DashboardPage /> },
      { path: 'admin-users', element: <AdminUsersPage /> },
    ],
  },
  {
    path: '*',
    element: <Navigate to='/dashboard' replace />,
  },
])
