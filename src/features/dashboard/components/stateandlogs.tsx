import { ScrollArea } from "@/components/ui/scroll-area";
import { useEffect, useRef, useLayoutEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useDashboardStore } from "@/stores/dashboardStore";




export default function LogWindow() {
  const logs = useDashboardStore((state) => state.logComponentValue); // 取 Zustand 中的日志
  const addLog = useDashboardStore((state) => state.addLogComponentValueEntry); // 获取日志更新方法
  
  const logEndRef = useRef<HTMLDivElement | null>(null); // 引用滚动到最后的元素

  useEffect(() => {
    // 监听后端发送的 log_received 事件
    const handleLogReceived = (event: { payload: string }) => {
      const logMessage = event.payload;

      // 使用正则表达式解析日志格式: [source] [type] [message]
      const logPattern = /^\[(.*?)\] \[(.*?)\] \[(.*?)\]$/;
      const match = logMessage.match(logPattern);

      if (match) {
        const sender = match[1]; // 日志来源 (如 camera)
        const info = match[3]; // 日志内容 (如 打开相机成功: 0)
        const level = match[2];
        // 将解析后的日志信息添加到日志列表中
        addLog({ sender, level, info });
      } else {
        console.warn("日志格式不符合预期: ", logMessage);
      }
    };

    // 监听 "log_received" 事件
    const unlisten = listen("log_received", handleLogReceived);

    // 清理监听器
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [addLog]);

  // 在 logs 更新时，滚动到最底部
  useLayoutEffect(() => {
    if (logEndRef.current) {
      logEndRef.current.scrollIntoView({ behavior: "smooth", block: "end" });
    }
  }, [logs]); // 依赖 logs，当日志更新时触发滚动
  return (
    <ScrollArea className="h-72 p-4 bg-gray-300 rounded-lg shadow-lg overflow-auto">
      <div className="space-y-1">
        {logs.map((log, idx) => (
          <div key={idx} className="text-sm text-gray-700">
            <strong className="text-blue-950">{log.sender}:</strong> {log.info}
          </div>
        ))}
        {/* 通过 ref 引用最后的日志项 */}
        <div ref={logEndRef} />
      </div>
    </ScrollArea>
  );
}  