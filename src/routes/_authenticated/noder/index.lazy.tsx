import { createLazyFileRoute } from '@tanstack/react-router'
import Noder from '@/features/noder'

export const Route = createLazyFileRoute('/_authenticated/noder/')({
  component: Noder,
})
