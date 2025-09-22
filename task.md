# Project Planning

## 1. Coding Features Removal Status
- Language-server settings remain in `assets/settings/default.json:899` and related files, and coding crates like `crates/language`, `crates/project`, and `crates/lsp` are still part of the workspace.
- Editor modules (`crates/editor/src/editor.rs`) and workspace components still subscribe to LSP/git events, indicating code tooling is intact.
- Documentation and extensions continue to reference programming workflows (e.g., `docs/src/diagnostics.md`, `extensions/README.md`).
- Conclusion: the code-focused surface has not yet been stripped; the project still behaves like a programming IDE.

## 2. Roadmap for MCP-Enabled Novel Authoring Tool
1. **Isolate Text Core**: introduce feature gating or split crates so the core build excludes `project`, `worktree`, LSP, git, debugger, and extension registries by default.
2. **Author-Focused UI**: replace project/git panels with manuscript navigation, scene/character management, and markdown preview tailored to writers.
3. **Manuscript Data Model**: define persistence for manuscripts, chapters, characters, research notes, and expose safe read/write APIs for agents.
4. **Agent & MCP Integration**: retain agent infrastructure but curate novel-writing tool profiles; register required MCP servers (research, outlining, editing) and ensure configuration UX.
5. **Workflow Automations**: script agent-driven flows (outline → draft → revision), leverage thread store checkpoints, and provide evaluation loops for output quality.
6. **Testing & Telemetry**: add regression tests for manuscript operations, agent automations, and collect telemetry to monitor tool usage/errors.
7. **Documentation & Onboarding**: craft guides covering new workflows, MCP setup, and clearly document removed coding capabilities.

## Immediate Next Steps
- Map dependencies to decide which crates/features must be gated or removed for the text-first build.
- Prototype a minimal manuscript navigator UI to replace the project panel.
- Inventory existing agent tools to determine which can transition to writing-focused profiles.

## Dependency Audit (2024-09-20)
+ Coding-related crates still in workspace: command_palette, command_palette_hooks, copilot, dap, dap_adapters, debug_adapter_extension, debugger_tools, debugger_ui, extension*, git*, go_to_line, jj*, language*, lsp, node_runtime, outline*, prettier, project*, recent_projects, release_channel, remote*, repl, search, supermaven*, terminal*, theme_extension, vim*, web_search*, worktree.
+ Core text stack to retain: assets, editor, markdown, markdown_preview, multi_buffer, text, ui, gpui, gpui_tokio, settings, util, workspace, agent*, agent_servers, assistant_tool*, context_server, prompt_store, task, rich_text, theme.
+ Supporting crates requiring review (infra/services): activity_indicator, ai*, auto_update*, cloud*, collab*, diagnostics, feedback, telemetry, zed*, etc.
+ Next action: design feature gating/workspace split to drop coding crate set for text builds.

- Created new `crates/manuscript_panel` crate providing a placeholder panel implementing the `Panel` trait.
  Currently renders a simple message and registers a toggle action; integration with the app will follow when coding
  features are gated off.

## Agent Tool Inventory (initial)
- Core filesystem tools: TerminalTool, CreateDirectoryTool, CopyPathTool, DeletePathTool, MovePathTool, EditFileTool.
- Navigation/read: ListDirectoryTool, FindPathTool, OpenTool, ReadFileTool.
- Utilities: NowTool, ThinkingTool, FetchTool, WebSearchTool.
- Coding-specific to disable for writers: DiagnosticsTool, ProjectNotificationsTool, GrepTool, TerminalTool (maybe optional).
- Follow-up: design manuscript-aware replacements (outline builder, character DB) and prune or gate coding tools via agent profiles.
