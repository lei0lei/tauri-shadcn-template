import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/detection'

export const Route = createLazyFileRoute('/_authenticated/settings/detection')({
  component: SettingsProfile,
})
