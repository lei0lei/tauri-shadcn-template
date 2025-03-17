import { Separator } from '@/components/ui/separator'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import '@xyflow/react/dist/style.css';
import { ReactFlow,  Controls, ControlButton, Background,BackgroundVariant  } from '@xyflow/react';


import {IconPencil,
  IconDownload,
  IconPlus
} from '@tabler/icons-react';

const initialNodes = [
  { id: '1', position: { x: 0, y: 0 }, data: { label: '1' } },
  { id: '2', position: { x: 0, y: 100 }, data: { label: '2' } },
];
const initialEdges = [{ id: 'e1-2', source: '1', target: '2' }];



export default function Noder() {
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
        {/* <div className='space-y-0.5'> */}
          <h5 className='font-bold tracking-tight'>
            Node editor
          </h5>
          {/* <p className='text-muted-foreground'>
            Create Node for project.
          </p> */}
        {/* </div> */}
        <Separator className='my-4 lg:my-6' />
        <div className='flex flex-1 flex-col space-y-2 md:space-y-2 overflow-hidden lg:flex-row lg:space-x-12 lg:space-y-0'>
          <div style={{ width: '100vw', height: '100vh'}}>
            <ReactFlow nodes={initialNodes} edges={initialEdges}>
              <Background color="#ccc" variant={BackgroundVariant.Lines} />
              <Controls position={'top-left'} showZoom={false}>
              <ControlButton onClick={() => alert('Edit. ✨')}>
                <IconPencil />
              </ControlButton>
              <ControlButton onClick={() => alert('Something magical just happened. ✨')}>
                <IconDownload />
              </ControlButton>
              <ControlButton onClick={() => alert('Something magical just happened. ✨')}>
                <IconPlus />
              </ControlButton>
            </Controls>
              </ReactFlow>
          </div>
        </div>
      </Main>
    </>
  )
}

// const sidebarNavItems = [
//   {
//     title: 'Profile',
//     icon: <IconUser size={18} />,
//     href: '/settings',
//   },
//   {
//     title: 'Account',
//     icon: <IconTool size={18} />,
//     href: '/settings/account',
//   },
//   {
//     title: 'Appearance',
//     icon: <IconPalette size={18} />,
//     href: '/settings/appearance',
//   },
//   {
//     title: 'Notifications',
//     icon: <IconNotification size={18} />,
//     href: '/settings/notifications',
//   },
//   {
//     title: 'Display',
//     icon: <IconBrowserCheck size={18} />,
//     href: '/settings/display',
//   },
// ]




// export default function Monaco() {
//   return (<div>
//     <Editor height="90vh" defaultLanguage="javascript" defaultValue="// some comment" />
//     </div>);
// }