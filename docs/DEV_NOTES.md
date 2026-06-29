---
description: '设计决策备忘 + 剩余问题。变更列表和修改文件见 changelog/。'
---

# DEV_NOTES

> 最后更新：2026-06-29

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

---

## 2026-06-29 — 输出系统重构：单行摘要 + 目录镜像 + 容错转换

### 设计决策

- **单行 JSON 摘要替代多行 IPC**：将逐文件 progress/done 消息替换为一次性的 `{"status","files[]","errors[]","warnings[]"}` 摘要行。空 errors/warnings 省略。父进程只需解析一行即可获得完整结果。
- **输出文件直接写入 outputDir**：最初硬编码了 `.runtime-cached/resources/configured/` 子路径，后移除。现输出直接写入 `<outputDir>/<configName>.json`。
- **JSON 文件输出结构**：`{"configName":"...","description":"...","items":[...rows...]}` — configName 和 description 作为顶层字段，数据行在 `items` 数组下。
- **逐文件错误收集**：`convert_all` 从 `Result<Vec<ConversionResult>>` 改为返回 `ConversionSummary`（infallible）。单文件失败不再中止整批转换，错误收集到 `errors[]` 数组。
- **Status 逻辑**：有任一 config 成功 → `"success"`；无 config 且无错误（空目录）→ `"success"`；全部 config 失败 → `"error"`（exit 1）。
- **calamine 0.26 不支持 CSV**：calamine 0.26.1 移除了 CSV reader（无 `csv.rs` 模块）。移除 `convert_csv` 函数和 SUPPORTED_EXTENSIONS 中的 `"csv"`。
- **deploy.sh 脚本**：`scripts/deploy.sh` — 编译 release binary 并部署到可配置目标目录，更名为 `config-importer`。

### 剩余问题

- CSV 支持已移除（calamine 0.26 不兼容）。若未来需要 CSV，可引入 `csv` crate 作为独立 reader，或降级 calamine。
- WarningEntry 类型已定义但当前无 producer（`warnings` 始终为空数组）。预留用于未来的非致命问题（如公式单元格、空 sheet 等）。
- 测试 fixtures 目前仅 `sample.xlsx`（最小化有效文件）。需补充多 sheet、日期格式、空 sheet 等边界情况 fixture。

---

## 2026-06-29 (2) — Excel Tab 元数据约定 + 行统计 + configName 校验

### 设计决策

- **Sheet 元数据提取**：Row 0-3 为元数据行。Row 0 Col B = configName，Row 1 Col B = description，Row 2 = 字段定义（headers），Row 3 = 字段注释（跳过）。Row 4+ 为数据行。每个 sheet 生成独立 config 文件。
- **`id` 列强制**：所有 sheet 的 Col A 统一为 `id` 列。值必须为唯一无符号整数（u64）。以 `//` 开头的行为注释行，跳过不转换。重复 id 跳过并 warn。
- **sheet 级容错**：单个 sheet 解析失败不中止整个文件 — 跳过该 sheet 继续处理下一个。
- **configName 校验**：正则 `^[a-z][a-zA-Z0-9_]*[a-zA-Z0-9]$`。首字符必须小写字母，body 允许大写/小写/数字/下划线，尾部必须字母或数字。不允许连字符 `-`。不强制转换大小写，不符合规则直接报错。
- **行统计**：每 config 输出 `inputRows`（总数据行）、`validRows`（成功转换）、`skippedRows`（注释行 + 重复 id）、`failedRows`（无效 id）。
- **FileSummary 重构**：移除 `file`（相对路径）和 `rows` 字段。新增 `configName`、`path`（`<configName>.json`）、`description`、`validRows`、`inputRows`、`skippedRows`、`failedRows`。`configName` 序列化为 JSON key `configName`（camelCase），供 `initResourceIndex()` 索引。
- **StdinInput 简化**：移除 `configName` 字段（现从 Excel sheet 提取）。stdin 仅传 `outputDir`。
- **移除 Config.config_name**：configName 不再存储在全局 Config 中，每个 sheet 独立携带。
- **移除旧 extract_headers_and_rows**：新 parser `parse_sheet_with_meta` 完全替代。旧格式 Excel（无元数据行）不再兼容。
- **测试 fixtures**：`sample.xlsx` 更新为新格式（1 sheet + 元数据行）。新增 `multi-sheet.xlsx`（2 sheets，含注释行和重复 id 用于边界测试）。
- **移除硬编码子路径**：`.runtime-cached/resources/configured/` 为示例路径，误硬编码到 `emit_results`。已移除，输出直接写入 `outputDir`。
- **JSON 文件输出结构**：输出文件从 `{"<configName>": [...]}` 改为 `{"configName":"...","description":"...","items":[...]}`。configName 和 description 提升为顶层字段，数据行统一在 `items` 键下。

### 剩余问题

- 多 sheet 场景下，若某个 sheet 的 configName 重复，后者会覆盖前者的输出文件。未做去重检测。
- 旧格式 Excel（row 0 = header）不再支持。如需兼容可考虑基于 Row 0 Col B 是否为空做 fallback 检测。
