import { useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Button } from "@/components/ui/button";

function StartStopButton({ className }: { className?: string }) {
  const [isRunning, setIsRunning] = useState(false);

  // 处理按钮点击
    // const callRustCommand = async () => {
  //   try {
  //     // 调用 Rust 后端的自定义命令
  //     const result = await invoke('test_call_from_frontend');
  //     setResponse(result as string); // 更新响应内容
  //   } catch (error) {
  //     console.error("Error calling Rust command:", error);
  //   }

  const handleClick = async () => {
    try {
      if (isRunning) {
        // 停止流程，传递 "end" 参数给后端
        const result = await invoke("start_software", { start_state: "end" });
        if (result === "ended") {
          console.log("ended");
          setIsRunning(false);
        }
      } else {
        // 启动流程，传递 "start" 参数给后端
        const result = await invoke("start_software", { start_state: "start" });
        if (result === "started") {
          console.log("started")
          setIsRunning(true);
        }
      }
    } catch (error) {
      console.error("Error during process invocation:", error);
    }
  };

  return (
    <Button
      className={`${
        isRunning ? "bg-green-700" : "bg-red-700"
      } text-white p-4 rounded-lg w-full text-xl ${className}`}
      onClick={handleClick}
    >
      {isRunning ? "停止" : "开始"}
    </Button>
  );
}

export default StartStopButton;