import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { Separator } from "@/components/ui/separator"
import {Areachart } from "./areachart"
import {Piechart } from "@/features/analytics/piechart"
import {Radiachart } from "./radiachart"

export default function Tasks() {
  return (
    <div>
      <Header fixed>
        {/* <Search /> */}
        <div className='ml-auto flex items-center space-x-4'>
        </div>
      </Header>

      <Main>
        <div className='mb-2 flex items-center justify-between space-y-2 flex-wrap gap-x-4'>
          <div>
            <h2 className='text-2xl font-bold tracking-tight'>Analytics</h2>
          </div>
          <Separator />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="w-full">
            <Piechart />
          </div>
          <div className="w-full">
            <Radiachart />
          </div>
        </div>

        <div className="mt-4">
          <Areachart />
        </div>
      </Main>
</div>

  )
}
