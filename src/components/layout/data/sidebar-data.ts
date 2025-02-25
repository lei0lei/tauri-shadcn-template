import {
  IconLayoutDashboard,
  IconParkingCircle,
  IconCamera,
  IconCircleDottedLetterC,
  IconCircleLetterH,
  IconPrompt,
  IconBrandGoogleAnalytics,
  IconLetterO,
  IconBrandVscode,
  IconGenderThird,
  IconHexagon3d,
  IconLogs,
  IconFileIsr,
  IconWaveSawTool,
  IconSettings,
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
              url: '/tasks',
              icon: IconWaveSawTool,
            },
            {
              title: 'records',
              url: '/tasks',
              icon: IconFileIsr,
            },
            {
              title: 'logs',
              url: '/tasks',
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
              url: '/noder',
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
    //   title: 'Projects',
    //   items: [
    //     {
    //       title: 'Auth',
    //       icon: IconLockAccess,
    //       items: [
    //         {
    //           title: 'Sign In',
    //           url: '/sign-in',
    //         },
    //         {
    //           title: 'Sign In (2 Col)',
    //           url: '/sign-in-2',
    //         },
    //         {
    //           title: 'Sign Up',
    //           url: '/sign-up',
    //         },
    //         {
    //           title: 'Forgot Password',
    //           url: '/forgot-password',
    //         },
    //         {
    //           title: 'OTP',
    //           url: '/otp',
    //         },
    //       ],
    //     },
    //     {
    //       title: 'Errors',
    //       icon: IconBug,
    //       items: [
    //         {
    //           title: 'Unauthorized',
    //           url: '/401',
    //           icon: IconLock,
    //         },
    //         {
    //           title: 'Forbidden',
    //           url: '/403',
    //           icon: IconUserOff,
    //         },
    //         {
    //           title: 'Not Found',
    //           url: '/404',
    //           icon: IconError404,
    //         },
    //         {
    //           title: 'Internal Server Error',
    //           url: '/500',
    //           icon: IconServerOff,
    //         },
    //         {
    //           title: 'Maintenance Error',
    //           url: '/503',
    //           icon: IconBarrierBlock,
    //         },
    //       ],
    //     },
    //   ],
    // },
    {
      title: 'Hardware',
      items: [
        {
          title: 'Cameras',
          icon: IconCamera,
          items: [
            {
              title: 'Virtual',
              url: '/settings',
              icon: IconCircleDottedLetterC,
            },
            {
              title: 'Hikvision',
              url: '/settings/account',
              icon: IconCircleLetterH,
            },
            // {
            //   title: 'Appearance',
            //   url: '/settings/appearance',
            //   icon: IconPalette,
            // },
            // {
            //   title: 'Notifications',
            //   url: '/settings/notifications',
            //   icon: IconNotification,
            // },
            // {
            //   title: 'Display',
            //   url: '/settings/display',
            //   icon: IconBrowserCheck,
            // },
          ],
        },
        {
          title: 'PLC',
          url: '/help-center',
          icon: IconParkingCircle,
        },
        {
          title: 'Others',
          url: '/settings/account',
          icon: IconLetterO,
        },
      ],
    },
  ],
}
