import { createLazyFileRoute } from '@tanstack/react-router'
import Noder from '@/features/node'

export const Route = createLazyFileRoute('/_authenticated/node/')({
  component: Noder,
})
