import { createLazyFileRoute } from '@tanstack/react-router'
import Monaco from '@/features/monaco'

export const Route = createLazyFileRoute('/_authenticated/monaco/')({
  component: Monaco,
})
