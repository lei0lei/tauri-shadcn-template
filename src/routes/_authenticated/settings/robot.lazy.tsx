import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/robot'

export const Route = createLazyFileRoute('/_authenticated/settings/robot')({
  component: SettingsProfile,
})
