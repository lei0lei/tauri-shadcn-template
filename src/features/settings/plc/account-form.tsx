import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { CaretSortIcon, CheckIcon } from '@radix-ui/react-icons'
import { zodResolver } from '@hookform/resolvers/zod'
import { cn } from '@/lib/utils'
import { toast } from '@/hooks/use-toast'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandList,
} from '@/components/ui/command'
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { useState } from "react"
import { invoke } from '@tauri-apps/api/core';



const protocol = [
  { label: 'modbusTCP', value: 'modbus' },
  { label: 'socket', value: 'socket' },
] as const

const accountFormSchema = z.object({
  ip: z
    .string()
    .min(7, { message: '请输入有效 IP 地址' })
    .max(15, { message: 'IP 地址格式错误' }),
  port: z
    .string()
    .min(1, { message: '请输入端口号' })
    .max(5, { message: '端口号格式错误' }),
  protocol: z.string({
    required_error: '请选择协议.',
  }),
  readregisterAddress: z.string().optional(), // 添加寄存器地址字段
  writeregisterAddress: z.string().optional(), // 添加寄存器地址字段
  writeregisterValue: z.string().optional(), // 添加写入值字段
})

type AccountFormValues = z.infer<typeof accountFormSchema>

// This can come from your database or API.
const defaultValues: Partial<AccountFormValues> = {
  ip: '',
  port: '',
  protocol: "",
  readregisterAddress: "",
  writeregisterAddress: "",
  writeregisterValue: "",
}
export function AccountForm() {
  const form = useForm<AccountFormValues>({
    resolver: zodResolver(accountFormSchema),
    defaultValues,
  })

  const [registerValue, setRegisterValue] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  async function readRegister() {
    const address = form.getValues("readregisterAddress")
    if (!address) {
      toast({ title: "错误", description: "请输入寄存器地址" })
      return
    }

    setLoading(true)
    try {
      const value = await invoke<number>("read_register_frontend_plc", { reg_address: parseInt(address) })
      setRegisterValue(`寄存器值: ${value}`)
    } catch (err) {
      setRegisterValue(`读取失败: ${err}`)
    }
    setLoading(false)
  }


  // 写入寄存器
  async function writeRegister() {
    const address = form.getValues("writeregisterAddress")
    const value = form.getValues("writeregisterValue")
    if (!address || !value) {
      toast({ title: "错误", description: "请输入地址和值" })
      return
    }

    setLoading(true)
    try {
      await invoke("write_register_frontend_plc", {
        reg_address: parseInt(address),
        value: parseInt(value),
      })
      toast({ title: "写入成功", description: `地址 ${address} 写入值 ${value}` })
    } catch (err) {
      toast({ title: "写入失败", description: String(err), variant: "destructive" })
    }
    setLoading(false)
  }

  function onSubmit(data: AccountFormValues) {
    toast({
      title: '更新设置',
      description: (
        <pre className='mt-2 w-[340px] rounded-md bg-slate-950 p-4'>
          <code className='text-white'>{JSON.stringify(data, null, 2)}</code>
        </pre>
      ),
    })
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
      <FormItem>
          <FormLabel>连接设置</FormLabel>
          <div className="flex gap-2">
            {/* IP 输入框 */}
            <FormField
              control={form.control}
              name="ip"
              render={({ field }) => (
                <FormControl className="flex-1">
                  <Input placeholder="IP" {...field} />
                </FormControl>
              )}
            />
            {/* PORT 输入框 */}
            <FormField
              control={form.control}
              name="port"
              render={({ field }) => (
                <FormControl className="w-24">
                  <Input placeholder="PORT" {...field} />
                </FormControl>
              )}
            />
          </div>
          <FormMessage />
        </FormItem>
        <FormField
          control={form.control}
          name='protocol'
          render={({ field }) => (
            <FormItem className='flex flex-col'>
              <FormLabel>协议</FormLabel>
              <Popover>
                <PopoverTrigger asChild>
                  <FormControl>
                    <Button
                      variant='outline'
                      role='combobox'
                      className={cn(
                        'w-[200px] justify-between',
                        !field.value && 'text-muted-foreground'
                      )}
                    >
                      {field.value
                        ? protocol.find(
                            (protocol) => protocol.value === field.value
                          )?.label
                        : '选择协议'}
                      <CaretSortIcon className='ml-2 h-4 w-4 shrink-0 opacity-50' />
                    </Button>
                  </FormControl>
                </PopoverTrigger>
                <PopoverContent className='w-[200px] p-0'>
                  <Command>
                    <CommandEmpty>请选择连接协议.</CommandEmpty>
                    <CommandGroup>
                      <CommandList>
                        {protocol.map((protocol) => (
                          <CommandItem
                            value={protocol.label}
                            key={protocol.value}
                            onSelect={() => {
                              form.setValue('protocol', protocol.value)
                            }}
                          >
                            <CheckIcon
                              className={cn(
                                'mr-2 h-4 w-4',
                                protocol.value === field.value
                                  ? 'opacity-100'
                                  : 'opacity-0'
                              )}
                            />
                            {protocol.label}
                          </CommandItem>
                        ))}
                      </CommandList>
                    </CommandGroup>
                  </Command>
                </PopoverContent>
              </Popover>
              <FormDescription>
                选择plc连接协议
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        {/* 读取寄存器 */}
        <FormItem>
          <FormLabel>寄存器地址</FormLabel>
          <div className="flex gap-2">
            <FormField
              control={form.control}
              name="readregisterAddress"
              render={({ field }) => (
                <FormControl className="flex-1">
                  <Input placeholder="输入寄存器地址" {...field} />
                </FormControl>
              )}
            />
            <Button type="button" onClick={readRegister} disabled={loading}>
              {loading ? "读取中..." : "读取寄存器"}
            </Button>
          </div>
          {registerValue && <p className="text-sm text-gray-600">{registerValue}</p>}
          <FormMessage />
        </FormItem>
        {/* 写入寄存器 */}
        <FormItem>
          <FormLabel>写入寄存器</FormLabel>
          <div className="flex gap-2">
            <FormField
              control={form.control}
              name="writeregisterAddress"
              render={({ field }) => (
                <FormControl className="flex-1">
                  <Input placeholder="输入寄存器地址" {...field} />
                </FormControl>
              )}
            />
            <FormField
              control={form.control}
              name="writeregisterValue"
              render={({ field }) => (
                <FormControl className="w-24">
                  <Input placeholder="输入写入值" {...field} />
                </FormControl>
              )}
            />
            <Button type="button" onClick={writeRegister} disabled={loading}>
              {loading ? "写入中..." : "写入寄存器"}
            </Button>
          </div>
          <FormMessage />
        </FormItem>

        <Separator />
        <Button type='submit'>更新设置</Button>
      </form>
    </Form>
  )
}
