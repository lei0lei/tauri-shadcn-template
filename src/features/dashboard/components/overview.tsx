// import { Bar, BarChart, ResponsiveContainer, XAxis, YAxis } from 'recharts'
import { useEffect, useRef } from "react";
import { TransformWrapper, TransformComponent } from 'react-zoom-pan-pinch';
import { listen } from '@tauri-apps/api/event';
import { useDashboardStore } from "@/stores/dashboardStore";


export function Overview() {

  // const [value1] = useState<number>(0);
  // const [value2] = useState<number>(0);

  const image1 = useDashboardStore((state) => state.image_1);
  const image2 = useDashboardStore((state) => state.image_2);
  const addImage1 = useDashboardStore((state) => state.addImage_1);
  const addImage2 = useDashboardStore((state) => state.addImage_2);

  const value1 = useDashboardStore((state) => state.info_1);
  const value2 = useDashboardStore((state) => state.info_2);
  const set_value1 = useDashboardStore((state) => state.setInfo_1);
  const set_value2 = useDashboardStore((state) => state.setInfo_2);

  const prevImage1Ref = useRef<string | null>(null); // 记录上次的图片
  const prevImage2Ref = useRef<string | null>(null); // 记录上次的图片

  const prevInfo1Ref = useRef<string | null>(null); // 记录上次的图片
  const prevInfo2Ref = useRef<string | null>(null); // 记录上次的图片


  useEffect(() => {
    const setupListener = async () => {
      try {
        await listen("image-send-image-1", (event) => {
          console.log("Received image 1");
          const base64Image = event.payload as string;

          if (prevImage1Ref.current !== base64Image) {
            prevImage1Ref.current = base64Image;
            addImage1(base64Image);
          }
        });
      } catch (error) {
        console.error("Error while listening to the event:", error);
      }
    };

    setupListener();
  }, [addImage1]);


  useEffect(() => {
    const setupListener = async () => {
      try {
        await listen("image-send-image-2", (event) => {
          console.log("Received image 2");
          const base64Image = event.payload as string;

          if (prevImage2Ref.current !== base64Image) {
            prevImage2Ref.current = base64Image;
            addImage2(base64Image);
          }
        });
      } catch (error) {
        console.error("Error while listening to the event:", error);
      }
    };

    setupListener();
  }, [addImage2]);


  useEffect(() => {
    const setupListener = async () => {
      try {
        await listen("sensor-send-data-1", (event) => {
          console.log("Received data 1");
          const base64Image = event.payload as string;

          if (prevInfo1Ref.current !== base64Image) {
            prevInfo2Ref.current = base64Image;
            set_value1(base64Image);
          }
        });
      } catch (error) {
        console.error("Error while listening to the event:", error);
      }
    };

    setupListener();
  }, [set_value1]);


  useEffect(() => {
    const setupListener = async () => {
      try {
        await listen("sensor-send-data-2", (event) => {
          console.log("Received data 2");
          const base64Image = event.payload as string;

          if (prevInfo2Ref.current !== base64Image) {
            prevInfo2Ref.current = base64Image;
            set_value2(base64Image);
          }
        });
      } catch (error) {
        console.error("Error while listening to the event:", error);
      }
    };

    setupListener();
  }, [set_value2]);


  return (
    <div className="flex flex-col justify-between w-full h-full">
      {/* 图片区域，增加上边距 mt-6，并居中 */}
      <div className="grid grid-cols-2 gap-2 flex-grow min-h-[250px] mt-2 w-full">
        <div className="flex justify-center items-center border rounded-lg overflow-hidden bg-gray-200">
        <TransformWrapper>
          <TransformComponent  contentClass="w-full h-full">
          {image1 ? (
                <img
                  src={`data:image/jpeg;base64,${image1}`}
                  alt="Image 1"
                  className="w-full h-full object-cover"
                />
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
                <img
                  src={`data:image/jpeg;base64,${image2}`}
                  alt="Image 2"
                  className="w-full h-full object-cover"
                />
            ) : (
              <p className="text-gray-500">No Image</p>
            )}
          </TransformComponent>
        </TransformWrapper>
        </div>
      </div>
      
      {/* 数字信息区域，增加 `mt-4` 以分隔图片区域 */}
      <div className="grid grid-cols-2 gap-2 h-[70px] text-center mt-4 select-none">
        <div className="flex justify-center items-center bg-gray-200 rounded-lg text-xl">{value1}</div>
        <div className="flex justify-center items-center bg-gray-200 rounded-lg text-xl">{value2}</div>
      </div>
    </div>
  )
}
