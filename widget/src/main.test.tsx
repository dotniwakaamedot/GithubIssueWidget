// @vitest-environment jsdom
import React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";

const mocks = vi.hoisted(() => ({
  invoke: vi.fn(),
  setAlwaysOnTop: vi.fn().mockResolvedValue(undefined),
  startDragging: vi.fn().mockResolvedValue(undefined),
  startResizeDragging: vi.fn().mockResolvedValue(undefined),
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ setAlwaysOnTop: mocks.setAlwaysOnTop, startDragging: mocks.startDragging, startResizeDragging: mocks.startResizeDragging }) }));
vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));

import { App } from "./main";

afterEach(() => { cleanup(); vi.clearAllMocks(); vi.useRealTimers(); });

describe("window input", () => {
  it("starts a window drag only for a left click outside the titlebar buttons", async () => {
    mocks.invoke.mockImplementation((command: string) => Promise.resolve(command === "connection_info" ? { owner: "", repo: "", has_token: false, revision: "revision-1" } : null));
    render(<App />);
    await screen.findByText("接続設定");
    const header = screen.getByText("GitHub Issue Widget").closest("header")!;
    fireEvent.mouseDown(header, { button: 2 });
    fireEvent.mouseDown(screen.getByRole("button", { name: "最小化" }), { button: 0 });
    expect(mocks.startDragging).not.toHaveBeenCalled();
    fireEvent.mouseDown(header, { button: 0 });
    expect(mocks.startDragging).toHaveBeenCalledTimes(1);
  });

  it("requests the matching resize direction for a left click on the edge", async () => {
    mocks.invoke.mockImplementation((command: string) => Promise.resolve(command === "connection_info" ? { owner: "", repo: "", has_token: false, revision: "revision-1" } : null));
    const { container } = render(<App />);
    await screen.findByText("接続設定");
    const corner = container.querySelector(".resize-southeast")!;
    fireEvent.mouseDown(corner, { button: 2 });
    expect(mocks.startResizeDragging).not.toHaveBeenCalled();
    fireEvent.mouseDown(corner, { button: 0 });
    expect(mocks.startResizeDragging).toHaveBeenCalledWith("SouthEast");
  });
});

describe("moving issues", () => {
  it("shows the actual GitHub state even when both issues are in Done", async () => {
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "a", repo: "b", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([
        { number: 1, title: "open in done", url: "https://github.com/a/b/issues/1", status: "done", state: "open", updated_at: "v1" },
        { number: 2, title: "closed in done", url: "https://github.com/a/b/issues/2", status: "done", state: "closed", updated_at: "v2" },
      ]);
      throw new Error("unexpected command");
    });
    render(<App />);
    const open = await screen.findByText("open in done");
    const closed = screen.getByText("closed in done");
    expect(open.closest("section")?.textContent).toContain("Done");
    expect(open.closest("button")?.textContent).toContain("Open");
    expect(closed.closest("button")?.textContent).toContain("Closed");
  });

  it.each([
    ["todo", "Todo", "in-progress", "In Progress"], ["todo", "Todo", "done", "Done"],
    ["in-progress", "In Progress", "todo", "Todo"], ["in-progress", "In Progress", "done", "Done"],
    ["done", "Done", "todo", "Todo"], ["done", "Done", "in-progress", "In Progress"],
  ])("moves %s to %s via a refreshed board", async (source, sourceName, target, targetName) => {
    let lists = 0;
    const original = { number: 42, title: "move me", url: "https://github.com/a/b/issues/42", status: source, state: source === "done" ? "closed" : "open", updated_at: "v1" };
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "a", repo: "b", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([{ ...original, status: ++lists === 1 ? source : target }]);
      if (command === "move_issue") return Promise.resolve();
      throw new Error("unexpected command");
    });
    render(<App />);
    const card = await screen.findByText("move me");
    expect(card.closest("section")?.textContent).toContain(sourceName);
    const transfer = { effectAllowed: "", setData: vi.fn() };
    fireEvent.dragStart(card.closest("button")!, { dataTransfer: transfer });
    fireEvent.drop(screen.getByText(targetName).closest("section")!, { dataTransfer: transfer });
    await waitFor(() => expect(screen.getByText("move me").closest("section")?.textContent).toContain(targetName));
    expect(lists).toBe(2);
    expect(mocks.invoke).toHaveBeenCalledWith("move_issue", { expectedOwner: "a", expectedRepo: "b", expectedRevision: "revision-1", number: 42, expectedUpdatedAt: "v1", status: target });
  });

  it("refreshes after an uncertain move and keeps the error visible", async () => {
    let lists = 0;
    const original = { number: 42, title: "move me", url: "https://github.com/a/b/issues/42", status: "todo", state: "open", updated_at: "v1" };
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "a", repo: "b", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") { lists++; return Promise.resolve([lists === 1 ? original : { ...original, status: "done" }]); }
      if (command === "move_issue") return Promise.reject("更新結果は不明です。");
      throw new Error("unexpected command");
    });
    render(<App />);
    const card = await screen.findByText("move me");
    const transfer = { effectAllowed: "", setData: vi.fn() };
    fireEvent.dragStart(card.closest("button")!, { dataTransfer: transfer });
    fireEvent.drop(screen.getByText("Done").closest("section")!, { dataTransfer: transfer });
    await screen.findByText("更新結果は不明です。");
    await waitFor(() => expect(lists).toBe(2));
    expect(mocks.invoke).toHaveBeenCalledWith("move_issue", { expectedOwner: "a", expectedRepo: "b", expectedRevision: "revision-1", number: 42, expectedUpdatedAt: "v1", status: "done" });
    expect(screen.getByText("move me").closest("section")?.textContent).toContain("Done");
    expect(screen.getByText("move me").closest("button")?.textContent).toContain("Open");
    expect(screen.getByText("更新結果は不明です。")).toBeTruthy();
  });
});

describe("connection settings", () => {
  it("sends the displayed revision and keeps the saved token state on a stale delete", async () => {
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "delete_token") return Promise.reject("接続設定が変更されました。");
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    await waitFor(() => expect(mocks.invoke.mock.calls.filter(([command]) => command === "connection_info").length).toBe(2));
    fireEvent.click(screen.getByText("保存済みトークンを削除"));
    await screen.findByText("接続設定が変更されました。");
    expect(mocks.invoke).toHaveBeenCalledWith("delete_token", { expectedRevision: "revision-1" });
    expect(screen.queryByText("保存済みトークンを削除しました。")).toBeNull();
    expect((screen.getByText("保存済みトークンを削除") as HTMLButtonElement).disabled).toBe(false);
  });

  it("uses the new revision after deleting a saved token", async () => {
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "delete_token") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: false, revision: "revision-2" });
      if (command === "save_connection") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-3" });
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    await waitFor(() => expect(mocks.invoke.mock.calls.filter(([command]) => command === "connection_info").length).toBe(2));
    fireEvent.click(screen.getByText("保存済みトークンを削除"));
    await screen.findByText("保存済みトークンを削除しました。");
    fireEvent.change(screen.getByLabelText("トークン"), { target: { value: "synthetic-token" } });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("old-owner / old-repo");
    expect(mocks.invoke).toHaveBeenCalledWith("save_connection", expect.objectContaining({ expectedRevision: "revision-2" }));
  });

  it("keeps the attempted connection after a failed save refreshes saved settings", async () => {
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "save_connection") return Promise.reject("接続できませんでした。");
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    await waitFor(() => expect(mocks.invoke.mock.calls.filter(([command]) => command === "connection_info").length).toBe(2));
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "new-owner" } });
    fireEvent.change(screen.getByLabelText("リポジトリ"), { target: { value: "new-repo" } });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("接続できませんでした。");
    await waitFor(() => expect((screen.getByText("接続を確認して保存") as HTMLButtonElement).disabled).toBe(false));
    expect((screen.getByLabelText("オーナー") as HTMLInputElement).value).toBe("new-owner");
    expect((screen.getByLabelText("リポジトリ") as HTMLInputElement).value).toBe("new-repo");
  });

  it("prevents settings changes and returning to the board while a save is pending", async () => {
    let resolveSave!: (value: unknown) => void;
    const save = new Promise(resolve => { resolveSave = resolve; });
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "save_connection") return save;
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    fireEvent.click(screen.getByText("接続を確認して保存"));
    expect((screen.getByLabelText("オーナー") as HTMLInputElement).disabled).toBe(true);
    expect((screen.getByLabelText("リポジトリ") as HTMLInputElement).disabled).toBe(true);
    expect((screen.getByLabelText("トークン") as HTMLInputElement).disabled).toBe(true);
    expect((screen.getByText("ボードに戻る") as HTMLButtonElement).disabled).toBe(true);
    expect((screen.getByText("設定") as HTMLButtonElement).disabled).toBe(true);
    await act(async () => { resolveSave({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }); await save; });
    await screen.findByText("old-owner / old-repo");
  });

  it("keeps edits when retrying a settings read", async () => {
    let reads = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return ++reads === 1 ? Promise.reject("unavailable") : Promise.resolve({ owner: "saved-owner", repo: "saved-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("接続設定を読み込めませんでした。");
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "draft-owner" } });
    fireEvent.click(screen.getByText("設定"));
    await waitFor(() => expect((screen.getByLabelText("リポジトリ") as HTMLInputElement).value).toBe("saved-repo"));
    expect((screen.getByLabelText("オーナー") as HTMLInputElement).value).toBe("draft-owner");
    expect(screen.queryByText("接続設定を読み込めませんでした。")).toBeNull();
  });

  it("ignores a settings read after returning to the board", async () => {
    let resolveReload!: (value: unknown) => void;
    const reload = new Promise(resolve => { resolveReload = resolve; });
    let reads = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return ++reads === 1 ? Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }) : reload;
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    fireEvent.click(screen.getByText("ボードに戻る"));
    await act(async () => { resolveReload({ owner: "other-owner", repo: "other-repo", has_token: true, revision: "revision-1" }); await reload; });
    expect(screen.getByText("old-owner / old-repo")).toBeTruthy();
    expect(screen.queryByText("other-owner / other-repo")).toBeNull();
  });

  it("preserves edits while reopening settings loads the saved connection", async () => {
    let resolveReload!: (value: unknown) => void;
    const reload = new Promise(resolve => { resolveReload = resolve; });
    let reads = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return ++reads === 1 ? Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }) : reload;
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "draft-owner" } });
    await act(async () => { resolveReload({ owner: "old-owner", repo: "fresh-repo", has_token: true, revision: "revision-1" }); await reload; });
    expect((screen.getByLabelText("オーナー") as HTMLInputElement).value).toBe("draft-owner");
    expect((screen.getByLabelText("リポジトリ") as HTMLInputElement).value).toBe("fresh-repo");
  });

  it("ignores a settings reload that finishes after saving a new connection", async () => {
    let resolveReload!: (value: unknown) => void;
    const reload = new Promise(resolve => { resolveReload = resolve; });
    let reads = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return ++reads === 1 ? Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }) : reload;
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "save_connection") return Promise.resolve({ owner: "new-owner", repo: "new-repo", has_token: true, revision: "revision-1" });
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "new-owner" } });
    fireEvent.change(screen.getByLabelText("リポジトリ"), { target: { value: "new-repo" } });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("new-owner / new-repo");
    await act(async () => { resolveReload({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }); await reload; });
    expect(screen.getByText("new-owner / new-repo")).toBeTruthy();
    expect(screen.queryByText("old-owner / old-repo")).toBeNull();
  });

  it("keeps in-progress edits visible when initial settings arrive", async () => {
    let resolveInitial!: (value: unknown) => void;
    const initial = new Promise(resolve => { resolveInitial = resolve; });
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return initial;
      if (command === "pending_creation") return Promise.resolve(null);
      throw new Error("unexpected command");
    });
    render(<App />);
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "draft-owner" } });
    fireEvent.change(screen.getByLabelText("トークン"), { target: { value: "draft-token" } });
    await act(async () => { resolveInitial({ owner: "saved-owner", repo: "saved-repo", has_token: true, revision: "revision-1" }); await initial; });
    expect((screen.getByLabelText("オーナー") as HTMLInputElement).value).toBe("draft-owner");
    expect((screen.getByLabelText("リポジトリ") as HTMLInputElement).value).toBe("saved-repo");
    expect((screen.getByLabelText("トークン") as HTMLInputElement).value).toBe("draft-token");
    expect(screen.getByText("接続設定")).toBeTruthy();
  });

  it("waits for the initial connection revision before allowing a save", async () => {
    let resolveInitial!: (value: unknown) => void;
    const initial = new Promise(resolve => { resolveInitial = resolve; });
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return initial;
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "save_connection") return Promise.resolve({ owner: "new-owner", repo: "new-repo", has_token: true, revision: "revision-1" });
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "new-owner" } });
    fireEvent.change(screen.getByLabelText("リポジトリ"), { target: { value: "new-repo" } });
    expect((screen.getByText("接続を確認して保存") as HTMLButtonElement).disabled).toBe(true);
    await act(async () => { resolveInitial({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" }); await initial; });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("new-owner / new-repo");
    expect(mocks.invoke).toHaveBeenCalledWith("save_connection", expect.objectContaining({ expectedRevision: "revision-1" }));
    expect(screen.queryByText("old-owner / old-repo")).toBeNull();
  });

  it("keeps a settings error after an unrelated window action succeeds", async () => {
    mocks.invoke.mockImplementation((command: string) => command === "connection_info"
      ? Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" })
      : command === "pending_creation" ? Promise.resolve(null) : command === "list_issues" ? Promise.resolve([]) : Promise.reject("保存時に問題が発生しました"));
    render(<App />);
    await screen.findByText("test-owner / test-repo");
    fireEvent.click(screen.getByText("設定"));
    await screen.findByDisplayValue("test-owner");
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("保存時に問題が発生しました");
    fireEvent.click(screen.getByRole("button", { name: "最前面固定を解除" }));
    await waitFor(() => expect(mocks.setAlwaysOnTop).toHaveBeenCalledWith(false));
    expect(screen.getByText("保存時に問題が発生しました")).toBeTruthy();
  });

  it("ignores an old repository response after switching connection", async () => {
    let resolveOld!: (value: unknown) => void;
    const oldResponse = new Promise(resolve => { resolveOld = resolve; });
    let calls = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "save_connection") return Promise.resolve({ owner: "new-owner", repo: "new-repo", has_token: true, revision: "revision-1" });
      if (command === "list_issues") return ++calls === 1 ? oldResponse : Promise.resolve([{ number: 2, title: "new issue", url: "https://github.com/new-owner/new-repo/issues/2", status: "todo" }]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("old-owner / old-repo");
    fireEvent.click(screen.getByText("設定"));
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "new-owner" } });
    fireEvent.change(screen.getByLabelText("リポジトリ"), { target: { value: "new-repo" } });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("new issue");
    await act(async () => { resolveOld([{ number: 1, title: "old issue", url: "https://github.com/old-owner/old-repo/issues/1", status: "todo" }]); await oldResponse; });
    expect(screen.queryByText("old issue")).toBeNull();
    expect(screen.getByText("new issue")).toBeTruthy();
  });

  it("refreshes the active board after sixty seconds", async () => {
    vi.useFakeTimers();
    mocks.invoke.mockImplementation((command: string) => command === "connection_info"
      ? Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" })
      : command === "pending_creation" ? Promise.resolve(null) : Promise.resolve([]));
    await act(async () => { render(<App />); });
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "list_issues")).toHaveLength(1);
    await act(async () => { await vi.advanceTimersByTimeAsync(60_000); });
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "list_issues")).toHaveLength(2);
  });

  it("does not overlap refreshes when a request takes longer than sixty seconds", async () => {
    vi.useFakeTimers();
    let finish!: (value: unknown) => void;
    const slow = new Promise(resolve => { finish = resolve; });
    mocks.invoke.mockImplementation((command: string) => command === "connection_info"
      ? Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" })
      : command === "pending_creation" ? Promise.resolve(null) : slow);
    await act(async () => { render(<App />); });
    await act(async () => { await vi.advanceTimersByTimeAsync(120_000); });
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "list_issues")).toHaveLength(1);
    await act(async () => { finish([]); await slow; });
    await act(async () => { await vi.advanceTimersByTimeAsync(60_000); });
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "list_issues")).toHaveLength(2);
  });

  it("clears the board when another instance changes the connection", async () => {
    let listCalls = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "old-owner", repo: "old-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return ++listCalls === 1
        ? Promise.resolve([{ number: 42, title: "previous issue", url: "https://github.com/a/b/issues/42", status: "todo", state: "open", updated_at: "v1" }])
        : Promise.reject("別のウィンドウで接続先が変更されました。");
      throw new Error("unexpected command");
    });
    render(<App />);
    expect(await screen.findByText("previous issue")).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "更新" }));
    expect(await screen.findByText("別のウィンドウで接続先が変更されました。")).toBeTruthy();
    expect(screen.queryByText("previous issue")).toBeNull();
  });

  it("keeps the creation destination fixed while a draft is open", async () => {
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "owner-a", repo: "repo-a", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(null);
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "save_connection") return Promise.resolve({ owner: "owner-b", repo: "repo-b", has_token: true, revision: "revision-2" });
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("owner-a / repo-a");
    const createButton = screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement;
    await waitFor(() => expect(createButton.disabled).toBe(false));
    fireEvent.click(createButton);
    expect(screen.getByRole("dialog").textContent).toContain("作成先: owner-a / repo-a");
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "draft title" } });
    fireEvent.change(screen.getByLabelText("本文"), { target: { value: "draft body" } });
    const settings = screen.getByRole("button", { name: "設定" }) as HTMLButtonElement;
    expect(settings.disabled).toBe(true);
    fireEvent.click(settings);
    expect((screen.getByLabelText("本文") as HTMLTextAreaElement).value).toBe("draft body");
    fireEvent.click(screen.getByRole("button", { name: "キャンセル" }));
    expect(settings.disabled).toBe(false);
    fireEvent.click(settings);
    expect(await screen.findByRole("heading", { name: "接続設定" })).toBeTruthy();
    fireEvent.change(screen.getByLabelText("オーナー"), { target: { value: "owner-b" } });
    fireEvent.change(screen.getByLabelText("リポジトリ"), { target: { value: "repo-b" } });
    fireEvent.click(screen.getByText("接続を確認して保存"));
    await screen.findByText("owner-b / repo-b");
    const newCreateButton = screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement;
    await waitFor(() => expect(newCreateButton.disabled).toBe(false));
    fireEvent.click(newCreateButton);
    expect(screen.getByRole("dialog").textContent).toContain("作成先: owner-b / repo-b");
    expect((screen.getByLabelText("タイトル") as HTMLInputElement).value).toBe("");
    expect((screen.getByLabelText("本文") as HTMLTextAreaElement).value).toBe("");
  });

  it("shows an already created issue when closing a Done issue fails", async () => {
    let pending = false;
    let lists = 0;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(pending ? { operation_id: "test-id", owner: "test-owner", repo: "test-repo", number: 123, phase: "partial" } : null);
      if (command === "list_issues") {
        lists++;
        return Promise.resolve(pending ? [{ number: 123, title: "test title", url: "https://github.com/test-owner/test-repo/issues/123", status: "done", state: "open", updated_at: "v1" }] : []);
      }
      if (command === "create_issue") { pending = true; return Promise.resolve({ kind: "partial", number: 123, url: "https://github.com/test-owner/test-repo/issues/123", message: "Issue は作成されましたがクローズできませんでした。", operation_id: "test-id" }); }
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("test-owner / test-repo");
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledWith("list_issues", { expectedOwner: "test-owner", expectedRepo: "test-repo", expectedRevision: "revision-1" }));
    fireEvent.click(screen.getByRole("button", { name: "Done に Issue を作成" }));
    expect((screen.getByLabelText("ステータス") as HTMLSelectElement).value).toBe("done");
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "test title" } });
    fireEvent.click(screen.getByRole("button", { name: "作成" }));
    expect(await screen.findByText(/Issue は作成されましたが/)).toBeTruthy();
    expect(screen.getByRole("button", { name: "#123 を GitHub で開く" })).toBeTruthy();
    const card = await screen.findByText("test title");
    expect(lists).toBeGreaterThanOrEqual(2);
    expect(card.closest("section")?.textContent).toContain("Done");
    expect(card.closest("button")?.textContent).toContain("Open");
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "create_issue")).toHaveLength(1);
    expect(mocks.invoke).toHaveBeenCalledWith("create_issue", expect.objectContaining({ expectedRevision: "revision-1" }));
  });

  it("clears stale cards if refreshing after a partial creation fails", async () => {
    let pending = false;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(pending ? { operation_id: "test-id", owner: "test-owner", repo: "test-repo", number: 123, phase: "partial" } : null);
      if (command === "list_issues") return pending ? Promise.reject("一覧を取得できませんでした。") : Promise.resolve([{ number: 42, title: "stale issue", url: "https://github.com/test-owner/test-repo/issues/42", status: "done", state: "closed", updated_at: "v1" }]);
      if (command === "create_issue") { pending = true; return Promise.resolve({ kind: "partial", number: 123, url: "https://github.com/test-owner/test-repo/issues/123", message: "Issue は作成されましたがクローズできませんでした。", operation_id: "test-id" }); }
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("stale issue");
    fireEvent.click(screen.getByRole("button", { name: "Done に Issue を作成" }));
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "test title" } });
    fireEvent.click(screen.getByRole("button", { name: "作成" }));
    expect(await screen.findByText("一覧を取得できませんでした。")).toBeTruthy();
    expect(screen.queryByText("stale issue")).toBeNull();
    expect(screen.getByRole("button", { name: "#123 を GitHub で開く" })).toBeTruthy();
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "create_issue")).toHaveLength(1);
  });

  it("requires confirmation before retrying an unknown creation result", async () => {
    let pending = false;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(pending ? { operation_id: "test-id", owner: "test-owner", repo: "test-repo", number: null, phase: "pending" } : null);
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "create_issue") { pending = true; return Promise.resolve({ kind: "uncertain", number: null, url: null, message: "作成結果は不明です。", operation_id: "test-id" }); }
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("test-owner / test-repo");
    fireEvent.click(screen.getByRole("button", { name: "Issue を作成" }));
    expect((screen.getByLabelText("ステータス") as HTMLSelectElement).value).toBe("todo");
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "test title" } });
    fireEvent.click(screen.getByRole("button", { name: "作成" }));
    await screen.findByText("作成結果は不明です。");
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "create_issue")).toHaveLength(1);
  });

  it("restores the pending operation after restart and unlocks only after acknowledgement", async () => {
    let pending = true;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(pending ? { operation_id: "pending-id", owner: "test-owner", repo: "test-repo", number: 123, phase: "created_open" } : null);
      if (command === "acknowledge_creation") { pending = false; return Promise.resolve(); }
      if (command === "list_issues") return Promise.resolve([]);
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByRole("button", { name: "#123 を GitHub で開く" });
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "確認しました" }));
    await waitFor(() => expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(false));
    expect(mocks.invoke).toHaveBeenCalledWith("acknowledge_creation", { operationId: "pending-id" });
  });

  it.each(["stale", "unreadable"])("shows a known issue number when the pending record is %s", async mode => {
    let created = false;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "test-owner", repo: "test-repo", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") {
        if (!created) return Promise.resolve(null);
        if (mode === "unreadable") return Promise.reject("record unavailable");
        return Promise.resolve({ operation_id: "test-id", owner: "test-owner", repo: "test-repo", number: null, phase: "pending" });
      }
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "create_issue") { created = true; return Promise.resolve({ kind: "partial", number: 123, url: "https://github.com/test-owner/test-repo/issues/123", message: "確認記録を更新できません。", operation_id: "test-id" }); }
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("test-owner / test-repo");
    fireEvent.click(screen.getByRole("button", { name: "Issue を作成" }));
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "test title" } });
    fireEvent.click(screen.getByRole("button", { name: "作成" }));
    expect(await screen.findByRole("button", { name: "#123 を GitHub で開く" })).toBeTruthy();
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "create_issue")).toHaveLength(1);
  });

  it("keeps the local result separate from a different pending operation", async () => {
    let created = false;
    let otherPending = true;
    mocks.invoke.mockImplementation((command: string) => {
      if (command === "connection_info") return Promise.resolve({ owner: "owner-a", repo: "repo-a", has_token: true, revision: "revision-1" });
      if (command === "pending_creation") return Promise.resolve(created && otherPending ? { operation_id: "operation-b", owner: "owner-b", repo: "repo-b", number: 456, phase: "partial" } : null);
      if (command === "list_issues") return Promise.resolve([]);
      if (command === "create_issue") { created = true; return Promise.resolve({ kind: "partial", number: 123, url: "https://github.com/owner-a/repo-a/issues/123", message: "直前の作成結果を確認してください。", operation_id: "operation-a" }); }
      if (command === "acknowledge_creation") { otherPending = false; return Promise.resolve(); }
      throw new Error("unexpected command");
    });
    render(<App />);
    await screen.findByText("owner-a / repo-a");
    fireEvent.click(screen.getByRole("button", { name: "Issue を作成" }));
    fireEvent.change(screen.getByLabelText("タイトル"), { target: { value: "test title" } });
    fireEvent.click(screen.getByRole("button", { name: "作成" }));
    await screen.findByRole("button", { name: "#123 を GitHub で開く" });
    expect(screen.getByRole("button", { name: "#456 を GitHub で開く" })).toBeTruthy();
    expect(screen.getByText("owner-b / repo-b")).toBeTruthy();
    expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "再読み込み" }));
    await waitFor(() => expect(screen.getByText(/別の作成操作の記録があります/)).toBeTruthy());
    expect(screen.getByRole("button", { name: "#123 を GitHub で開く" })).toBeTruthy();
    expect((screen.getByRole("button", { name: "確認しました" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "直前の結果を確認しました" }));
    await waitFor(() => expect((screen.getByRole("button", { name: "確認しました" }) as HTMLButtonElement).disabled).toBe(false));
    expect(screen.getByRole("button", { name: "#456 を GitHub で開く" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "確認しました" }));
    await waitFor(() => expect((screen.getByRole("button", { name: "Issue を作成" }) as HTMLButtonElement).disabled).toBe(false));
    expect(mocks.invoke).toHaveBeenCalledWith("acknowledge_creation", { operationId: "operation-b" });
    expect(mocks.invoke.mock.calls.filter(([command]) => command === "create_issue")).toHaveLength(1);
  });
});
