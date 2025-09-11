// import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Badge } from "@/components/ui/badge";
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui/tooltip";
import { Tabs, TabsContent} from '@/components/ui/tabs'
import { Header } from '@/components/layout/header'
import { Main } from '@/components/layout/main'
import { ThemeSwitch } from '@/components/theme-switch'
import { Overview } from './components/overview'
import  LogWindow  from './components/stateandlogs'
import ResultShow from './components/resultshow'
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef,useState } from "react"
import StartStopButton from './components/startstop'
import { Separator } from '@radix-ui/react-separator'
import { useDashboardStore } from "@/stores/dashboardStore";
import { IconCheck, IconX, IconCamera, IconParkingCircleFilled, IconAsset, IconServer, IconCircle, IconDeviceFloppy } from "@tabler/icons-react";
// import Surreal from 'surrealdb';
import { toPng } from 'html-to-image';
import jsPDF from 'jspdf';
import { useHoleStore } from "@/stores/svgShow";
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { mkdir, exists  } from '@tauri-apps/plugin-fs';
import { Input } from "@/components/ui/input";
// import { Dialog, DialogTrigger, DialogContent, DialogClose } from "@/components/ui/dialog";
import { getDb } from "@/utils/surreal"; // 引入数据库初始化函数
import EH09_A from "@/assets/EH09_A.png";
import EH09_B from "@/assets/EH09_B.png";
import EH09_C from "@/assets/EH09_C.png";
import EH09_D from "@/assets/EH09_D.png";
import EH09_E from "@/assets/EH09_E.png";
import EH09_F from "@/assets/EH09_F.png";

import EH12_A from "@/assets/EH12_A.png";
import EH12_B from "@/assets/EH12_B.png";
import EH12_C from "@/assets/EH12_C.png";
import EH12_D from "@/assets/EH12_D.png";
import EH12_E from "@/assets/EH12_E.png";

import EY28_A from "@/assets/EY28_A.png";
import EY28_B from "@/assets/EY28_B.png";
import EY28_C from "@/assets/EY28_C.png";
import EY28_D from "@/assets/EY28_D.png";
import EY28_E from "@/assets/EY28_E.png";

import EK30_A from "@/assets/EK30_A.png";
import EK30_B from "@/assets/EK30_B.png";
import EK30_C from "@/assets/EK30_C.png";
import EK30_D from "@/assets/EK30_D.png";
import EK30_E from "@/assets/EK30_E.png";

import EK40_A from "@/assets/EK40_A.png";
import EK40_B from "@/assets/EK40_B.png";
import EK40_C from "@/assets/EK40_C.png";
import EK40_D from "@/assets/EK40_D.png";
import EK40_E from "@/assets/EK40_E.png";


const imageMap: Record<string, Record<string, string>> = {

  EH09: {
    A: EH09_A,
    B: EH09_B,
    C: EH09_C,
    D: EH09_D,
    E: EH09_E,
    F: EH09_F,
  },
  EH12: {
    A: EH12_A,
    B: EH12_B,
    C: EH12_C,
    D: EH12_D,
    E: EH12_E,
  },
  EY28: {
    A: EY28_A,
    B: EY28_B,
    C: EY28_C,
    D: EY28_D,
    E: EY28_E,
  },
  EK30: {
    A: EK30_A,
    B: EK30_B,
    C: EK30_C,
    D: EK30_D,
    E: EK30_E,
  },
  EK40: {
    A: EK40_A,
    B: EK40_B,
    C: EK40_C,
    D: EK40_D,
    E: EK40_E,
  },
  // 可添加其他类型
};


const iconMap : Record<string, React.ComponentType>= {
  "相机": IconCamera,
  "PLC": IconParkingCircleFilled,
  "传感器": IconCircle,
  "机器人": IconAsset,
  "算法": IconServer,
  "硬盘": IconDeviceFloppy,
};

import { invoke } from '@tauri-apps/api/core';

const faceReverseMapping: { [key: string]: number } = {
  "A": 1,
  "B": 2,
  "C": 3,
  "D": 4,
  "E": 5,
  "F": 6,
};

type ModelData = {
  [modelCode: string]: {
    last_face: number;
    last_hole: number;
  };
};

// 初始化
const D: ModelData = {
  EH09: { last_face: 6, last_hole: 4 },
  EH12: { last_face: 5, last_hole: 4 },
  EK30: { last_face: 5, last_hole: 5 },
  EK40: { last_face: 5, last_hole: 5 },
  EY28: { last_face: 5, last_hole: 8 },
};



export default function Dashboard() {

  // const dbInstance = useDashboardStore((state) => state.dbInstance);
  const setDbInstance = useDashboardStore((state) => state.setDbInstance);
  const dbInstanceRef = useRef<any>(null);
  const resultComponentValue = useDashboardStore((state) => state.resultComponentValue);
  const holePositions = useHoleStore((state) => state.holePositions);

  const [timer, setTimer] = useState(0)
  const timerRef = useRef<NodeJS.Timeout | null>(null)
  useEffect(() => {
    const startTimer = () => {
      // 如果定时器已经存在，先清除它
      console.log('start');
      if (timerRef.current !== null) {
        clearInterval(timerRef.current);
      }
      // 每次开始时重置计时器为 0
      setTimer(0);
      // 启动新的定时器
      timerRef.current = setInterval(() => {
        setTimer((prev) => prev + 1); // 每秒增加
      }, 1000);
    };


    const stopTimer = () => {
      console.log('stop');
      if (timerRef.current) {
        clearInterval(timerRef.current); // 停止定时器
        timerRef.current = null;          // 清除定时器引用
      }
      
    };


    const unlistenStart = listen("timer-start", startTimer)
    const unlistenStop = listen("timer-stop", stopTimer)

    return () => {
      unlistenStart.then(fn => fn())
      unlistenStop.then(fn => fn())
      if (timerRef.current) clearInterval(timerRef.current)
    }
  }, [])





  useEffect(() => {
    const initializeDb = async () => {
      if (!dbInstanceRef.current) {
        try {
          // 初始化数据库并将实例存储到 Zustand
          const db = await getDb();
          setDbInstance(db); // 将实例保存到 Zustand store
          dbInstanceRef.current = db;
          console.log("数据库连接成功");
        } catch (err) {
          console.error("数据库连接失败:", err);
        }
      }
    };

    initializeDb();
  }, [setDbInstance]);

  // @ts-ignore
  const { logs, setLogs, artifactType,setArtifactType,statics,artifact,setArtifact} = useDashboardStore();
  const {current_hole,setCurrentHole,current_face}= useDashboardStore();



  useEffect(() => {
    // 监听后端发送的 log_received 事件
    const handleStageReceived = async (event: { payload: { face: number, hole: number, artifact: string} }) => {
      const { face, hole,artifact} = event.payload;
      const faceStr = face.toString();
      const holeStr = hole.toString();
      setCurrentHole(holeStr,faceStr);
      setArtifact(artifact)
      const { last_face, last_hole } = D[artifactType];

      if (face == last_face && hole == last_hole) {
        const artifactForExport = artifact;
        console.log("已到最后一个面和孔，开始自动导出PDF");
        setTimeout(async () => {
          if (!dbInstanceRef.current) {
            console.error("数据库依然未初始化，取消导出");
            return;
          }
          await handleAutoExportClick(artifactForExport);
        }, 500);
      }
    };
  //   handleStageReceived({
  //   payload: {
  //     face: 5,        // 这里填你想要的初始face
  //     hole: 8,        // 这里填你想要的初始hole
  //     artifact: "----",   // 这里填初始artifact
  //   },
  // });
    // 监听 "log_received" 事件
    const unlisten = listen("current_stage", handleStageReceived);

    // 清理监听器
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [setCurrentHole,artifactType]);


  useEffect(() => {
    // 监听后端发送的 log_received 事件
    const handleTypeReceived = (event: { payload: string }) => {
      const current_type = event.payload;

      setArtifactType(current_type);
    };

    // 监听 "log_received" 事件
    const unlisten = listen("current-type", handleTypeReceived);

    // 清理监听器
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [setArtifactType]);


  useEffect(() => {
    const fetchData = async () => {
      if (artifactType === '---') { // 只有在 artifactType 为 '---' 时执行
        try {
          const result = await invoke("selected_artifact_type"); // 调用后端命令
          console.log("后端返回的数据:", result);
        } catch (error) {
          console.error("调用后端命令失败:", error);
        }
      } else {
        console.log("artifactType 已设置，不执行 invoke");
      }
    };

    fetchData();
  }, []); // 空数组表示只在组件挂载时执行一次



  const initSidecarListeners = async () => {
    // Listen for stdout lines from the sidecar

    const unlistenStdout = await listen('sidecar-stdout', (event) => {
      console.log('Sidecar stdout:', event.payload);
      if (`${event.payload}`.length > 0 && event.payload !== "\r\n")
        useDashboardStore.getState().setLogs(event.payload as string);
    });

    // Listen for stderr lines from the sidecar
    const unlistenStderr = await listen('sidecar-stderr', (event) => {
      console.error('Sidecar stderr:', event.payload);
      if (`${event.payload}`.length > 0 && event.payload !== "\r\n")
        useDashboardStore.getState().setLogs(event.payload as string);
    });

    // Cleanup listeners when not needed
    return () => {
      unlistenStdout();
      unlistenStderr();
    };
  }

  useEffect(() => {
    initSidecarListeners()
  }, [])

  const handleAutoExportClick = async (artifactArg:string) => {
    if (!dbInstanceRef.current) {
    console.warn("数据库未初始化，尝试初始化...");
    try {
      const db = await getDb();
      setDbInstance(db);
      console.log("数据库初始化成功！");
    } catch (err) {
      console.error("数据库初始化失败:", err);
      return;
    }
    }
    const doc = new jsPDF({
        orientation: 'landscape', // 横向
        unit: 'px',               // 使用像素单位（可选）
        format: [800, 1600]        // 自定义宽高（如有需要）
      })

    for (const { surface, holes } of resultComponentValue) {
      // === 1. 查询当前面孔数据 ===
      if (!dbInstanceRef.current) {
        console.error("数据库未初始化")
        continue  // 跳过当前面
      }
      const faceId = faceReverseMapping[surface];
      const query = `
        SELECT * FROM Hole_library
        WHERE artifact_name = $artifactArg AND face_id = $faceId
        ORDER BY hole_id
      `;
      const surreal_result = await dbInstanceRef.current.query(query, {
        artifactArg,
        faceId: faceId,
      });
      const holesData = Array.isArray(surreal_result?.[0]) ? surreal_result[0] : [];
      const surfaceImage = imageMap[artifactType]?.[surface]
      if (!surfaceImage) continue


      const leftWrapper = document.createElement("div");
      leftWrapper.style.width = "800px";
      leftWrapper.style.height = "800px";
      leftWrapper.style.position = "absolute";
      leftWrapper.style.top = "80px";
      leftWrapper.style.left = "50px";


      const svgNS = "http://www.w3.org/2000/svg"
      const svg = document.createElementNS(svgNS, "svg")
      svg.setAttribute("viewBox", "0 0 800 800")
      svg.setAttribute("width", "800")
      svg.setAttribute("height", "800")

      // 网格
      const defs = document.createElementNS(svgNS, "defs")
      const pattern = document.createElementNS(svgNS, "pattern")
      pattern.setAttribute("id", "grid")
      pattern.setAttribute("width", "30")
      pattern.setAttribute("height", "30")
      pattern.setAttribute("patternUnits", "userSpaceOnUse")
      const path = document.createElementNS(svgNS, "path")
      path.setAttribute("d", "M 30 0 L 0 0 0 30")
      path.setAttribute("fill", "none")
      path.setAttribute("stroke", "rgba(0,0,0,0.1)")
      path.setAttribute("stroke-width", "1")
      pattern.appendChild(path)
      defs.appendChild(pattern)
      svg.appendChild(defs)

      const grid = document.createElementNS(svgNS, "rect")
      grid.setAttribute("width", "100%")
      grid.setAttribute("height", "100%")
      grid.setAttribute("fill", "url(#grid)")
      svg.appendChild(grid)


      // 背景图
      const bg = document.createElementNS(svgNS, "image")
      bg.setAttribute("href", surfaceImage) // ✅ 推荐写法，现代浏览器支持
      bg.setAttribute("x", "0")
      bg.setAttribute("y", "0")
      bg.setAttribute("width", "800")
      bg.setAttribute("height", "800")
      svg.appendChild(bg)

      // 孔位
      holes.forEach((status, index) => {
        const holeId = index + 1
        const pos = holePositions.find(
          (p) => p.artifact === artifactType && p.surface === surface && p.holeId === holeId
        )
        if (!pos) return

        const circle = document.createElementNS(svgNS, "circle")
        circle.setAttribute("cx", pos.x.toString())
        circle.setAttribute("cy", pos.y.toString())
        circle.setAttribute("r", pos.r.toString())
        circle.setAttribute("stroke", "#1e293b");
        circle.setAttribute("stroke-width", "5");
        status=true
        const fill =
          status === true ? "#15803d" : status === false ? "#b91c1c" : "#1e293b"
        circle.setAttribute("fill", fill)
        svg.appendChild(circle)
      })


      leftWrapper.appendChild(svg);
      document.body.appendChild(leftWrapper);

          // 2. 创建右侧表格容器，独立生成
      const rightWrapper = document.createElement("div");
      rightWrapper.style.width = "800px";
      rightWrapper.style.height = "800px";
      rightWrapper.style.position = "absolute";
      rightWrapper.style.top = "80px";
      rightWrapper.style.left = "50px";
      // rightWrapper.style.fontSize = "12px";

      rightWrapper.innerHTML = `
            <table border="1" style="border-collapse:collapse;width:90%;font-size:16px;text-align:center;line-height:2;">
              <thead>
                <tr style="background:#334155;color:white;">
                  <th>孔位</th><th>型号</th><th>直径</th><th>深度</th><th>螺纹</th>
                </tr>
              </thead>
              <tbody>
                ${(holesData as any[]).map((hole: any, index: number) => {
                  const holeId = index + 1;
                  const diameterColor = hole.dimeter_result === false ? "red" : "#334155";
                  const depthColor = hole.depth_result === false ? "red" : "#334155";
                  const threadColor = hole.luowen_result === false ? "red" : "#334155";
                  return `
                    <tr style="background:${index % 2 === 0 ? "#e2e8f0" : "#f1f5f9"};">
                      <td>${holeId}</td>
                      <td>${hole.standard_hole_type || ""}</td>
                      <td style="color:${diameterColor};">${hole.diameter?.toFixed(3) || ""}</td>
                      <td style="color:${depthColor};">${hole.depth?.toFixed(3) || ""}</td>
                      <td style="color:${threadColor};">${hole.have_luowen === undefined ? "" : hole.luowen ? "是" : "否"}</td>
                    </tr>
                  `;
                }).join("")}
              </tbody>
            </table>
          `;
      document.body.appendChild(rightWrapper);

      // 3. 分别截图
      const leftDataUrl = await toPng(leftWrapper);
      const rightDataUrl = await toPng(rightWrapper);
      // 4. 加到 PDF，左右各占 800x800，间距可调整
      doc.text(surface, 40, 40); // 标题放这里

      doc.addImage(leftDataUrl, "PNG", 10, 10, 800, 800);
      doc.addImage(rightDataUrl, "PNG", 800, 50, 800, 800);

      doc.addPage();

      // 清理DOM
      document.body.removeChild(leftWrapper);
      document.body.removeChild(rightWrapper);


    }
  // 移除最后一页空白
    const pageCount = doc.getNumberOfPages();
    if (pageCount > 1) {
      doc.deletePage(pageCount);
    }
    const pdfArrayBuffer = doc.output("arraybuffer");
    const pdfBytes = new Uint8Array(pdfArrayBuffer);
    const userName = useDashboardStore.getState().userName || "defaultUser";
    const baseDir = `D:/pdfresult/${userName}`;

    exists(baseDir).then((dirExists) => {
      if (!dirExists) {
        return mkdir(baseDir, { recursive: true }).then(() => {
          console.log("目录已创建:", baseDir);
        });
      }
    }).then(() => {
      const defaultName = `${baseDir}/${artifactType}-${artifactArg}.pdf`;
      return writeFile(defaultName, pdfBytes).then(() => {
        console.log("PDF 已自动保存:", defaultName);
      });
    }).catch((err) => {
      console.error("保存出错:", err);
    });

  };
  const handleExportClick = async () => {
    const doc = new jsPDF({
      orientation: 'landscape', // 横向
      unit: 'px',               // 使用像素单位（可选）
      format: [800, 1600]        // 自定义宽高（如有需要）
    })

    for (const { surface, holes } of resultComponentValue) {
      // === 1. 查询当前面孔数据 ===
      if (!dbInstanceRef.current) {
        console.error("数据库未初始化")
        continue  // 跳过当前面
      }
      const faceId = faceReverseMapping[surface];
      const query = `
        SELECT * FROM Hole_library
        WHERE artifact_name = $artifact AND face_id = $faceId
        ORDER BY hole_id
      `;
      const surreal_result = await dbInstanceRef.current.query(query, {
        artifact,
        faceId: faceId,
      });
      const holesData = Array.isArray(surreal_result?.[0]) ? surreal_result[0] : [];
      const surfaceImage = imageMap[artifactType]?.[surface]
      if (!surfaceImage) continue


      const leftWrapper = document.createElement("div");
      leftWrapper.style.width = "800px";
      leftWrapper.style.height = "800px";
      leftWrapper.style.position = "absolute";
      leftWrapper.style.top = "80px";
      leftWrapper.style.left = "50px";


      const svgNS = "http://www.w3.org/2000/svg"
      const svg = document.createElementNS(svgNS, "svg")
      svg.setAttribute("viewBox", "0 0 800 800")
      svg.setAttribute("width", "800")
      svg.setAttribute("height", "800")

      // 网格
      const defs = document.createElementNS(svgNS, "defs")
      const pattern = document.createElementNS(svgNS, "pattern")
      pattern.setAttribute("id", "grid")
      pattern.setAttribute("width", "30")
      pattern.setAttribute("height", "30")
      pattern.setAttribute("patternUnits", "userSpaceOnUse")
      const path = document.createElementNS(svgNS, "path")
      path.setAttribute("d", "M 30 0 L 0 0 0 30")
      path.setAttribute("fill", "none")
      path.setAttribute("stroke", "rgba(0,0,0,0.1)")
      path.setAttribute("stroke-width", "1")
      pattern.appendChild(path)
      defs.appendChild(pattern)
      svg.appendChild(defs)

      const grid = document.createElementNS(svgNS, "rect")
      grid.setAttribute("width", "100%")
      grid.setAttribute("height", "100%")
      grid.setAttribute("fill", "url(#grid)")
      svg.appendChild(grid)


      // 背景图
      const bg = document.createElementNS(svgNS, "image")
      bg.setAttribute("href", surfaceImage) // ✅ 推荐写法，现代浏览器支持
      bg.setAttribute("x", "0")
      bg.setAttribute("y", "0")
      bg.setAttribute("width", "800")
      bg.setAttribute("height", "800")
      svg.appendChild(bg)

      // 孔位
      holes.forEach((status, index) => {
        const holeId = index + 1
        const pos = holePositions.find(
          (p) => p.artifact === artifactType && p.surface === surface && p.holeId === holeId
        )
        if (!pos) return

        const circle = document.createElementNS(svgNS, "circle")
        circle.setAttribute("cx", pos.x.toString())
        circle.setAttribute("cy", pos.y.toString())
        circle.setAttribute("r", pos.r.toString())
        circle.setAttribute("stroke", "#1e293b");
        circle.setAttribute("stroke-width", "5");

        const fill =
          status === true ? "#15803d" : status === false ? "#b91c1c" : "#1e293b"
        circle.setAttribute("fill", fill)
        svg.appendChild(circle)
      })


      leftWrapper.appendChild(svg);
      document.body.appendChild(leftWrapper);

          // 2. 创建右侧表格容器，独立生成
      const rightWrapper = document.createElement("div");
      rightWrapper.style.width = "800px";
      rightWrapper.style.height = "800px";
      rightWrapper.style.position = "absolute";
      rightWrapper.style.top = "80px";
      rightWrapper.style.left = "50px";
      // rightWrapper.style.fontSize = "12px";

      rightWrapper.innerHTML = `
            <table border="1" style="border-collapse:collapse;width:90%;font-size:16px;text-align:center;line-height:2;">
              <thead>
                <tr style="background:#334155;color:white;">
                  <th>孔位</th><th>型号</th><th>直径</th><th>深度</th><th>螺纹</th>
                </tr>
              </thead>
              <tbody>
                ${(holesData as any[]).map((hole: any, index: number) => {
                  const holeId = index + 1;
                  const diameterColor = hole.dimeter_result === false ? "red" : "#334155";
                  const depthColor = hole.depth_result === false ? "red" : "#334155";
                  const threadColor = hole.luowen_result === false ? "red" : "#334155";
                  return `
                    <tr style="background:${index % 2 === 0 ? "#e2e8f0" : "#f1f5f9"};">
                      <td>${holeId}</td>
                      <td>${hole.standard_hole_type || ""}</td>
                      <td style="color:${diameterColor};">${hole.diameter?.toFixed(3) || ""}</td>
                      <td style="color:${depthColor};">${hole.depth?.toFixed(3) || ""}</td>
                      <td style="color:${threadColor};">${hole.have_luowen === undefined ? "" : hole.luowen ? "是" : "否"}</td>
                    </tr>
                  `;
                }).join("")}
              </tbody>
            </table>
          `;
      document.body.appendChild(rightWrapper);

      // 3. 分别截图
      const leftDataUrl = await toPng(leftWrapper);
      const rightDataUrl = await toPng(rightWrapper);
      // 4. 加到 PDF，左右各占 800x800，间距可调整
      doc.text(surface, 40, 40); // 标题放这里

      doc.addImage(leftDataUrl, "PNG", 10, 10, 800, 800);
      doc.addImage(rightDataUrl, "PNG", 800, 50, 800, 800);

      doc.addPage();

      // 清理DOM
      document.body.removeChild(leftWrapper);
      document.body.removeChild(rightWrapper);


    }
  // 移除最后一页空白
    const pageCount = doc.getNumberOfPages();
    if (pageCount > 1) {
      doc.deletePage(pageCount);
    }

    // 获取 PDF 二进制内容
    const pdfArrayBuffer = doc.output("arraybuffer");
    const pdfBytes = new Uint8Array(pdfArrayBuffer);
    const userName = useDashboardStore.getState().userName || "defaultUser";
    const baseDir = `D:/pdfresult/${userName}`;
    const dirExists = await exists(baseDir);
    if (!dirExists) {
      // 递归创建目录（绝对路径，不带 baseDir）
      await mkdir(baseDir, { recursive: true });
      console.log('目录已创建:', baseDir);
    }


    // 弹出保存对话框
    const defaultName = `${baseDir}/${artifactType}-${artifact}.pdf`;
    const path = await save({
      defaultPath: defaultName,
      filters: [{ name: "PDF 文件", extensions: ["pdf"] }]
    });

    if (path) {
      await writeFile(path, pdfBytes);
      console.log("PDF 保存成功:", path);
    } else {
      console.log("用户取消保存");
    }
  }



  return (
    <div className='flex flex-col min-h-screen p-4'>
      <Header>
        <div className="flex-1 flex justify-center">
          <StartStopButton className="w-full" />
        </div>
        <div className='ml-auto flex items-center space-x-4'>
          <ThemeSwitch />
        </div>
      </Header>


      <Main className="overflow-x-hidden overflow-y-hidden flex-grow">
        <Tabs
          orientation='vertical'
          defaultValue='overview'
          className='h-full'
        >
          <TabsContent value='overview' className='space-y-4 h-full'>
            <div className='grid gap-4 sm:grid-cols-2 lg:grid-cols-4'>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <div className="flex items-center gap-2 whitespace-nowrap">
                    <span className="text-sm font-medium">
                      型号
                    </span>
                    <Input
                      id="username"
                      placeholder="输入用户名并回车"
                      className="h-6 max-w-xs py-0"
                      onKeyDown={(e) => {
                        if (e.key === "Enter") {
                          const name = (e.target as HTMLInputElement).value.trim();
                          if (name) {
                            useDashboardStore.getState().setUsername(name);
                          }
                        }
                      }}
                    />
                  </div>
                </CardHeader>
                <CardContent>
                  <div className='text-5xl font-bold'>{artifactType}</div>
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>
                    计时
                  </CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <path d='M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2' />
                    <circle cx='9' cy='7' r='4' />
                    <path d='M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75' />
                  </svg>
                </CardHeader>
                <CardContent>
                  <div className='text-5xl font-bold'>
                    {String(Math.floor(timer / 60)).padStart(2, '0')}:{String(timer % 60).padStart(2, '0')}
                  </div>
                  
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>当前工件</CardTitle>
                  <Badge
                    className="cursor-pointer"
                    onClick={handleExportClick}                  >
                    导出工件数据
                  </Badge>
                 
                </CardHeader>
                <CardContent>
                <div className='text-lg font-bold'>{artifact}</div>
                <p className="text-sm text-muted-foreground">面: {current_face} | 孔: {current_hole}</p>
                </CardContent>
              </Card>
              <Card className="select-none">
                <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                  <CardTitle className='text-sm font-medium'>
                    系统状态
                  </CardTitle>
                  <svg
                    xmlns='http://www.w3.org/2000/svg'
                    viewBox='0 0 24 24'
                    fill='none'
                    stroke='currentColor'
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    className='h-4 w-4 text-muted-foreground'
                  >
                    <path d='M22 12h-4l-3 9L9 3l-3 9H2' />
                  </svg>
                </CardHeader>
                <CardContent className="grid grid-cols-3 gap-2">
                <StatusItem label="相机" status={true} />
                <StatusItem label="PLC" status={true} />
                <StatusItem label="传感器" status={false} />
                <StatusItem label="机器人" status={true} />
                <StatusItem label="算法" status={true} />
                <StatusItem label="硬盘" status={true} />
      </CardContent>
              </Card>
            </div>
            <div className='grid grid-cols-1 gap-4 lg:grid-cols-10 flex-grow'>
              <Card className='col-span-1 lg:col-span-8 pt-2 h-full'>
                <CardContent className='px-2 flex justify-center items-center h-full'>
                  <Overview />
                </CardContent>
              </Card>
              <Card className='col-span-1 lg:col-span-2 flex flex-col h-full'>
                <CardContent className="flex flex-col h-full pt-4 px-2 pb-2">
                <div className="flex-grow p-2 bg-gray-100 rounded-lg shadow-lg mb-2" style={{ maxHeight: '220px' }}>
                  <ResultShow />
                </div>
                <Separator />
                <div className="mt-2 flex-grow">
                  <LogWindow />
                  </div>
              </CardContent>
              </Card>
            </div>
          </TabsContent>
        </Tabs>
      </Main>
    </div>
  )
}

function StatusItem({ label, status }: { label: string; status: boolean }) {
  const Icon = iconMap[label];

  return (
    <div className="flex items-center space-x-2">
      {/* Badge 悬停显示 label */}

          <Badge className={status ? "bg-green-700 text-white" : "bg-red-700 text-white"}>
            {status ? <IconCheck className="w-4 h-4 text-white" /> : <IconX className="w-4 h-4 text-white" />}
          </Badge>

      {/* Icon 悬停显示 label */}
      <Tooltip>
        <TooltipTrigger asChild>
          {Icon && <Icon/>}
        </TooltipTrigger>
        <TooltipContent>{label}</TooltipContent>
      </Tooltip>
    </div>
  );
}