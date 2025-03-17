import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/camera'

export const Route = createLazyFileRoute('/_authenticated/settings/camera')({
  component: SettingsProfile,
})
