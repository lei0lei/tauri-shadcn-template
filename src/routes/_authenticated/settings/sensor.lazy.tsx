import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/sensor'

export const Route = createLazyFileRoute('/_authenticated/settings/sensor')({
  component: SettingsProfile,
})
