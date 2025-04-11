import { Badge } from "@/components/ui/badge"; // 引入Shadcn的Button和Badge组件
import { useDashboardStore } from "@/stores/dashboardStore"; // 导入 zustand store
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Dialog, DialogTrigger, DialogContent, DialogTitle, DialogDescription } from "@/components/ui/dialog"; // 引入Shadcn的Dialog组件
// import { invoke } from '@tauri-apps/api/core';
import { Separator } from "@/components/ui/separator"

import { useState } from "react";

const faceMapping: { [key: number]: string } = {
  1: "A",
  2: "B",
  3: "C",
  4: "D",
  5: "E",
  6: "F",
  // 如果有更多的 face 数值，可以继续添加
};

const faceReverseMapping: { [key: string]: number } = {
  "A": 1,
  "B": 2,
  "C": 3,
  "D": 4,
  "E": 5,
  "F": 6,
};
export default function ResultShow() {
  const resultComponentValue = useDashboardStore((state) => state.resultComponentValue);
  const updateResultComponent = useDashboardStore((state) => state.updateResultComponent);
  const clearResult = useDashboardStore((state) => state.clearResult);
  const [isDialogOpen, setDialogOpen] = useState(false);
  const [dialogData, setDialogData] = useState<any>(null); // 用于存储从 Tauri 获取的数据
  const artifact = useDashboardStore((state) => state.artifact);
  const dbInstance = useDashboardStore((state) => state.dbInstance);


  const handleBadgeClick = async (surface: string, holeIndex: number) => {
    try {
      if (!dbInstance) {
        console.error("数据库实例未初始化");
        setDialogData("数据库未连接");
        setDialogOpen(true);
        return;
      }

      // 调用 Tauri 后端命令，传递 surface 和 holeIndex 参数
      const dialogData = 'test';
      console.log(surface);
      console.log(holeIndex);
      setDialogData(dialogData); // 更新 Dialog 中的数据
      setDialogOpen(true);  // 打开 Dialog

      const faceId = faceReverseMapping[surface];

      const query = `
      SELECT * FROM Hole_library
      WHERE artifact_name = $artifact
      AND face_id = $faceId
      AND hole_id = $holeId
      `;

      const surreal_result = await dbInstance.query(query, {
        artifact,
        faceId,
        holeId: holeIndex,
      });
      const result = surreal_result as any[][]; // 强制断言为二维数组
      const data = result?.[0]?.[0];
      console.log("queryResult:", data);

      setDialogData(data); // data 是你从数据库拿到的记录对象
      setDialogOpen(true);

    } catch (error) {
      console.error("调用 Tauri 后端命令失败:", error);
    }
    // try {
    //   // 调用 Tauri 后端命令，传递 surface 和 holeIndex 参数
    //   const dialogData = await invoke("get_face_hole_result", {
    //     surface,
    //     holeIndex,
    //   });
      
    //   setDialogData(dialogData); // 更新 Dialog 中的数据
    //   setDialogOpen(true);  // 打开 Dialog
    // } catch (error) {
    //   console.error("调用 Tauri 后端命令失败:", error);
    // }
  };

  useEffect(() => {
    // 监听后端发送的 hole_final_result 事件
    const handleHoleFinalResult = (event: { payload: { face: number, hole: number, artifact: string, final_result: boolean } }) => {
      const { face, hole, final_result } = event.payload;

      // 根据 face 数值转换成字母
      const surfaceName = faceMapping[face] || "Unknown";

      // 如果 final_result 是 true，更新对应的 surface 和 hole 状态
      if (final_result) {
        updateResultComponent(surfaceName, {
          holeIndex: hole,
          holeState: true, // 设为 true 表示孔状态良好（或者根据你的需求设定状态）
        });
      }
    };

    // 监听 "hole_final_result" 事件
    const unlisten = listen("hole_final_result", handleHoleFinalResult);

    // 清理监听器
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [updateResultComponent]); // 依赖于 updateResultComponent

  useEffect(() => {
    // 监听后端发送的 log_received 事件
    const handleFinishedReceived = (event: { payload: boolean }) => {
      const finished = event.payload;
      if (finished){
        clearResult();
      }
    };

    // 监听 "log_received" 事件
    const unlisten = listen("current-finished", handleFinishedReceived);

    // 清理监听器
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [clearResult]);




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
                onClick={() => hole !== null && handleBadgeClick(surface.surface, idx + 1)}
              />
            ))}
          </div>
        </div>
      ))}
      {/* Dialog 组件 */}
      <Dialog open={isDialogOpen} onOpenChange={setDialogOpen}>
        <DialogTrigger />
        <DialogContent>
          <DialogTitle>孔详细信息</DialogTitle>
          <DialogDescription>
          {dialogData ? (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div><strong>面</strong>: {dialogData.face_id}</div>
                <div><strong>孔索引</strong>: {dialogData.hole_id}</div>
                <div><strong>深度</strong>: {dialogData.depth}</div>
                <div><strong>孔类型</strong>: {dialogData.standard_hole_type}</div>
                <div><strong>直径</strong>: {dialogData.diameter}（{dialogData.diameter_min} ~ {dialogData.diameter_max}）</div>
              </div>

              <Separator />

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <strong>螺纹检测</strong>:{" "}
                  <Badge variant={dialogData.luowen_result ? "default" : "destructive"}>
                    {dialogData.luowen_result ? "通过" : "不通过"}
                  </Badge>
                </div>
                <div>
                  <strong>孔整体检测</strong>:{" "}
                  <Badge variant={dialogData.hole_result ? "default" : "destructive"}>
                    {dialogData.hole_result ? "通过" : "不通过"}
                  </Badge>
                </div>
              </div>

              <Separator />

              <div className="space-y-2">
                <div><strong>内圆直径</strong>: {dialogData.action3?.nei_diameter}</div>
                <div><strong>外圆直径</strong>: {dialogData.action3?.wai_diameter}</div>
                <div><strong>识别置信度</strong>: {dialogData.action4?.confidence}</div>
              </div>
            </div>
          ) : (
            <div>加载中...</div>
          )}
          </DialogDescription>
        </DialogContent>
      </Dialog>

    </div>
  );
}
