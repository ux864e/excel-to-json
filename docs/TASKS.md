---
description: '完成Plan并准备实施或已在实施中的具体工作内容，由Agent维护和更新，供团队成员参考和跟踪'
---

# Overview

> Last Update: 2026-06-29

# 任务列表

## [P1][短期] Phase 1 — 核心转换逻辑实现

> 完善 converter.rs 和 mapping.rs，添加测试 fixtures。

- [ ] **mapping.rs 嵌套路径完善** — 端到端测试 dot-path 展开逻辑
- [ ] **测试 fixtures** — 添加日期格式、空 sheet、公式单元格等边界情况 fixture
- [ ] **空 sheet 处理** — 空 sheet（仅元数据行，无数据行）的合理行为
- [ ] **大文件流式读取** — 当前全量加载到内存，大文件需优化
- [x] **输出格式重构** — 单行 JSON 摘要（status/files[]/errors[]/warnings[]）
- [x] **Excel Tab 元数据约定** — Row 0-3 元数据 + Row 4+ 数据，configName/description 提取
- [x] **`id` 列强制** — Col A = id（唯一 u64），`//` 注释行跳过，重复 id 跳过
- [x] **configName 校验** — 正则 `^[a-z][a-z0-9_-]*[a-z0-9]$`，强制 lowercase
- [x] **行统计** — inputRows/validRows/skippedRows/failedRows 输出
- [x] **逐 sheet 容错** — 单 sheet 失败不中止整文件
- [x] **deploy.sh 脚本** — 编译后部署到目标目录并更名

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
- [x] **输出格式文档** — README 完整描述 stdout 摘要、Excel 约定、stdin 用法

## [P3][长期] Phase 4 — 文档与发布

> README 完善，crates.io 发布准备。

- [ ] **用户指南** — 使用示例、常见场景、配置示例
- [ ] **GitHub Release workflow** — tag → build matrix → release artifacts
- [ ] **Shell completions** — clap_complete 生成 bash/zsh/fish 补全
- [ ] **crates.io 发布** — package metadata、keywords、categories

## [P2][普通] Phase 5 — 健壮性

> 边界情况和冲突处理。

- [ ] **duplicate configName 检测** — 多 sheet 同名 configName 时后者覆盖前者，需检测并报错
- [ ] **旧格式兼容** — 可选的 legacy mode（Row 0 = header）fallback 检测
- [ ] **CSV 支持恢复** — 评估降级 calamine 或引入 csv crate
