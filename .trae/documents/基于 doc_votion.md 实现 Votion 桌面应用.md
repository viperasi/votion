# 目标与范围
- 按照 `/doc/votion.md` 的特性实现：自动监控笔记并同步到知识库、AI 智能问答、Markdown 笔记编辑/预览、跨平台桌面应用。
- 技术栈：Tauri（桌面壳 + Rust 后端）、TypeScript + Vue3（前端 UI）、Vite（构建）。

## 现有仓库状态
- 当前仓库未包含 Tauri/Vue3 源码与配置，仅有文档与元信息文件。
- 计划从零搭建项目骨架与关键模块。

## 总体架构
- 前端：Vue3 + TypeScript + Vite，提供笔记浏览、编辑、搜索与 AI 问答界面。
- 后端：Tauri（Rust）实现文件系统监听、索引构建、向量检索与模型/服务调用；暴露命令给前端。
- 存储：本地 SQLite（通过 Tauri 插件或 Rust 集成）存储笔记元数据、分块、嵌入向量与配置。
- AI：检索增强生成（RAG）。可插拔的嵌入/推理提供者（先集成云端嵌入 API，后续支持本地模型）。

## 项目结构
- 根目录：`package.json`、`vite.config.ts`、`tsconfig.json`、`.editorconfig` 等
- 前端：`/src`（`main.ts`、`App.vue`、`router`、`pages`、`components`、`stores`、`services`）
- 桌面后端：`/src-tauri`（`Cargo.toml`、`tauri.conf.json`、`src/main.rs` 与模块化命令实现）
- 笔记目录：用户在设置中选择；默认创建 `~/VotionNotes`（可变更）

## 功能模块与实现
- 笔记监控与知识库同步
  - 设置中选择笔记根目录
  - Rust 用 `notify`（或等价）监听新增/修改/删除
  - 解析 Markdown、提取 front-matter 与正文，进行分块（按标题/长度/段落）
  - 计算嵌入向量，写入 SQLite（笔记、分块、嵌入、索引版本）
  - 增量更新：仅重算受影响的分块
- Markdown 编辑/预览
  - Vue 组件：编辑器（支持基础快捷键）、预览（`markdown-it` + 代码高亮）
  - 打开/保存到本地文件，状态与冲突处理（监听时区分外部修改）
- AI 智能问答（RAG）
  - 前端输入问题 → 后端：查询相关分块（相似度 Top-K）→ 组装上下文 → 调用模型生成回答
  - 支持流式输出与对话历史（本地保存）
  - 错误与网络异常处理（降级模式：仅检索，不调用生成）
- 设置与安全
  - 提供者配置：嵌入/推理服务选择、API Key、超时与重试
  - 本地安全存储（不写日志、不明文持久化 Key）
  - 选择笔记目录、索引重建与清理
- 跨平台
  - macOS、Windows、Linux 打包构建；图标与签名配置

## 数据模型（SQLite）
- `notes(id, path, title, created_at, updated_at, hash)`
- `chunks(id, note_id, seq, content, content_hash)`
- `embeddings(id, chunk_id, vector_blob, dim)`
- `settings(key, value, updated_at)`
- 通过 Rust 进行相似度计算（余弦/点积，先用朴素 Top-K，后续可扩展近似检索）

## 后端命令接口（Tauri）
- `watch_notes(dir)`：启动/更新监听
- `index_note(path)`：单文件索引
- `search_embeddings(query, top_k)`：向量检索返回分块与相关度
- `generate_answer(query, context_chunks)`：生成回答（支持流式）
- `get_notes()` / `get_note(path)` / `save_note(path, content)`：笔记 CRUD
- `get_settings()` / `update_settings(kv)`：设置读写

## 前端页面与交互
- 首页/搜索：查询、相关分块展示与导出
- 编辑器：文件树、编辑区、预览区、保存与外部变更提示
- 对话：问答输入、参考来源引用、历史列表
- 设置：目录选择、模型与密钥、索引管理
- 状态管理：`pinia`（或组合式 API）统一状态与服务调用

## 依赖建议
- 前端：`vue`, `vue-router`, `pinia`, `markdown-it`, `highlight.js`
- 后端：`notify`, `serde`, `tokio`, `reqwest`（如需云端嵌入/推理），`rusqlite` 或 Tauri SQL 插件
- 开发：`vite`, `typescript`, `eslint` + 规则集

## 安全与隐私
- API Key 仅用于内存/安全存储，避免日志与前端暴露
- 用户数据仅本地存储，明确导入/导出行为
- 错误信息脱敏

## 验证与测试
- 搭建基础 E2E：创建示例笔记、监听与索引、检索与问答的演练脚本
- 单元测试：分块策略、相似度计算、索引增量更新
- 手动测试：编辑器保存与外部修改冲突

## 迭代计划
1. 初始化 Tauri + Vue3 项目骨架，跑通开发与打包
2. 文件监听与索引管道（解析、分块、存储）
3. 检索 + 问答最小可用版本（云端嵌入优先）
4. Markdown 编辑/预览与文件树
5. 设置界面与安全存储
6. 跨平台打包与发布配置

请确认该实现规划；确认后我将开始初始化项目并逐步交付各模块。