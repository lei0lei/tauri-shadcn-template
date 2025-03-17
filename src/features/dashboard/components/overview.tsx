// import { Bar, BarChart, ResponsiveContainer, XAxis, YAxis } from 'recharts'
import { IconPhotoScan } from '@tabler/icons-react'; // 用于显示图标
import { useState } from "react";
import { TransformWrapper, TransformComponent } from 'react-zoom-pan-pinch';


export function Overview() {
  const [image1, setImage1] = useState<string | null>(null);
  const [image2, setImage2] = useState<string | null>(null);
  const [value1, setValue1] = useState<number>(0);
  const [value2, setValue2] = useState<number>(0);



  return (
    <div className="flex flex-col justify-between w-full h-full">
      {/* 图片区域，增加上边距 mt-6，并居中 */}
      <div className="grid grid-cols-2 gap-2 flex-grow min-h-[250px] mt-2 w-full">
        <div className="flex justify-center items-center border rounded-lg overflow-hidden bg-gray-200">
        <TransformWrapper>
          <TransformComponent>
            {image1 ? (
              <img src={image1} alt="Image 1" className="w-full h-full object-contain" />
            ) : (
              <p className="text-gray-500">No Image</p>
            )}
          </TransformComponent>
        </TransformWrapper>
        </div>
        <div className="flex justify-center items-center border rounded-lg overflow-hidden bg-gray-200">
        <TransformWrapper>
          <TransformComponent>
            {image2 ? (
              <img src={image2} alt="Image 2" className="w-full h-full object-contain" />
            ) : (
              <p className="text-gray-500">No Image</p>
            )}
          </TransformComponent>
        </TransformWrapper>
        </div>
      </div>
      
      {/* 数字信息区域，增加 `mt-4` 以分隔图片区域 */}
      <div className="grid grid-cols-2 gap-2 h-[40px] text-center mt-4">
        <div className="flex justify-center items-center bg-gray-200 rounded-lg">{value1}</div>
        <div className="flex justify-center items-center bg-gray-200 rounded-lg">{value2}</div>
      </div>
    </div>
  )
}
