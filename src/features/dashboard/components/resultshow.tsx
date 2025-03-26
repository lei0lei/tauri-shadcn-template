import { Badge } from "@/components/ui/badge"; // 引入Shadcn的Button和Badge组件
import { useDashboardStore } from "@/stores/dashboardStore"; // 导入 zustand store

export default function ResultShow() {
  const resultComponentValue = useDashboardStore((state) => state.resultComponentValue);
  // const updateResultComponent = useDashboardStore((state) => state.updateResultComponent);



  return (
    <div className="space-y-1 flex-grow">
      {resultComponentValue.map((surface, index) => (
        <div key={index} className="flex items-center border p-1 rounded-lg max-h-10 select-none">
          {/* 第一列: 面名称和状态，占 1/9 */}
          <div className="w-[10%] flex-shrink-0 text-center mr-4 pr-4 border-r-2">
          <h5
              className={`text-xs font-medium ${
                surface.status === "OK"
                  ? "text-green-700"
                  : surface.status === "NG"
                  ? "text-red-700"
                  : "text-gray-700"
              }`}
            >
              <strong>{surface.surface}</strong>
            </h5>
          </div>
          
          {/* 第二列: 孔的状态，占 8/9 */}
          <div className="flex flex-wrap w-8/9 justify-start items-center gap-x-2 gap-y-1">
            {surface.holes.map((hole, idx) => (
              <Badge
                key={idx}
                className={`basis-[10px] text-center py-1 rounded-md ${
                  hole === true
                    ? "bg-green-700 text-white"
                    : hole === false
                    ? "bg-red-700 text-white"
                    : "bg-gray-700 text-white"
                }`} // 根据状态颜色显示
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
