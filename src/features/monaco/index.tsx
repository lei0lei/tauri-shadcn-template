// import React from 'react';
// import ReactDOM from 'react-dom';

import Editor from '@monaco-editor/react';
// import {FileSidebar} from '@/components/file-sidebar';
// import { Outlet } from '@tanstack/react-router'
// import {
//   IconBrowserCheck,
//   IconNotification,
//   IconPalette,
//   IconTool,
//   IconUser,
// } from '@tabler/icons-react'
import { Separator } from '@/components/ui/separator'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
// import { ProfileDropdown } from '@/components/profile-dropdown'
// import { Search } from '@/components/search'
// import { ThemeSwitch } from '@/components/theme-switch'
// import SidebarNav from './components/sidebar-nav'

export default function Monaco() {
  return (
    <>
      {/* ===== Top Heading ===== */}
      <Header>
        {/* <Search /> */}
        {/* <div className='ml-auto flex items-center space-x-4'> */}
          {/* <ThemeSwitch />
          <ProfileDropdown /> */}
        {/* </div> */}
      </Header>

      <Main fixed>
        {/* <div> */}
          <h1 className='text-2xl font-bold tracking-tight md:text-3xl'>
            Code editor
          </h1>
          {/* <p className='text-muted-foreground'>
            Edit code.
          </p> */}
        {/* </div> */}
        <Separator className='my-4 lg:my-6' />
        <div className='flex flex-1 flex-col space-y-2 md:space-y-2 overflow-hidden lg:flex-row lg:space-x-12 lg:space-y-0'>
          {/* <aside className='top-0 lg:sticky lg:w-1/5'> */}
            {/* <SidebarNav items={sidebarNavItems} /> */}
            <Editor height="90vh"  defaultLanguage="javascript" defaultValue="// some comment" />
          {/* </aside> */}
          {/* <div className='flex w-full p-1 pr-4 overflow-y-hidden'>
            <Outlet />
          </div> */}
        </div>
      </Main>
    </>
  )
}
