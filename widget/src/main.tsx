import React, { useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { invoke } from "@tauri-apps/api/core";
import "./style.css";

const appWindow = getCurrentWindow();
type ResizeDirection = Parameters<typeof appWindow.startResizeDragging>[0];
const directions: ResizeDirection[] = ["North", "South", "East", "West", "NorthEast", "NorthWest", "SouthEast", "SouthWest"];
type Connection = { owner: string; repo: string; has_token: boolean; revision: string };
type BoardIssue = { number: number; title: string; url: string; status: "todo" | "in-progress" | "done"; updated_at: string };
type IssueStatus = BoardIssue["status"];
type CreateResult = { kind: "created" | "partial" | "uncertain"; number: number | null; url: string | null; message: string; operation_id: string };
type PendingCreation = { operation_id: string; owner: string; repo: string; number: number | null; phase: "pending" | "created_open" | "partial" | "completed" };

export function App() {
  const [pinned, setPinned] = useState(true);
  const [error, setError] = useState("");
  const [windowError, setWindowError] = useState("");
  const [owner, setOwner] = useState("");
  const [repo, setRepo] = useState("");
  const [token, setToken] = useState("");
  const [hasToken, setHasToken] = useState<boolean | null>(null);
  const [connection, setConnection] = useState<Connection | null>(null);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState("");
  const [showSettings, setShowSettings] = useState(true);
  const [issues, setIssues] = useState<BoardIssue[]>([]);
  const [boardError, setBoardError] = useState("");
  const [moveError, setMoveError] = useState("");
  const [moving, setMoving] = useState(false);
  const movingRef = useRef(false);
  const draggedIssue = useRef<BoardIssue | null>(null);
  const [loading, setLoading] = useState(false);
  const [boardEpoch, setBoardEpoch] = useState(0);
  const [createOpen, setCreateOpen] = useState(false);
  const [createStatus, setCreateStatus] = useState<IssueStatus>("todo");
  const [createTitle, setCreateTitle] = useState("");
  const [createBody, setCreateBody] = useState("");
  const [creating, setCreating] = useState(false);
  const [createError, setCreateError] = useState("");
  const [createResult, setCreateResult] = useState<CreateResult | null>(null);
  const [resultTarget, setResultTarget] = useState<Connection | null>(null);
  const [pending, setPending] = useState<PendingCreation | null>(null);
  const [pendingReady, setPendingReady] = useState(false);
  const [pendingError, setPendingError] = useState("");
  const [acknowledging, setAcknowledging] = useState(false);
  const creatingRef = useRef(false);
  const acknowledgingRef = useRef(false);
  const resultRef = useRef<CreateResult | null>(null);
  const pendingRequestVersion = useRef(0);
  const requestVersion = useRef(0);
  const connectionRequestVersion = useRef(0);
  const settingsEdited = useRef({ owner: false, repo: false, token: false });
  const applyConnection = (value: Connection) => { setConnection(value); setOwner(value.owner); setRepo(value.repo); setHasToken(value.has_token); };
  const applyConnectionPreservingEdits = (value: Connection) => {
    setConnection(value); setHasToken(value.has_token);
    if (!settingsEdited.current.owner) setOwner(value.owner);
    if (!settingsEdited.current.repo) setRepo(value.repo);
  };
  useEffect(() => {
    const version = ++connectionRequestVersion.current;
    let active = true;
    void invoke<Connection>("connection_info").then(value => {
      if (!active || version !== connectionRequestVersion.current) return;
      applyConnectionPreservingEdits(value);
      if (!settingsEdited.current.owner && !settingsEdited.current.repo && !settingsEdited.current.token)
        setShowSettings(!(value.owner && value.repo && value.has_token));
    }).catch(() => {
      if (active && version === connectionRequestVersion.current) setError("接続設定を読み込めませんでした。");
    });
    return () => { active = false; };
  }, []);
  function openSettings() {
    if (createOpen || moving || creating || busy) return;
    requestVersion.current++; setIssues([]); setBoardError(""); setShowSettings(true);
    if (!showSettings) settingsEdited.current = { owner: false, repo: false, token: false };
    const version = ++connectionRequestVersion.current;
    void invoke<Connection>("connection_info").then(value => {
      if (version === connectionRequestVersion.current) { applyConnectionPreservingEdits(value); setError(""); }
    }).catch(() => {
      if (version !== connectionRequestVersion.current) return;
      setHasToken(null); setConnection(null); setError("接続設定を読み込めませんでした。");
    });
  }
  async function refreshPending() {
    const version = ++pendingRequestVersion.current;
    try {
      const value = await invoke<PendingCreation | null>("pending_creation");
      if (version !== pendingRequestVersion.current) return undefined;
      setPending(value);
      if (value && resultRef.current && value.operation_id !== resultRef.current.operation_id) {
        setPendingReady(false); setPendingError("別の作成操作の記録があります。それぞれの結果を確認してください。");
      } else { setPendingReady(true); setPendingError(""); }
      return value;
    } catch {
      if (version === pendingRequestVersion.current) { setPendingReady(false); setPendingError("以前の Issue 作成結果を確認できません。設定を確認し、再読み込みしてください。"); }
      return undefined;
    }
  }
  useEffect(() => { void refreshPending(); return () => { pendingRequestVersion.current++; }; }, []);
  useEffect(() => {
    if (showSettings || !hasToken || !connection) return;
    let active = true;
    let timer: number | undefined;
    const refresh = async () => {
      const version = ++requestVersion.current;
      setLoading(true);
      try {
        const next = await invoke<BoardIssue[]>("list_issues", { expectedOwner: connection.owner, expectedRepo: connection.repo, expectedRevision: connection.revision });
        if (active && version === requestVersion.current) { setIssues(next); setBoardError(""); }
      } catch (reason) {
        if (active && version === requestVersion.current) { setIssues([]); setBoardError(typeof reason === "string" ? reason : "Issue を取得できませんでした。"); }
      } finally {
        if (active && version === requestVersion.current) { setLoading(false); timer = window.setTimeout(refresh, 60_000); }
      }
    };
    void refresh();
    return () => { active = false; requestVersion.current++; window.clearTimeout(timer); };
  }, [showSettings, hasToken, boardEpoch, connection]);
  async function saveSettings(e: React.FormEvent) {
    e.preventDefault();
    if (!connection?.revision) { setError("接続設定を再読み込みしてください。"); return; }
    connectionRequestVersion.current++; setBusy(true); setNotice(""); requestVersion.current++; setIssues([]);
    try { const result = await invoke<Connection>("save_connection", {owner, repo, token, expectedRevision: connection.revision}); applyConnection(result); setToken(""); setError(""); setNotice("接続を確認し、設定を保存しました。"); setShowSettings(false); setBoardEpoch(value => value + 1); }
    catch (reason) {
      setError(typeof reason === "string" ? reason : "接続設定を保存できませんでした。");
      try { applyConnectionPreservingEdits(await invoke<Connection>("connection_info")); }
      catch { setHasToken(null); setError("保存後の状態を確認できません。接続設定を再確認してください。"); }
    }
    finally { setBusy(false); }
  }
  async function removeToken() {
    if (!connection?.revision) { setError("接続設定を再読み込みしてください。"); return; }
    connectionRequestVersion.current++; setBusy(true); setNotice("");
    try { const result = await invoke<Connection>("delete_token", { expectedRevision: connection.revision }); setToken(""); applyConnectionPreservingEdits(result); setError(""); setNotice("保存済みトークンを削除しました。"); }
    catch (reason) { setError(typeof reason === "string" ? reason : "トークンを削除できませんでした。"); }
    finally { setBusy(false); }
  }
  async function run(action: () => Promise<void>) {
    try { await action(); setWindowError(""); } catch { setWindowError("ウィンドウ操作に失敗しました。もう一度お試しください。"); }
  }
  async function togglePin() {
    const next = !pinned;
    await run(async () => { await appWindow.setAlwaysOnTop(next); setPinned(next); });
  }
  function openCreate(status: IssueStatus) {
    if (movingRef.current || !pendingReady || pending || resultRef.current) return;
    setCreateStatus(status); setCreateTitle(""); setCreateBody(""); setCreateError("");
    setCreateOpen(true);
  }
  async function moveIssue(issue: BoardIssue, status: IssueStatus) {
    if (!connection || movingRef.current || creatingRef.current || createOpen || issue.status === status) return;
    movingRef.current = true; setMoving(true); setMoveError("");
    try {
      await invoke("move_issue", { expectedOwner: connection.owner, expectedRepo: connection.repo, expectedRevision: connection.revision, number: issue.number, expectedUpdatedAt: issue.updated_at, status });
    } catch (reason) {
      setMoveError(typeof reason === "string" ? reason : "Issue を移動できませんでした。最新の状態を確認してください。");
    } finally {
      movingRef.current = false; setMoving(false); setBoardEpoch(value => value + 1);
    }
  }
  async function acknowledgePending() {
    if (!pending || !pendingReady || acknowledgingRef.current) return;
    acknowledgingRef.current = true; setAcknowledging(true);
    try {
      await invoke("acknowledge_creation", { operationId: pending.operation_id });
      pendingRequestVersion.current++; setPending(null); setPendingReady(false);
      if (resultRef.current?.operation_id === pending.operation_id) { resultRef.current = null; setCreateResult(null); setResultTarget(null); }
      setPendingError(""); await refreshPending();
    } catch { setPendingReady(false); setPendingError("作成結果の確認を記録できません。再読み込みしてください。"); }
    finally { acknowledgingRef.current = false; setAcknowledging(false); }
  }
  async function acknowledgeLocalResult() {
    if (!resultRef.current) return;
    resultRef.current = null; setCreateResult(null); setResultTarget(null); setPendingReady(false);
    await refreshPending();
  }
  async function submitIssue(e: React.FormEvent) {
    e.preventDefault();
    if (!connection || creatingRef.current || pending || resultRef.current || !pendingReady) return;
    if (!createTitle.trim()) { setCreateError("タイトルを入力してください。"); return; }
    creatingRef.current = true; setCreating(true); setCreateError("");
    try {
      const result = await invoke<CreateResult>("create_issue", { expectedOwner: connection.owner, expectedRepo: connection.repo, expectedRevision: connection.revision, title: createTitle, body: createBody, status: createStatus });
      resultRef.current = result; setCreateResult(result); setResultTarget(connection);
      setCreateOpen(false); setCreateTitle(""); setCreateBody(""); setPendingReady(false);
      const current = await refreshPending();
      if (!current || current.operation_id !== result.operation_id) { setPendingReady(false); setPendingError("作成結果の確認記録が一致しません。GitHub と記録を確認してください。"); }
      setBoardEpoch(value => value + 1);
    } catch (reason) {
      setCreateError(typeof reason === "string" ? reason : "Issue を作成できませんでした。");
      const current = await refreshPending();
      if (current || current === undefined) setCreateOpen(false);
    }
    finally { creatingRef.current = false; setCreating(false); }
  }
  const matchingResult = createResult && pending?.operation_id === createResult.operation_id ? createResult : null;
  const localResult = createResult && !matchingResult ? createResult : null;
  const pendingNumber = matchingResult?.number ?? pending?.number;
  const pendingUrl = matchingResult?.url ?? (pending ? `https://github.com/${pending.owner}/${pending.repo}/issues${pendingNumber ? `/${pendingNumber}` : ""}` : null);
  const localUrl = localResult?.url ?? (resultTarget ? `https://github.com/${resultTarget.owner}/${resultTarget.repo}/issues` : null);
  return <main>
    <header className="titlebar" onMouseDown={e => { if (e.button === 0 && !(e.target as HTMLElement).closest("button")) void run(() => appWindow.startDragging()); }}>
      <div className="brand"><span className="mark">●</span><strong>GitHub Issue Widget</strong><small>WORKSPACE</small></div>
      <nav>{!showSettings && <button disabled={moving || creating || !!pending || !!createResult || !pendingReady} onClick={() => openCreate("todo")}>Issue を作成</button>}<button disabled={moving || creating || busy || createOpen} onClick={openSettings}>設定</button><button aria-label={pinned ? "最前面固定を解除" : "最前面に固定"} aria-pressed={pinned} onClick={togglePin}>{pinned ? "固定中" : "固定"}</button><button aria-label="最小化" onClick={() => run(() => appWindow.minimize())}>−</button><button aria-label="閉じる" onClick={() => run(() => appWindow.close())}>×</button></nav>
    </header>
    {showSettings ? <section className="content"><div className="eyebrow">YOUR ISSUES, AT A GLANCE</div><h1>接続設定</h1><p className="description">GitHub リポジトリと Fine-grained personal access token を設定します。</p>
    <form onSubmit={saveSettings} className="settings-form">
      <label>オーナー<input disabled={busy} value={owner} onChange={e => { settingsEdited.current.owner = true; setOwner(e.target.value); }} required autoComplete="off" /></label>
      <label>リポジトリ<input disabled={busy} value={repo} onChange={e => { settingsEdited.current.repo = true; setRepo(e.target.value); }} required autoComplete="off" /></label>
      <label>トークン<input type="password" disabled={busy} value={token} onChange={e => { settingsEdited.current.token = true; setToken(e.target.value); }} autoComplete="off" placeholder={hasToken === null ? "保存状態を確認できません" : hasToken ? "空欄なら保存済みを使用" : "Fine-grained PAT"} /></label>
      <p>対象リポジトリに Metadata: Read と Issues: Read and write を付与してください。接続確認は Issue の書き込み権限を保証しません。</p>
      <div className="settings-actions"><button type="submit" disabled={busy || !connection?.revision}>接続を確認して保存</button><button type="button" disabled={busy || !hasToken || !connection?.revision} onClick={removeToken}>保存済みトークンを削除</button>{connection?.has_token && connection.owner && connection.repo && <button type="button" disabled={busy} onClick={() => { connectionRequestVersion.current++; applyConnection(connection); setShowSettings(false); }}>ボードに戻る</button>}<button type="button" className="link" onClick={() => run(() => openUrl("https://github.com/"))}>GitHub を開く ↗</button></div>
    </form>
    {notice && <p role="status">{notice}</p>}{error && <p role="alert" className="error">{error}</p>}{pendingError && <p role="alert" className="error">{pendingError} <button type="button" onClick={() => void refreshPending()}>再読み込み</button></p>}{windowError && <p role="alert" className="error">{windowError}</p>}</section> : <section className="board-view">
      <div className="board-heading"><div><strong>{connection?.owner} / {connection?.repo}</strong><small>{loading ? " 同期中…" : " 完了後60秒ごとに同期"}</small></div><button disabled={loading} onClick={() => setBoardEpoch(value => value + 1)}>更新</button></div>
      {boardError && <p role="alert" className="error">{boardError}</p>}{moveError && <p role="alert" className="error">{moveError}</p>}{moving && <p role="status">Issue を移動しています…</p>}{windowError && <p role="alert" className="error">{windowError}</p>}{pendingError && <p role="alert" className="error">{pendingError} <button onClick={() => void refreshPending()}>再読み込み</button></p>}
      {localResult && <div role="status" className="creation-result"><strong>直前の作成結果</strong><span>{localResult.message}</span><span>{resultTarget?.owner} / {resultTarget?.repo}</span>{localUrl && <button className="link" onClick={() => run(() => openUrl(localUrl))}>{localResult.number ? `#${localResult.number} を GitHub で開く` : "GitHub で確認する ↗"}</button>}<button onClick={() => void acknowledgeLocalResult()}>直前の結果を確認しました</button></div>}
      {pending && <div role="status" className="creation-result"><strong>{localResult ? "別の確認待ち操作" : "作成結果"}</strong><span>{matchingResult?.message ?? (pending.phase === "completed" ? "前回の Issue は作成されました。" : "前回の Issue 作成結果を GitHub で確認してください。")}</span><span>{pending.owner} / {pending.repo}</span>{pendingUrl && <button className="link" onClick={() => run(() => openUrl(pendingUrl))}>{pendingNumber ? `#${pendingNumber} を GitHub で開く` : "GitHub で確認する ↗"}</button>}<button disabled={!pendingReady || acknowledging} onClick={() => void acknowledgePending()}>確認しました</button></div>}
      <div className="board-columns">{([ ["todo", "Todo"], ["in-progress", "In Progress"], ["done", "Done"] ] as const).map(([status, label]) => <section key={status} className="board-column" onDragOver={e => { if (draggedIssue.current && !moving) e.preventDefault(); }} onDrop={e => { e.preventDefault(); const issue = draggedIssue.current; draggedIssue.current = null; if (issue) void moveIssue(issue, status); }}><h2>{label} <small>{issues.filter(issue => issue.status === status).length}</small><button aria-label={`${label} に Issue を作成`} disabled={moving || creating || !!pending || !!createResult || !pendingReady} onClick={() => openCreate(status)}>＋</button></h2>{issues.filter(issue => issue.status === status).map(issue => <button key={issue.number} className="issue-card" draggable={!moving && !creating && !createOpen} onDragStart={e => { draggedIssue.current = issue; e.dataTransfer.effectAllowed = "move"; e.dataTransfer.setData("text/plain", String(issue.number)); }} onDragEnd={() => { draggedIssue.current = null; }} onClick={() => run(() => openUrl(issue.url))}><small>#{issue.number}</small><span>{issue.title}</span></button>)}</section>)}</div>
    </section>}
    {createOpen && !showSettings && <div className="modal-overlay"><section role="dialog" aria-modal="true" aria-labelledby="create-heading" className="create-dialog"><h2 id="create-heading">Issue を作成</h2><p>作成先: {connection?.owner} / {connection?.repo}</p><form onSubmit={submitIssue} className="create-form"><label>タイトル<input required autoFocus value={createTitle} onChange={e => setCreateTitle(e.target.value)} disabled={creating} /></label><label>本文<textarea value={createBody} onChange={e => setCreateBody(e.target.value)} disabled={creating} /></label><label>ステータス<select value={createStatus} onChange={e => setCreateStatus(e.target.value as IssueStatus)} disabled={creating}><option value="todo">Todo</option><option value="in-progress">In Progress</option><option value="done">Done</option></select></label>{createError && <p role="alert" className="error">{createError}</p>}<div className="settings-actions"><button type="submit" disabled={creating}>作成</button><button type="button" disabled={creating} onClick={() => setCreateOpen(false)}>キャンセル</button></div></form></section></div>}
    {directions.map(direction => <div key={direction} aria-hidden="true" className={`resize resize-${direction.toLowerCase()}`} onMouseDown={e => {if (e.button === 0) void run(() => appWindow.startResizeDragging(direction));}}/>)}
  </main>;
}
const rootElement = document.getElementById("root");
if (rootElement) createRoot(rootElement).render(<React.StrictMode><App /></React.StrictMode>);
