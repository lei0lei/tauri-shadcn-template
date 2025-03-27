// import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Badge } from "@/components/ui/badge";
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui/tooltip";
import { Tabs, TabsContent} from '@/components/ui/tabs'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
// import { TopNav } from '@/components/layout/top-nav'
// import { ProfileDropdown } from '@/components/profile-dropdown'
// import { Search } from '@/components/search'
import { ThemeSwitch } from '@/components/theme-switch'
import { Overview } from './components/overview'
import  LogWindow  from './components/stateandlogs'
import ResultShow from './components/resultshow'
// import { invoke } from '@tauri-apps/api/core';
// import React, { useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useEffect } from "react";
import StartStopButton from './components/startstop'
import { Separator } from '@radix-ui/react-separator'
import { useDashboardStore } from "@/stores/dashboardStore";
import { IconCheck, IconX, IconCamera, IconParkingCircleFilled, IconAsset, IconServer, IconCircle, IconDeviceFloppy } from "@tabler/icons-react";

const iconMap : Record<string, React.ComponentType>= {
  "相机": IconCamera,
  "PLC": IconParkingCircleFilled,
  "传感器": IconCircle,
  "机器人": IconAsset,
  "算法": IconServer,
  "硬盘": IconDeviceFloppy,
};

export default function Dashboard() {
  // 后端rust调用测试
  // const [response, setResponse] = useState<string>('');

  // const callRustCommand = async () => {
  //   try {
  //     // 调用 Rust 后端的自定义命令
  //     const result = await invoke('test_call_from_frontend');
  //     setResponse(result as string); // 更新响应内容
  //   } catch (error) {
  //     console.error("Error calling Rust command:", error);
  //   }
  // };
  // @ts-ignore
  const { logs, setLogs, artifactType,statics,artifact} = useDashboardStore();

  const initSidecarListeners = async () => {
    // Listen for stdout lines from the sidecar

    const unlistenStdout = await listen('sidecar-stdout', (event) => {
      console.log('Sidecar stdout:', event.payload);
      if (`${event.payload}`.length > 0 && event.payload !== "\r\n")
        useDashboardStore.getState().setLogs(event.payload as string);
    });

    // Listen for stderr lines from the sidecar
    const unlistenStderr = await listen('sidecar-stderr', (event) => {
      console.error('Sidecar stderr:', event.payload);
      if (`${event.payload}`.length > 0 && event.payload !== "\r\n")
        useDashboardStore.getState().setLogs(event.payload as string);
    });

    // Cleanup listeners when not needed
    return () => {
      unlistenStdout();
      unlistenStderr();
    };
  }

  useEffect(() => {
    initSidecarListeners()
  }, [])

  return (
    <div className='flex flex-col min-h-screen p-4'>
      <Header>
        <div className="flex-1 flex justify-center">
          <StartStopButton className="w-full" />
        </div>
        <div className='ml-auto flex items-center space-x-4'>
          <ThemeSwitch />
        </div>
      </Header>


      <Main className="overflow-x-hidden overflow-y-hidden flex-grow">
        <Tabs
          orientation='vertical'
          defaultValue='overview'
          className='h-full'
        >
          <TabsContent value='overview' className='space-y-4 h-full'>
            <div className='grid gap-4 sm:grid-cols-2 lg:grid-cols-4'>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>
                    型号
                  </CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <path d='M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6' />
                  </svg>
                </CardHeader>
                <CardContent>
                  <div className='text-2xl font-bold'>{artifactType}</div>
                  <p className='text-xs text-muted-foreground'>
                    +20.1% from last month
                  </p>
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>
                    统计
                  </CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <path d='M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2' />
                    <circle cx='9' cy='7' r='4' />
                    <path d='M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75' />
                  </svg>
                </CardHeader>
                <CardContent>
                  <div className='text-2xl font-bold'>{statics}</div>
                  <p className='text-xs text-muted-foreground'>
                    +180.1% from last month
                  </p>
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>当前工件</CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <rect width='20' height='14' x='2' y='5' rx='2' />
                    <path d='M2 10h20' />
                  </svg>
                </CardHeader>
                <CardContent>
                  <div className='text-2xl font-bold'>{artifact}</div>
                  <p className='text-xs text-muted-foreground'>
                    +19% from last month
                  </p>
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>
                    系统状态
                  </CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <path d='M22 12h-4l-3 9L9 3l-3 9H2' />
                  </svg>
                </CardHeader>
                <CardContent className="grid grid-cols-3 gap-2">
                <StatusItem label="相机" status={true} />
                <StatusItem label="PLC" status={true} />
                <StatusItem label="传感器" status={false} />
                <StatusItem label="机器人" status={true} />
                <StatusItem label="算法" status={true} />
                <StatusItem label="硬盘" status={true} />
      </CardContent>
              </Card>
            </div>
            <div className='grid grid-cols-1 gap-4 lg:grid-cols-10 flex-grow'>
              <Card className='col-span-1 lg:col-span-8 pt-2 h-full'>
                <CardContent className='px-2 flex justify-center items-center h-full'>
                  <Overview />
                </CardContent>
              </Card>
              <Card className='col-span-1 lg:col-span-2 flex flex-col h-full'>
                <CardContent className="flex flex-col h-full pt-4 px-2 pb-2">
                <div className="flex-grow p-2 bg-gray-100 rounded-lg shadow-lg mb-2" style={{ maxHeight: '220px' }}>
                  <ResultShow />
                </div>
                <Separator />
                <div className="mt-2 flex-grow">
                  <LogWindow />
                  </div>
              </CardContent>
              </Card>
            </div>
          </TabsContent>
        </Tabs>
      </Main>
    </div>
  )
}

function StatusItem({ label, status }: { label: string; status: boolean }) {
  const Icon = iconMap[label];

  return (
    <div className="flex items-center space-x-2">
      {/* Badge 悬停显示 label */}

          <Badge className={status ? "bg-green-700 text-white" : "bg-red-700 text-white"}>
            {status ? <IconCheck className="w-4 h-4 text-white" /> : <IconX className="w-4 h-4 text-white" />}
          </Badge>

      {/* Icon 悬停显示 label */}
      <Tooltip>
        <TooltipTrigger asChild>
          {Icon && <Icon/>}
        </TooltipTrigger>
        <TooltipContent>{label}</TooltipContent>
      </Tooltip>
    </div>
  );
}