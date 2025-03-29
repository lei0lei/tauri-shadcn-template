import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { toast } from '@/hooks/use-toast'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import {
  Form,
  FormItem,
  FormControl ,
} from '@/components/ui/form'
import { invoke } from '@tauri-apps/api/core';

import { Switch } from "@/components/ui/switch";
// import { invoke } from '@tauri-apps/api/core';
import { useSoftwareSettingsStore } from "@/stores/softwareSettingsStore";
import { useEffect } from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";


const accountFormSchema = z.object({
  modelType: z.enum(['EH09','EH12','EK30','EK40','EY28','TEST']), // 只允许选择 "EH09" 或 "EY28"
})

type AccountFormValues = z.infer<typeof accountFormSchema>
const artifactTypes = accountFormSchema.shape.modelType.options; // 获取枚举值数组



export function AccountForm() {
  const { isDevMode, toggleDevMode, selectedArtifactType, setSelectedArtifactType } = useSoftwareSettingsStore(); // 从 Zustand 获取状态

  const form = useForm<AccountFormValues>({
    resolver: zodResolver(accountFormSchema),
    defaultValues:{ modelType: selectedArtifactType },
  })

  useEffect(() => {
    // 当 Zustand 中的选项变更时，更新表单的选项
    form.setValue("modelType", selectedArtifactType);
  }, [selectedArtifactType, form]);

  async function onSubmit(data: AccountFormValues) {
    try {
      await invoke("frontend_select_artifact_type", { artifactType: data.modelType });
      toast({
        title: "设置成功",
        description: `已选择型号: ${data.modelType}`,
      });
      setSelectedArtifactType(data.modelType);
    } catch (error) {
      console.error("设置型号失败:", error);
      toast({
        title: "设置失败",
        description: "请检查后端是否正常运行",
        variant: "destructive",
      });
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
      {/* <div className="flex gap-2"> */}
        <FormItem>
          <div className="flex items-center gap-4">
            <span className="text-lg font-medium min-w-[150px]">硬件 Dev 模式</span>
            <FormControl>
              <Switch
                checked={isDevMode}
                onCheckedChange={toggleDevMode}
                className="data-[state=checked]:bg-green-700"
              />
            </FormControl>
          </div>
        </FormItem>
        {/* </div> */}
        <FormItem>
        <div className="flex items-center gap-4">
          <span className="text-lg font-medium min-w-[150px]">选择型号</span>
          <FormControl className="flex-1">
            <Select
              value={selectedArtifactType}
              onValueChange={setSelectedArtifactType}
            >
              <SelectTrigger className="w-full"> {/* w-full 使下拉框自适应宽度 */}
                <SelectValue placeholder="请选择型号" />
              </SelectTrigger>
              <SelectContent>
                {artifactTypes.map((type) => (
                  <SelectItem key={type} value={type}>
                    {type}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </FormControl>
        </div>
      </FormItem>

      <Separator />
        <Button type='submit'>更新设置</Button>
      </form>
    </Form>
  )
}
