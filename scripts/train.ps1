# train.ps1
param (
  [string]$dataPath,
  [int]$epochs,
  [int]$workers,
  [int]$batch
)

& "D:\code\tauri-shadcn-template\fastapi\app\.venv\Scripts\Activate.ps1"

$p = Start-Process -FilePath "yolo" -ArgumentList @(
  'detect', 'train',
  "data=$dataPath",
  'model=yolov8s.pt',
  "epochs=$epochs",
  'imgsz=640',
  "workers=$workers",
  "batch=$batch",
  'project=D:\project'
) -PassThru -WindowStyle Hidden

$p.Id | Out-File yolo.pid
$p.WaitForExit()
