import { Outlet } from '@tanstack/react-router'
import {
  IconAutomation,
  IconSettingsPin,
  IconCamera,
  IconParkingCircle,
  IconCircle,
  IconAsset

} from '@tabler/icons-react'
import { Separator } from '@/components/ui/separator'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
// import { ProfileDropdown } from '@/components/profile-dropdown'
// import { Search } from '@/components/search'
// import { ThemeSwitch } from '@/components/theme-switch'
import SidebarNav from './components/sidebar-nav'

export default function Settings() {
  return (
    <>
      {/* ===== Top Heading ===== */}
      <Header>
        {/* <Search /> */}
        <div className='ml-auto flex items-center space-x-4'>
          {/* <ThemeSwitch />
          <ProfileDropdown /> */}
        </div>
      </Header>

      <Main fixed>
        <div className='space-y-0.5'>
          <h1 className='text-2xl font-bold tracking-tight md:text-3xl'>
            Settings
          </h1>
        </div>
        <Separator className='my-4 lg:my-6' />
        <div className='flex flex-1 flex-col space-y-2 md:space-y-2 overflow-hidden lg:flex-row lg:space-x-12 lg:space-y-0'>
          <aside className='top-0 lg:sticky lg:w-1/5'>
            <SidebarNav items={sidebarNavItems} />
          </aside>
          <div className='flex w-full p-1 pr-4 overflow-y-hidden'>
            <Outlet />
          </div>
        </div>
      </Main>
    </>
  )
}

const sidebarNavItems = [
  {
    title: '软件设置',
    icon: <IconAutomation size={18} />,
    href: '/settings/software',
  },
  {
    title: '检测设置',
    icon: <IconSettingsPin size={18} />,
    href: '/settings/detection',
  },
  {
    title: '相机设置',
    icon: <IconCamera size={18} />,
    href: '/settings/camera',
  },
  {
    title: 'plc设置',
    icon: <IconParkingCircle size={18} />,
    href: '/settings/plc',
  },
  {
    title: '机器人设置',
    icon: <IconAsset size={18} />,
    href: '/settings/robot',
  },
  {
    title: '传感器设置',
    icon: <IconCircle size={18} />,
    href: '/settings/sensor',
  },
]
