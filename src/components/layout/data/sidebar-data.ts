import {
  IconLayoutDashboard,
  IconParkingCircle,
  IconCamera,
  IconCircleDottedLetterC,
  IconCircleLetterH,
  IconPrompt,
  IconBrandGoogleAnalytics,
  IconBrandVscode,
  IconGenderThird,
  IconHexagon3d,
  IconLogs,
  IconFileIsr,
  IconWaveSawTool,
  IconSettings,
  IconCircle,
} from '@tabler/icons-react'
import { Command } from 'lucide-react'
import { type SidebarData } from '../types'

export const sidebarData: SidebarData = {
  user: {
    name: 'Meng Fanlei',
    email: 'lei.lei.fan.meng@gmail.com',
    avatar: '/avatars/shadcn.jpg',
  },
  teams: [
    {
      name: 'Sengo',
      logo: Command,
      plan: 'Vite + ShadcnUI',
    },
  ],
  navGroups: [
    {
      title: 'General',
      items: [
        {
          title: 'Dashboard',
          url: '/',
          icon: IconLayoutDashboard,
        },
        {
          title: 'Analytics',
          icon: IconBrandGoogleAnalytics,
          items: [
            {
              title: 'analytics',
              url: '/analytics',
              icon: IconWaveSawTool,
            },
            {
              title: 'records',
              url: '/records',
              icon: IconFileIsr,
            },
            {
              title: 'logs',
              url: '/logs',
              icon: IconLogs,
            },


          ]
        },
        {
          title: 'Settings',
          url: '/settings',
          icon: IconSettings,
        },
        {
          title: 'Project',
          icon: IconPrompt,
          items: [
            {
              title: 'node-editor',
              url: '/node',
              icon: IconGenderThird,
            },
            {
              title: 'project-editor',
              url: '/project',
              icon: IconHexagon3d,
            },
            {
              title: 'algo-editor',
              url: '/monaco',
              icon: IconBrandVscode,
            },

          ]
        },
      ],
    },
    // {
    //   title: 'Hardware',
    //   items: [
    //     {
    //       title: 'Cameras',
    //       icon: IconCamera,
    //       items: [
    //         {
    //           title: 'Virtual',
    //           url: '/help-center',
    //           icon: IconCircleDottedLetterC,
    //         },
    //         {
    //           title: 'Hikvision',
    //           url: '/help-center',
    //           icon: IconCircleLetterH,
    //         },
    //       ],
    //     },
    //     {
    //       title: 'PLC',
    //       url: '/help-center',
    //       icon: IconParkingCircle,
    //     },
    //     {
    //       title: 'Others',
    //       url: '/help-center',
    //       icon: IconCircle,
    //     },
    //   ],
    // },
  ],
}
