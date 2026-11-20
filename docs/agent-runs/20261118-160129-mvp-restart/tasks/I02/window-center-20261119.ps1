$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class WidgetCenterObservation {
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
  [StructLayout(LayoutKind.Sequential)] public struct MONITORINFO { public int Size; public RECT Monitor, Work; public uint Flags; }
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  [DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int attribute, out RECT r, int size);
  [DllImport("user32.dll")] public static extern IntPtr MonitorFromWindow(IntPtr h, uint flags);
  [DllImport("user32.dll", CharSet=CharSet.Auto)] public static extern bool GetMonitorInfo(IntPtr h, ref MONITORINFO info);
}
'@
$widgetExe = 'C:\Users\semis\orca\workspaces\GithubIssueWidget\issue-widget-mvp-resume\widget\src-tauri\target\release\github-issue-widget.exe'
if (-not (Test-Path -LiteralPath $widgetExe -PathType Leaf)) { throw 'Target executable not found' }
$widgetHash = (Get-FileHash -LiteralPath $widgetExe -Algorithm SHA256).Hash
$widgetProcess = Start-Process -FilePath $widgetExe -PassThru
try {
  for ($attempt = 0; $attempt -lt 50; $attempt++) {
    $widgetProcess.Refresh()
    if ($widgetProcess.HasExited) { throw 'Target exited before observation' }
    if ($widgetProcess.MainWindowHandle -ne [IntPtr]::Zero) { break }
    Start-Sleep -Milliseconds 200
  }
  $hwnd = $widgetProcess.MainWindowHandle
  if ($hwnd -eq [IntPtr]::Zero) { throw 'Target window not found' }
  $origin = New-Object WidgetCenterObservation+POINT
  $client = New-Object WidgetCenterObservation+RECT
  $frame = New-Object WidgetCenterObservation+RECT
  if (-not [WidgetCenterObservation]::ClientToScreen($hwnd, [ref]$origin)) { throw 'Client origin unavailable' }
  if (-not [WidgetCenterObservation]::GetClientRect($hwnd, [ref]$client)) { throw 'Client rectangle unavailable' }
  if (0 -ne [WidgetCenterObservation]::DwmGetWindowAttribute($hwnd, 9, [ref]$frame, [Runtime.InteropServices.Marshal]::SizeOf($frame))) { throw 'Visible frame unavailable' }
  $monitor = New-Object WidgetCenterObservation+MONITORINFO
  $monitor.Size = [Runtime.InteropServices.Marshal]::SizeOf($monitor)
  $monitorHandle = [WidgetCenterObservation]::MonitorFromWindow($hwnd, 2)
  if ($monitorHandle -eq [IntPtr]::Zero -or -not [WidgetCenterObservation]::GetMonitorInfo($monitorHandle, [ref]$monitor)) { throw 'Monitor information unavailable' }
  [pscustomobject]@{ observed_utc = [DateTime]::UtcNow.ToString('o'); pid = $widgetProcess.Id; exe_sha256 = $widgetHash; client_origin = $origin; client = $client; visible_frame = $frame; work = $monitor.Work } | ConvertTo-Json -Depth 5
} finally {
  $ownedProcess = Get-Process -Id $widgetProcess.Id -ErrorAction SilentlyContinue
  if ($null -ne $ownedProcess -and $ownedProcess.Path -eq $widgetExe) { Stop-Process -Id $ownedProcess.Id }
}
