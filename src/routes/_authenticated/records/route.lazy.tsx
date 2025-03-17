import { createLazyFileRoute } from '@tanstack/react-router'
import Records from '@/features/records'

export const Route = createLazyFileRoute('/_authenticated/records')({
  component: Records,
})
