import { Badge } from "@/components/ui/badge"; // 引入Shadcn的Button和Badge组件
import { useDashboardStore } from "@/stores/dashboardStore"; // 导入 zustand store
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Dialog, DialogTrigger, DialogContent, DialogTitle, DialogHeader } from "@/components/ui/dialog"; // 引入Shadcn的Dialog组件
// import { invoke } from '@tauri-apps/api/core';
import { IconCircleCheck, IconCircleX } from "@tabler/icons-react"
import { Separator } from "@/components/ui/separator"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useState } from "react";
// import { Label } from "@/components/ui/label"
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
      } else{
        updateResultComponent(surfaceName, {
          holeIndex: hole,
          holeState: false, // 设为 true 表示孔状态良好（或者根据你的需求设定状态）
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
    <div className="space-y-1 flex-grow h-96">
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
      {dialogData && (
      <Dialog open={isDialogOpen && dialogData !== null} onOpenChange={setDialogOpen}>
        <DialogTrigger />
        <DialogContent className="max-w-xl">
        <DialogHeader>
          <div className="flex items-center gap-6 flex-wrap">
            <DialogTitle className="text-2xl">孔位检测详情</DialogTitle>
            <div className="flex items-center gap-4 text-sm text-muted-foreground">
              <div className="flex items-center gap-1">
              <Badge className="text-base"> 面{dialogData.face_id}</Badge>
              </div>
              <div className="flex items-center gap-1">
              <Badge className="text-base"> #{dialogData.hole_id}</Badge>
              </div>
              <div className="flex items-center gap-2">
                  {/* <Label className="whitespace-nowrap">型号：</Label> */}
                  <Badge className="text-base" >{dialogData.standard_hole_type}</Badge>
                </div>
                <div className="flex items-center gap-2">
                <Badge className={dialogData.hole_result ? "bg-green-700 text-white text-base" : "bg-red-700 text-white text-base"}>
                {dialogData.hole_result ? "OK" : "NG"}
              </Badge>
                </div>
            </div>
          </div>
        </DialogHeader>

          {dialogData ? (
            <div className="space-y-6">
              {/* 上方基础信息：一行内显示 */}
              <Separator />

              {/* 下方检测结果卡片（竖排） */}
              <div className="flex flex-col gap-4">
                {/* 深度检测 */}
                <Card>
                  <CardHeader className="flex-row justify-between items-center">
                    <div className="flex items-center gap-4">
                      <CardTitle className="text-lg">深度检测</CardTitle>
                      <span className="text-muted-foreground text-sm">通孔：{dialogData.thru_hole ? "是" : "否"}</span>
                    </div>
                    {dialogData.depth_result ? (
                      <IconCircleCheck className="text-green-700 w-11 h-11" />
                    ) : (
                      <IconCircleX className="text-red-700 w-11 h-11" />
                    )}
                  </CardHeader>
                  <CardContent className="text-base space-y-1">
                    <div className="flex justify-between">
                      <span>检测值：{dialogData.depth != null ? dialogData.depth.toFixed(4) : "无数据"}</span>
                      <span>标准：{dialogData.depth_min} - {dialogData.depth_max}</span>
                    </div>
                  </CardContent>
                </Card>
                {/* 直径检测 */}
                <Card>
                  <CardHeader className="flex-row justify-between items-center">
                    <CardTitle className="text-lg">直径检测</CardTitle>
                    {dialogData.dimeter_result ? (
                      <IconCircleCheck className="text-green-700 w-11 h-11" />
                    ) : (
                      <IconCircleX className="text-red-700 w-11 h-11" />
                    )}
                  </CardHeader>
                  <CardContent className="text-base space-y-1">
                    <div className="flex justify-between">
                      <span>检测值：{dialogData.diameter != null ? Number(dialogData.diameter).toFixed(2) : "无数据"}</span>
                      <span>标准：{dialogData.diameter_min} - {dialogData.diameter_max}</span>
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="flex-row justify-between items-center">
                    <CardTitle className="text-lg">螺纹检测</CardTitle>
                    {dialogData.luowen_result ? (
                      <IconCircleCheck className="text-green-700 w-11 h-11" />
                    ) : (
                      <IconCircleX className="text-red-700 w-11 h-11" />
                    )}
                  </CardHeader>
                  <CardContent className="text-base space-y-1">
                    <div className="flex justify-between">
                      <span>检测值：{dialogData.luowen ? "是" : "否"}</span>
                      <span>应有螺纹：{dialogData.have_luowen ? "是" : "否"}</span>
                    </div>
                  </CardContent>
                </Card>
              </div>
            </div>
          ) : (
            <div className="text-center py-8 text-muted-foreground">加载中...</div>
          )}
        </DialogContent>
      </Dialog>
      )}
    </div>
  );
}