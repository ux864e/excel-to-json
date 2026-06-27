# 2026-06-27

## Added

- **Cargo 项目初始化**：`excel-to-json` 单 crate，edition 2024，MSRV 1.85
- **核心依赖**：clap 4.5 (derive)、calamine 0.26、anyhow + thiserror、serde + serde_json + toml、tracing + tracing-subscriber
- **源码模块骨架**（9 文件）：main.rs（薄入口）、lib.rs（run() + mod 声明）、cli.rs、config.rs、error.rs（EError enum）、walker.rs、converter.rs、mapping.rs、output.rs（IPC 消息）
- **测试结构**：7 个单元测试（mapping + output 模块）+ 4 个集成测试（assert_cmd）
- **Agent 配置**：CLAUDE.md、AGENTS.md、8 个 .agents/instructions/ 文件（user-preferences、code-style-rust、code-style-rust-extra、tech-stack、project-directory、test-instructions、writing-design-docs、git-status-reminder）
- **.claude/ 配置**：settings.json（权限）、settings.local.json（SessionStart hook → git-health-check）、hooks/git-health-check.sh、memory/MEMORY.md + project-init-complete.md
- **CI/CD**：`.github/workflows/ci.yml`（GitHub Actions：fmt → clippy → test → build）
- **设计文档**：`docs/design/` 5 个文档（项目启动、用户故事、需求、系统设计、实施计划），YAML frontmatter + Mermaid 图
- **Project Init Skill**：安装到 `~/.agents/skills/project-init/`，含 SKILL.md、question-checklist、检测脚本、CLAUDE.md/AGENTS.md 模板

## 修改文件

- `Cargo.toml`：name → excel-to-json，添加完整依赖和生产 profile
- `src/main.rs`：从 hello-world 改为薄入口 + exit(1) 错误处理
- `.gitignore`：已存在，保留原有 Rust 项目配置
- `AGENTS.md`：从 agent-ninja 桩扩展为完整项目上下文
- `.claude/settings.local.json`：从临时权限改为 SessionStart hook 配置

## 设计决策

- **Enum E 前缀**：保留用户 TypeScript 项目的 EError 命名习惯（Rust 社区不做此要求，但个人项目一致性优先）
- **calamine 0.26**：使用当前 Rust 1.96 兼容的最新稳定版（非 0.35，因 MSRV 差距太大）
- **rustfmt 仅 stable 特性**：移除 imports_granularity/group_imports/wrap_comments 等 nightly-only 选项
- **单 crate**：项目规模小，workspace 过度设计。保留 `src/lib.rs` 以便集成测试和未来扩展
- **docs/design/ 在仓库内**：个人项目不需要分离仓库，直接用简化的 cuddle 三级结构

## 剩余问题

- Rust 1.96 安装路径 `~/.cargo/bin` 不在默认 PATH，需 `export PATH="$HOME/.cargo/bin:$PATH"`
- 后续 Phase 1-4 待实施（核心转换逻辑、配置与 CLI、IPC 与日志、文档与发布）
