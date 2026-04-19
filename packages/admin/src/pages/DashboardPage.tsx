import { Alert, Card, CardContent, Grid, Stack, Typography } from '@mui/material'

const cards = [
  {
    title: 'Admin Auth',
    description: 'Login, bearer JWT validation, and password rotation are wired to the backend.',
  },
  {
    title: 'Admin Users',
    description: 'The console can browse, create, edit, and delete backoffice administrators.',
  },
  {
    title: 'Project Convention',
    description: 'Admin routes use /admin/* and stay isolated from OAuth service flows.',
  },
]

export function DashboardPage() {
  return (
    <Stack spacing={3}>
      <Alert severity="info">
        This is the first-stage admin shell. Next steps can add 2FA, RBAC, refresh tokens, and audit
        logs.
      </Alert>
      <Grid container spacing={2}>
        {cards.map((card) => (
          <Grid size={{ xs: 12, md: 4 }} key={card.title}>
            <Card sx={{ height: '100%' }}>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {card.title}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {card.description}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Stack>
  )
}
