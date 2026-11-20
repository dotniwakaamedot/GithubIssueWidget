$ErrorActionPreference = 'Stop'

function Invoke-TaskProcess([string]$FileName, [string]$Arguments, [int]$TimeoutMs) {
    $taskInfo = [Diagnostics.ProcessStartInfo]::new()
    $taskInfo.FileName = $FileName
    $taskInfo.Arguments = $Arguments
    $taskInfo.UseShellExecute = $false
    $taskInfo.CreateNoWindow = $true
    $taskInfo.RedirectStandardOutput = $true
    $taskInfo.RedirectStandardError = $true
    $taskProcess = [Diagnostics.Process]::new()
    $taskProcess.StartInfo = $taskInfo
    try {
        if (-not $taskProcess.Start()) { throw 'Process did not start' }
        $taskStdout = $taskProcess.StandardOutput.ReadToEndAsync()
        $taskStderr = $taskProcess.StandardError.ReadToEndAsync()
        if (-not $taskProcess.WaitForExit($TimeoutMs)) {
            $taskProcess.Kill()
            $taskProcess.WaitForExit()
            throw 'Process timed out'
        }
        if (-not [Threading.Tasks.Task]::WaitAll([Threading.Tasks.Task[]]@($taskStdout, $taskStderr), 5000)) {
            throw 'Process output did not finish'
        }
        return [pscustomobject]@{
            ExitCode = $taskProcess.ExitCode
            Stdout = $taskStdout.Result
            Stderr = $taskStderr.Result
        }
    } finally {
        $taskProcess.Dispose()
    }
}

$taskKeys = @('GIW_READONLY_OWNER', 'GIW_READONLY_REPO', 'GIW_READONLY_TOKEN')
$taskPrevious = @{}
foreach ($taskKey in $taskKeys) {
    $taskPrevious[$taskKey] = [Environment]::GetEnvironmentVariable($taskKey, 'Process')
}
$taskExit = 1
try {
    $taskBinary = Join-Path (Get-Location).Path 'target\debug\deps\github_issue_widget-fc25f1eb6ae85386.exe'
    if ((Get-FileHash -LiteralPath $taskBinary -Algorithm SHA256).Hash -ne 'F225011E1F911F1D0056D2519CCFA2B576FCFA3261675880F467C28EA2E2A811') {
        throw 'Unexpected test executable'
    }
    $taskAuth = Invoke-TaskProcess 'gh.exe' 'auth token --hostname github.com' 30000
    if ($taskAuth.ExitCode -ne 0 -or [string]::IsNullOrWhiteSpace($taskAuth.Stdout)) {
        Write-Output 'credential_unavailable'
    } else {
        $taskToken = $taskAuth.Stdout.Trim()
        [Environment]::SetEnvironmentVariable('GIW_READONLY_OWNER', 'dotniwakaamedot', 'Process')
        [Environment]::SetEnvironmentVariable('GIW_READONLY_REPO', 'GithubIssueWidget', 'Process')
        [Environment]::SetEnvironmentVariable('GIW_READONLY_TOKEN', $taskToken, 'Process')
        $taskRun = Invoke-TaskProcess $taskBinary 'tests::authenticated_github_readonly_repository_and_issues --ignored --exact --nocapture' 60000
        $taskExit = $taskRun.ExitCode
        if ($taskRun.Stdout.Contains($taskToken) -or $taskRun.Stderr.Contains($taskToken)) {
            Write-Output 'output_suppressed_sensitive_value_detected'
            $taskExit = 2
        } else {
            $taskCount = [regex]::Matches($taskRun.Stdout, 'repository_and_issues_read_ok count=([0-9]+)')
            $taskPassed = [regex]::Matches($taskRun.Stdout, '(?m)^test result: ok\. 1 passed; 0 failed; 0 ignored;')
            if ($taskExit -eq 0 -and ($taskPassed.Count -ne 1 -or $taskCount.Count -ne 1)) { $taskExit = 3 }
            Write-Output ('test_exit=' + $taskExit)
            if ($taskExit -eq 0) { Write-Output ('issue_count=' + $taskCount[0].Groups[1].Value) }
        }
    }
} catch {
    Write-Output 'local_process_failed'
    $taskExit = 1
} finally {
    foreach ($taskKey in $taskKeys) {
        [Environment]::SetEnvironmentVariable($taskKey, $taskPrevious[$taskKey], 'Process')
    }
    $taskToken = $null
    $taskAuth = $null
    $taskRun = $null
}
exit $taskExit
