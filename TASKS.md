---
description: '完成Plan并准备实施或已在实施中的具体工作内容，由Agent维护和更新，供团队成员参考和跟踪'
---

# Overview

> Last Update: 2026-06-27

# 任务列表

## [P1][短期] Phase 1 — 核心转换逻辑实现

> 完善 converter.rs 和 mapping.rs，添加测试 fixtures。

- [ ] **converter.rs 完善** — 处理空 sheet、空单元格、大文件流式读取
- [ ] **mapping.rs 嵌套路径完善** — 端到端测试 dot-path 展开逻辑
- [ ] **测试 fixtures** — 添加 .xlsx/.csv/.xls 测试文件到 `tests/fixtures/`
- [ ] **边界情况测试** — 空文件、单行列、多 sheet、日期格式、公式单元格

## [P1][短期] Phase 2 — 配置与 CLI 完善

> 验证 CLI + TOML 合并逻辑，添加安全选项。

- [ ] **CLI 参数覆盖验证** — 确保 input/output/recursive/pretty 四项正确合并
- [ ] **--dry-run 模式** — 扫描并报告但不写入文件
- [ ] **--overwrite / --skip-existing** — 输出文件冲突处理
- [ ] **config.toml schema 文档** — 在 README 或 docs/ 中提供完整 schema

## [P2][普通] Phase 3 — IPC 与日志完善

> tracing-subscriber 初始化，日志级别控制。

- [ ] **tracing-subscriber 初始化** — fmt layer + env-filter，默认 info 级别
- [ ] **-v flag 集成** — verbose count → log level mapping
- [ ] **RUST_LOG 支持** — env var 优先级高于 -v flag
- [ ] **IPC 消息格式文档** — 为下游消费者提供 JSON schema

## [P3][长期] Phase 4 — 文档与发布

> README 完善，crates.io 发布准备。

- [ ] **用户指南** — 使用示例、常见场景、配置示例
- [ ] **GitHub Release workflow** — tag → build matrix → release artifacts
- [ ] **Shell completions** — clap_complete 生成 bash/zsh/fish 补全
- [ ] **crates.io 发布** — package metadata、keywords、categories
