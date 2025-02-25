import { createLazyFileRoute } from '@tanstack/react-router'
import Project from '@/features/project'

export const Route = createLazyFileRoute('/_authenticated/project/')({
  component: Project,
})
