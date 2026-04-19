import { createTheme } from '@mui/material/styles'

export const appTheme = createTheme({
  cssVariables: true,
  shape: {
    borderRadius: 14,
  },
  palette: {
    mode: 'light',
    primary: {
      main: '#1f4dd8',
    },
    secondary: {
      main: '#7b61ff',
    },
    background: {
      default: '#f5f7fb',
      paper: '#ffffff',
    },
  },
  typography: {
    fontFamily:
      "Inter, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
  },
  components: {
    MuiButton: {
      defaultProps: {
        disableElevation: true,
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          border: '1px solid rgba(15, 23, 42, 0.08)',
          boxShadow: '0 12px 32px rgba(15, 23, 42, 0.06)',
        },
      },
    },
  },
})
