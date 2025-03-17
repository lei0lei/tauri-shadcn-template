import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/software'

export const Route = createLazyFileRoute('/_authenticated/settings/software')({
  component: SettingsProfile,
})
