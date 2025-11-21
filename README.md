# Votion

跨平台本地知识与对话助手，基于 Tauri + Vue3 构建。支持 Markdown 笔记编辑/预览、向量检索与 AI 问答，私有化部署与本地运行。

## Features / 特性
- 左侧垂直工具栏，全局深色科技主题与滚动条样式
- Chat 问答：流式生成、模型徽章、思考占位与等待图标；输入胶囊一体化（@/# 快捷、回车发送、Shift+回车换行）
- Editor 编辑器：安全渲染（DOMPurify）、代码高亮（highlight.js）、内部链接跳转；顶部三段式胶囊工具栏、仅预览/仅编辑/双栏切换；侧栏文件列表可拖拽调宽
- Settings 设置：OpenAI/Ollama 模型与嵌入配置、笔记目录选择（Tauri Dialog）、知识库参数；移除测试分组
- 后端：SQLite 索引与 embeddings 表；事件广播自动刷新；向量优先检索，关键词回退

## Install / 安装
Prerequisites / 前置依赖：
- Node.js ≥ 16
- Rust toolchain + Tauri 环境（详见 https://tauri.app/ ）

Install / 安装依赖：
```
npm install
```

## Dev / 开发
Web 开发：
```
npm run dev
```

Tauri 应用开发：
```
npm run tauri:dev
```

## Build / 构建
打包前端：
```
npm run build
```

## Usage / 使用
- 左侧工具栏切换：Chat / Editor / Settings
- Chat：输入问题后回车发送；点击“收起引用”后聊天区域全宽铺满；支持显示思考状态与流式输出
- Editor：顶部按钮悬浮提示；文件列表按排序规则展示（时间倒序/正序、名称倒序/正序）；支持仅预览/仅编辑/双栏切换；分隔条拖拽调整侧栏宽度
- Settings：选择笔记目录（系统目录选择窗口），配置 AI 提供商与模型/嵌入模型

## Config / 配置
在 Settings 中设置：
- `provider`: `openai` 或 `ollama`
- `openai_api_key` / `openai_base_url` / `openai_model` / `openai_embed_model`
- `ollama_base_url` / `ollama_model` / `ollama_embed_model`
- `notes_dir`（笔记目录）、知识库参数（分块大小/重叠、Top-K、最小相似度）

## Roadmap / 规划
- 响应式细节优化与更多快捷操作
- 引用卡片类型图标与相似度条
- 更多导出与批量操作

## Contribute / 贡献
欢迎 Issue 与 PR；建议在 `develop` 分支上提交改动。

## License / 许可
MIT
