import type { ReactNode } from 'react'
import { Navigate, useLocation } from 'react-router'
import { getAccessToken } from '../stores/auth'

interface RequireAuthProps {
  children: ReactNode
}

export function RequireAuth({ children }: RequireAuthProps) {
  const location = useLocation()
  const token = getAccessToken()

  if (!token) {
    return <Navigate to="/login" replace state={{ from: location.pathname }} />
  }

  return <>{children}</>
}
