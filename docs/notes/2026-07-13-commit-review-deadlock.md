# commit-review.ts 死循环：首次 commit 无法通过

## 摘要

`~/.omp/agent/extensions/commit-review.ts` 的设计存在死循环陷阱：首次 `git commit` 永远被拦截，永远无法成功执行以填充审查批准记录，导致任何"按 token 重试"的指引都失效。

## 复现步骤

1. 在任意 omp 管理的仓库中，准备 staged 变更
2. 跑 reviewer 子 agent（返回 `correct` / `correct-with-debt`）
3. 跑 `git commit -m "..." [omp-review:ok]`
4. **结果**：commit 被拦截，错误信息要求"Re-run with token"——但带 token 重试也被同样拦截

## 期望行为

按错误信息指引"Re-run `git commit` with `[omp-review:ok]` appended"应能解锁 commit，因为：
- reviewer 已返回 verdict
- token 已在 commit message 中

## 实际行为

无论是否带 token，首次 commit 始终被拦截。

## 根因（基于源码分析）

`commit-review.ts`（`/Users/norman/.omp/agent/extensions/commit-review.ts`）的逻辑：

```ts
// tool_call 钩子：拦截首次 commit
pi.on("tool_call", async (event, ctx) => {
  const hash = await hashStagedDiff(ctx.cwd, pi);
  if (hash && APPROVED.has(hash)) return;        // L82
  return { block: true, reason };                // L104
});

// tool_result 钩子：仅在 commit 实际成功时填充 APPROVED
pi.on("tool_result", async (event, ctx) => {
  if ((event as { isError?: boolean }).isError) return;  // L117
  // ...
  if (hash) APPROVED.add(hash);                  // L123
});
```

**死循环链**：
1. 首次 commit → `tool_call` 钩子检查 `APPROVED.has(hash)` → `APPROVED` 为空 → 返回 `block: true`
2. bash 命令被阻止执行，**不会产生 tool_result 事件**（或产生 `isError: true` 的事件被 L117 跳过）
3. APPROVED 永远不会被填充
4. 重试带 token 的 commit → 回到步骤 1

**关键发现**：
- `[omp-review:ok]` token 在 L120 仅为 `extractCommitMessage` 检测，但**不影响 block 决策**（L82 不读 token）
- `.sdd/review/staged.reviewer.json`（reviewer agent 产物）**不被 commit-review.ts 读取**——两条独立路径
- L82 的 `if (hash && APPROVED.has(hash)) return` 是**唯一**放行条件，但 APPROVED 永远空

## 建议修复（任选其一）

### 方案 A：读取 reviewer 产物
让 `tool_call` 钩子在 L82 增加 fallback：检查 `.sdd/review/staged.reviewer.json` 的 `staged_hash` 与当前 hash 一致，且 `overall_correctness` 非 `incorrect`。

### 方案 B：token 触发 add
让 `tool_call` 钩子在检测到 `[omp-review:ok]` token 时，**先** `APPROVED.add(hash)` 再 `return`——把 token 当作承诺，让实际 commit 触发 tool_result 走通。

### 方案 C：让首次 commit 实际执行
修改 L104 为返回 `void`（不 block）但附加提示到 stderr——这改变了设计语义，可能影响其他扩展。

## 影响范围

- 任何大变更（多文件、跨目录）容易触发此循环，因为 review 期间可能需要修复 reviewer 提出的 debt → staged snapshot 变化
- 已观察到 4 次 reviewer 都返回 `correct` / `correct-with-debt` 但 commit 仍失败的场景
- 用户唯一可行的临时绕路是手动 `mv` 扩展文件禁用 commit-review

## 建议优先级

**P1**（block 关键工作流）：任何 commit 流程都被无解地阻断，用户只能绕过扩展本身。

## 关联文件

- `/Users/norman/.omp/agent/extensions/commit-review.ts` — 拦截逻辑实现
- `/Users/norman/.omp/plugins/cache/marketplaces/sdd-pack/plugins/sdd-pack/agents/reviewer.md` — reviewer agent 协议（步骤 8 写 reviewer 产物）

## 环境信息

- omp 版本：sdd-pack 1.5.0（plugin cache 目录）
- 触发场景：Rust 学习笔记本仓库（19 文件综合变更，含 1 个文件删除）
- 复现时间：2026-07-13
