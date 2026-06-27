---
description: '设计决策备忘 + 剩余问题。变更列表和修改文件见 changelog/。'
---

# DEV_NOTES

> 最后更新：2026-06-27

---

## 2026-06-27 — 项目初始化：技术选型与架构设计

### 设计决策

- **Rust edition 2024 + MSRV 1.85**：使用当前最新稳定版 1.96，向下兼容到 1.85（edition 2024 最低要求）
- **任何how + thiserror 双层错误**：应用层 `anyhow::Result` + `?` 传播；库层 `EError` enum（thiserror derive）。不混用，不裸写 `unwrap()/expect()`
- **calamine 0.26 vs 0.35**：当前环境 Rust 1.96，calamine 0.35 可能需要更高 MSRV。选择 0.26 作为兼容基线，后续可升级
- **IPC NDJSON 格式**：stdout 逐行 JSON，`{"type":"progress|done|error",...}`。stderr 走 tracing。父进程按行 parse，无需特殊分隔符
- **Config 合并策略**：TOML 文件为基础，CLI 参数覆盖。`resolve_config_path()` → input_dir 下的 `excel-to-json.toml` 或 `--config` 显式路径
- **Enum E 前缀**：保留 TypeScript 项目的 `EError` 命名习惯。Rust 社区不推荐前缀（clippy 不会 warn），但个人项目一致性优先
- **单 crate**：项目规模小（预计 <10 源文件），workspace 加重维护负担。`src/lib.rs` 保留以支持集成测试和未来 lib 化
- **docs/design/ 在仓库内**：个人 CLI 工具不需要分离设计文档仓库。采用简化版 cuddle 三级结构：`NN.system/` 扁平化（0-4 编号，无子系统子目录）
- **rustfmt stable only**：`.rustfmt.toml` 仅用稳定特性（edition、max_width、newline_style、reorder_imports）。移除 nightly-only 选项（imports_granularity、group_imports、wrap_comments、format_code_in_doc_comments）
- **Project Init Skill 形态**：引导式讨论 + 阶段产物生成。9 阶段工作流（用户故事 → Review → 需求 → 设计 → Deep Review → 计划 → Final Review → 执行 → 归档）。基于本次 excel-to-json 初始化实践提炼

### 剩余问题

- Rust 1.96 toolchain 路径 `~/.cargo/bin` 未在默认 PATH，需要每次 `export` 或配置 shell profile
- CI 当前仅 ubuntu-latest，如需 macOS 交叉编译需补充 matrix
- calamine 0.26 的 `Data` enum 有 `DateTime`、`DateTimeIso`、`DurationIso` 三个时间相关变体，需确保映射逻辑正确处理
