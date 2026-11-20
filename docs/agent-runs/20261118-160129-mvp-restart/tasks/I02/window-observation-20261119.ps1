$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class WidgetWindowObservation {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
  [StructLayout(LayoutKind.Sequential)] public struct MONITORINFO { public int Size; public RECT Monitor, Work; public uint Flags; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT rect);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT rect);
  [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
  [DllImport("user32.dll", EntryPoint="GetWindowLongW")] public static extern int GetWindowLong(IntPtr h, int index);
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
  $outer = New-Object WidgetWindowObservation+RECT
  $client = New-Object WidgetWindowObservation+RECT
  if (-not [WidgetWindowObservation]::GetWindowRect($hwnd, [ref]$outer)) { throw 'GetWindowRect failed' }
  if (-not [WidgetWindowObservation]::GetClientRect($hwnd, [ref]$client)) { throw 'GetClientRect failed' }
  $dpi = [WidgetWindowObservation]::GetDpiForWindow($hwnd)
  if ($dpi -eq 0) { throw 'DPI unavailable' }
  $monitor = New-Object WidgetWindowObservation+MONITORINFO
  $monitor.Size = [Runtime.InteropServices.Marshal]::SizeOf($monitor)
  $monitorHandle = [WidgetWindowObservation]::MonitorFromWindow($hwnd, 2)
  if ($monitorHandle -eq [IntPtr]::Zero -or -not [WidgetWindowObservation]::GetMonitorInfo($monitorHandle, [ref]$monitor)) { throw 'Monitor information unavailable' }
  $extendedStyle = [WidgetWindowObservation]::GetWindowLong($hwnd, -20)
  [pscustomobject]@{
    observed_utc = [DateTime]::UtcNow.ToString('o'); pid = $widgetProcess.Id; exe_sha256 = $widgetHash; dpi = $dpi
    outer = $outer; client = $client; monitor = $monitor.Monitor; work = $monitor.Work
    topmost = (($extendedStyle -band 8) -ne 0)
  } | ConvertTo-Json -Depth 5
} finally {
  $ownedProcess = Get-Process -Id $widgetProcess.Id -ErrorAction SilentlyContinue
  if ($null -ne $ownedProcess -and $ownedProcess.Path -eq $widgetExe) { Stop-Process -Id $ownedProcess.Id }
}
