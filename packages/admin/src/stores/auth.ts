import type { AdminUser } from '../types/index'

const ACCESS_TOKEN_KEY = 'mauth.admin.access-token'
const ADMIN_USER_KEY = 'mauth.admin.user'

export function getAccessToken() {
  return localStorage.getItem(ACCESS_TOKEN_KEY)
}

export function setSession(accessToken: string, user: AdminUser) {
  localStorage.setItem(ACCESS_TOKEN_KEY, accessToken)
  localStorage.setItem(ADMIN_USER_KEY, JSON.stringify(user))
}

export function clearSession() {
  localStorage.removeItem(ACCESS_TOKEN_KEY)
  localStorage.removeItem(ADMIN_USER_KEY)
}

export function getStoredAdminUser(): AdminUser | null {
  const raw = localStorage.getItem(ADMIN_USER_KEY)
  if (!raw) return null

  try {
    return JSON.parse(raw) as AdminUser
  } catch {
    clearSession()
    return null
  }
}
