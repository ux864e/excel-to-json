# 2026-06-29

## Added

- **deploy.sh 脚本**：`scripts/deploy.sh` — 编译 release binary 并部署到可配置目标目录（默认 `../cuddle-app-backend/tools`），更名为 `config-importer`（Windows 附 `.exe`）
- **Excel Tab 元数据约定**：`parse_sheet_with_meta` 解析器。Row 0 = configName，Row 1 = description，Row 2 = headers，Row 3 = skip，Row 4+ = data
- **`id` 列强制**：Col A 统一为 `id`，唯一 u64。`//` 前缀 = 注释行跳过。重复 id 跳过 + warn
- **configName 校验**：正则 `^[a-z][a-z0-9_-]*[a-z0-9]$`，强制 lowercase，非法起止符/字符报错
- **行统计**：`inputRows`/`validRows`/`skippedRows`/`failedRows` 四维统计，每 config 独立
- **单行 JSON 摘要**：stdout 输出 `{"status","files[]","errors[]","warnings[]"}`，空 errors/warnings 省略
- **FileSummary 重构**：`configName`/`path`/`description`/`validRows`/`inputRows`/`skippedRows`/`failedRows`。移除 `file` 和 `rows`
- **ConfigOutput 类型**：每 sheet 独立产出，携带自身 configName、description、行统计
- **ErrorEntry / WarningEntry 类型**：converter.rs 中的摘要数据结构
- **`validate_config_name`**：configName 校验 + 强制 lowercase
- **`parse_id`**：u64 解析（Int/Float/String），`cell_to_string` 辅助
- **测试 fixtures**：`sample.xlsx`（新格式 1 sheet）、`multi-sheet.xlsx`（2 sheets，含注释行 + 重复 id）
- **新集成测试**：`test_single_xlsx_conversion`、`test_multi_sheet_configs`、`test_comment_rows_skipped`、`test_duplicate_id_skipped`、`test_mixed_success_failure`、`test_all_files_fail`
- **新单元测试**：validate_config_name (5)、parse_id (3)、ConfigOutput 序列化、emit_results config 模式

## Changed

- **converter.rs**：完全重写。移除 `extract_headers_and_rows`、`convert_csv`。新增 `parse_sheet_with_meta`、`ConfigOutput`、`ConversionResult.configs`。逐 sheet 容错
- **output.rs**：`FileSummary` 字段重构。`emit_results` 统一写入 `.runtime-cached/resources/configured/`。移除 legacy 目录镜像模式
- **config.rs**：移除 `Config.config_name`。`StdinInput` 仅保留 `outputDir`（configName 从 Excel 提取）
- **lib.rs**：stdin 仅覆盖 `output_dir`。移除 `create_dir_all`（output.rs 内部处理）
- **cli.rs**：`recursive`/`pretty` bool 参数添加 `action = ArgAction::Set`
- **walker.rs**：`SUPPORTED_EXTENSIONS` 移除 `"csv"`；`walk_dir` 添加 `base_dir` 参数；`ExcelFile` 添加 `relative_path`
- **README.md**：完全重写 — Excel 约定、stdin 用法、输出格式、configName 规则、行统计说明
- **docs/DEV_NOTES.md**：新增元数据约定、行统计、configName 校验等设计决策
- **docs/TASKS.md**：标记已完成项，新增 Phase 5（健壮性）

## Removed

- **IpcMessage enum**（Progress/Done/Error/Status 变体）
- **CSV 支持**：`convert_csv` 函数、`"csv"` extension
- **extract_headers_and_rows**：被 `parse_sheet_with_meta` 替代
- **旧 Excel 格式兼容**：Row 0 = header 的行为不再支持
- **Config.config_name**：configName 现在是 sheet 级属性
- **StdinInput.configName**：不再从 stdin 传入
- **硬编码 .runtime-cached/resources/configured/**：示例路径误硬编码到 emit_results，已移除，输出直接写入 outputDir.
- **旧 JSON 文件结构**：`{"<configName>": [...]}` 改为 `{"configName":"...","description":"...","items":[...]}`
- **configName 校验规则迭代**：最终确定为 `^[a-z][a-zA-Z0-9_]*[a-zA-Z0-9]$`。移除连字符 `-` 支持，移除强制 lowercase 转换（改为拒绝大写首字符），body 允许大写。

## 修改文件

- `src/converter.rs` — 完全重写（ConfigOutput、parse_sheet_with_meta、validate_config_name、parse_id）
- `src/output.rs` — FileSummary 重构，emit_results 统一配置路径
- `src/config.rs` — 移除 config_name，简化 StdinInput
- `src/lib.rs` — stdin 处理简化
- `src/main.rs` — 全局错误摘要
- `src/cli.rs` — ArgAction::Set for bool args
- `src/walker.rs` — relative_path + base_dir
- `tests/cli.rs` — 10 个集成测试
- `scripts/deploy.sh` — 新增部署脚本（自动检测平台编译，自动调用 setup-rust.sh）
- `scripts/setup-rust.sh` — 新增 Rust 环境初始化脚本（幂等）
- `tests/fixtures/sample.xlsx` — 新格式 fixture
- `tests/fixtures/multi-sheet.xlsx` — 新增多 sheet fixture
- `scripts/deploy.sh` — 新增部署脚本
- `README.md` — 完全重写
- `docs/DEV_NOTES.md` — 设计决策更新
- `docs/TASKS.md` — 任务状态更新
