$ErrorActionPreference = 'Stop'
$taskLog = Join-Path $env:TEMP 'giw-ipc-retry-20261120.log'
if (-not (Test-Path -LiteralPath $taskLog -PathType Leaf)) { throw 'Test log missing' }
$bytes = [IO.File]::ReadAllBytes($taskLog)
$sha = [Security.Cryptography.SHA256]::Create()
try { $digest = [BitConverter]::ToString($sha.ComputeHash($bytes)).Replace('-', '') } finally { $sha.Dispose() }
if ($digest -ne 'CE3D2D242F7FD2410A9992863793B898F19A44C547ADC7D5BA30A204343E88D3') { throw 'Test log hash mismatch' }
$stream = [IO.MemoryStream]::new($bytes)
$reader = [IO.StreamReader]::new($stream, [Text.Encoding]::UTF8, $true)
try { $content = $reader.ReadToEnd() } finally { $reader.Dispose(); $stream.Dispose() }
$pattern = '(?m)^test tests::windows_ipc_guard \.\.\. fixture_root=[^\r\n]+? test_service=(github-task-widget-ipc-test-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}) synthetic_id=([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\r?$'
$matchesFound = [regex]::Matches($content, $pattern)
if ($matchesFound.Count -ne 1) { throw 'Test identity not uniquely identified' }
$service = $matchesFound[0].Groups[1].Value
$identifier = $matchesFound[0].Groups[2].Value
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class GiwCredentialReadOnly {
    [DllImport("advapi32.dll", EntryPoint="CredReadW", CharSet=CharSet.Unicode, SetLastError=true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool Read(string target, uint type, uint flags, out IntPtr credential);
    [DllImport("advapi32.dll")]
    public static extern void CredFree(IntPtr credential);
}
'@
$ptr = [IntPtr]::Zero
$found = [GiwCredentialReadOnly]::Read(($identifier + '.' + $service), 1, 0, [ref]$ptr)
$errorCode = [Runtime.InteropServices.Marshal]::GetLastWin32Error()
if ($found) {
    [GiwCredentialReadOnly]::CredFree($ptr)
    'credential_state=present'
} elseif ($errorCode -eq 1168) {
    'credential_state=not_found_in_this_session'
} else {
    'credential_state=unknown'
}
