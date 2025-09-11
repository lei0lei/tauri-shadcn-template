# stop.ps1
if (Test-Path "yolo.pid") {
  $pid = Get-Content yolo.pid
  Stop-Process -Id $pid -Force
  Remove-Item yolo.pid
} else {
  Write-Output "PID 文件不存在"
}
