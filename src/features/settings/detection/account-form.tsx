import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { toast } from '@/hooks/use-toast'
import { Button } from '@/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from '@/components/ui/form'
import { Switch } from '@/components/ui/switch'
import { Input } from '@/components/ui/input'

import { Command } from '@tauri-apps/plugin-shell'
import { ScrollArea } from "@/components/ui/scroll-area"

import  { useState, useRef, useEffect } from 'react'
import type { Child } from '@tauri-apps/plugin-shell'
const accountFormSchema = z.object({

  defectDetection: z.boolean().default(false), // 添加 toggle 字段
  datasetPath: z.string().nonempty({ message: '请输入数据集路径' }),
  epoches: z.coerce.number().int().min(1),
  batches: z.coerce.number().int().min(1),
  workers: z.coerce.number().int().min(0),
})

type AccountFormValues = z.infer<typeof accountFormSchema>

// This can come from your database or API.
const defaultValues: Partial<AccountFormValues> = {
  defectDetection: false,
  datasetPath: '',
  epoches: 1000,
  batches: 4,
  workers: 2,
}

export function AccountForm() {

  const [logs, setLogs] = useState<string[]>([])
  const scrollRef = useRef<HTMLDivElement>(null)
  const childRef = useRef<Child | null>(null)



  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
    }
  }, [logs])

  const form = useForm<AccountFormValues>({
    resolver: zodResolver(accountFormSchema),
    defaultValues,
  })

  function onSubmit(data: AccountFormValues) {
    toast({
      title: 'You submitted the following values:',
      description: (
        <pre className='mt-2 w-[340px] rounded-md bg-slate-950 p-4'>
          <code className='text-white'>{JSON.stringify(data, null, 2)}</code>
        </pre>
      ),
    })
  }

  async function handleTrain() {
    console.log("开始训练")
    const values = form.getValues()
    const datasetPath = values.datasetPath.trim()
    const epochs = values.epoches
    const workers = values.workers
    const batches = values.batches

    const dataPath = `"${datasetPath}\\data.yaml"` // Windows路径加引号

    const activateCmd  = `& "D:\\code\\tauri-shadcn-template\\fastapi\\app\\.venv\\Scripts\\Activate.ps1"`
    const yoloCmd  = `yolo detect train data=${dataPath} model=yolov8s.pt epochs=${epochs} imgsz=640 workers=${workers} batch=${batches} project=D:\\project`
    const fullCmd = `${activateCmd}; ${yoloCmd}`

    setLogs([]) // 清空日志


    try {
      const command = Command.create('exec-sh', ['-Command', fullCmd])

      command.stdout.on('data', (line: string) => {
        setLogs((prev) => [...prev, line])
      })
      command.stderr.on('data', (line: string) => {
        setLogs((prev) => [...prev, line])
      })

      const child = await command.spawn()
      childRef.current = child
    } catch (err) {
      setLogs((prev) => [...prev, `[ERROR] ${String(err)}`])
    }
  }
function handleStop() {
  if (childRef.current) {
    childRef.current.kill() // 这是可以的
    setLogs(prev => [...prev, '[INFO] 已发送终止指令'])
  } else {
    setLogs(prev => [...prev, '[WARN] 没有运行中的任务'])
  }
}
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <FormField
          control={form.control}
          name='defectDetection'
          render={({ field }) => (
            <FormItem className='flex flex-row items-center justify-between rounded-lg border p-4'>
              <div className='space-y-0.5'>
                <FormLabel>启用缺陷检测</FormLabel>
              </div>
              <FormControl>
                <Switch checked={field.value} onCheckedChange={field.onChange} className='data-[state=checked]:bg-green-700 bg-gray-500'/>
              </FormControl>
            </FormItem>
          )}
        />

        {/* 训练区域 */}
        <div className='border p-4 rounded-lg space-y-4'>
          <h3 className='text-sm font-medium text-gray-700'>训练</h3>

          <FormField
            control={form.control}
            name='datasetPath'
            render={({ field }) => (
              <FormItem className='flex items-center space-x-4'>
                <FormLabel className='w-24 text-left'>数据集路径</FormLabel>
                <FormControl>
                  <Input placeholder='例如 /data/dataset' {...field} />
                </FormControl>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name='epoches'
            render={({ field }) => (
              <FormItem className='flex items-center space-x-4'>
                <FormLabel className='w-24 text-left'>epoches</FormLabel>
                <FormControl>
                  <Input type='number' placeholder='1000' {...field} />
                </FormControl>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name='batches'
            render={({ field }) => (
              <FormItem className='flex items-center space-x-4'>
                <FormLabel className='w-24 text-left'>batches</FormLabel>
                <FormControl>
                  <Input type='number' placeholder='4' {...field} />
                </FormControl>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name='workers'
            render={({ field }) => (
              <FormItem className='flex items-center space-x-4'>
                <FormLabel className='w-24 text-left'>workers</FormLabel>
                <FormControl>
                  <Input type='number' placeholder='2' {...field} />
                </FormControl>
              </FormItem>
            )}
          />

          <div className="flex space-x-4">
            <Button type="button" onClick={handleTrain}>
              开始训练
            </Button>
            <Button type="button" onClick={handleStop} variant="destructive">
              停止训练
            </Button>
          </div>
        </div>
        <div className="border rounded p-2 h-64 bg-black text-green-400 font-mono text-sm w-[calc(100vw-600px)]">
          <ScrollArea className="h-full" ref={scrollRef}>
            {logs.map((line, idx) => (
              <div key={idx} className="whitespace-pre-wrap break-words">
                {line}
              </div>
            ))}
          </ScrollArea>
        </div>

        <Button type='submit'>更新设置</Button>
      </form>
    </Form>
  )
}
