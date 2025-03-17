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
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'



const accountFormSchema = z.object({
  ip: z
    .string()
    .min(7, { message: '请输入有效 IP 地址' })
    .max(15, { message: 'IP 地址格式错误' }),
  port: z
    .string()
    .min(1, { message: '请输入端口号' })
    .max(5, { message: '端口号格式错误' }),
})

type AccountFormValues = z.infer<typeof accountFormSchema>

// This can come from your database or API.
const defaultValues: Partial<AccountFormValues> = {
  ip: '',
  port: '',
}

export function AccountForm() {
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
        <Button type='submit'>更新设置</Button>
      </form>
    </Form>
  )
}
