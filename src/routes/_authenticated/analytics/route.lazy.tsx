import { createLazyFileRoute } from '@tanstack/react-router'
import Analytics from '@/features/analytics'

export const Route = createLazyFileRoute('/_authenticated/analytics')({
  component: Analytics,
})
