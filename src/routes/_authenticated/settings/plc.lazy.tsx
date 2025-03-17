import { createLazyFileRoute } from '@tanstack/react-router'
import SettingsProfile from '@/features/settings/plc'

export const Route = createLazyFileRoute('/_authenticated/settings/plc')({
  component: SettingsProfile,
})
