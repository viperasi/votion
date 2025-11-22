# 项目提示词与问题整理

## 指令与操作概览
- 持续美化前端与统一主题（Home/Chat/Editor/Settings，引入 `.btn/.input/.panel` 与网格背景）
- 编辑器安全渲染与滚动同步、目录 TOC、内部链接跳转、保存索引
- 设置页新增 OpenAI/Ollama 配置（大模型与嵌入模型）、系统提示与 Top‑K/相似度参数
- 后端向量检索与问答生成（OpenAI/Ollama），流式回答与取消、广播索引事件
- 聊天页面重构（两栏布局、消息气泡、工具栏按钮、Markdown 安全渲染、自动滚动）
- 创建科技风原型与主题（`src/styles/theme.css`）并应用到各页面
- 统一设置页为卡片风格，按钮加图标与悬浮动画；响应式断点优化
- 导航改为左侧垂直工具栏；默认路由改为问答；移除首页
- 编辑器工具栏图标化（仅保留图标）、新增预览模式切换（仅预览/仅编辑/双栏）
- 文件列表简化为平铺列表；新增排序（时间/名称，升/降序）
- 侧栏支持拖拽分隔条调整宽度，移除横向滚动
- 启用 Tauri 对话框与窗口 API 允许列表（allowlist），修复目录选择与窗口控制
- 桌面应用无边框窗体，自定义标题栏拖动与窗口操作按钮
- 生成并应用 SVG 品牌图标；打包 macOS 应用；准备 Windows 打包指引
- 创建 GitHub Pages（`docs/`）并美化，支持中英文切换；推送到远端
- 版本与分支管理：创建/删除/重建 `0.0.1` 标签；合并 `develop` → `master`；推送到 GitHub/Gitee
- 许可证切换为 MIT；新增英文 README（`README.en.md`）

## 关键问题与修复
- Rust 编译错误（E0277/类型匹配）：替换不当的 `if let Ok(v)` 匹配方式；完善函数签名与命令注册
- Chat.vue 结构错误：脚本函数出现在 `</style>` 后，移动到 `<script setup>` 修复
- 设置页修改失败：多次 ApplyPatch 匹配失败，改为逐段重构并统一样式
- Tauri 对话框未启用：错误 `Dialog module is not enabled`，在 `tauri.conf.json` 开启 `dialog.open/save`
- 窗口操作未在允许列表：错误 `window > maximize not in allowlist`，开启 `window` 相关 API（`minimize/maximize/unmaximize/close/startDragging`）
- 标题栏样式无效：`titleBarStyle` 不被 Tauri 支持，改用自定义无边框窗体 + CSS 标题栏 + `startDragging`
- 标题栏拖动无效：增加 `-webkit-app-region: drag` 与 `appWindow.startDragging()` 双兜底，并避免覆盖层阻挡
- 图标生成失败：SVG 不能直接用于 macOS 打包；使用构建脚本栅格化为 PNG（`resvg/usvg/tiny-skia`），再生成 `icns/ico/png` 集并在 `bundle.icon` 引用
- 远端推送错误：`origin` 未配置，改用已存在的 `github/gitee` 远端推送
- 侧栏分隔条无效：增加 `height:100%`、拖动时设置 `cursor/userSelect`，并按拖动更新 `gridTemplateColumns`
- GitHub Pages 资源路径错误：统一使用 `docs/` 内资源路径并添加导航与下载区块
- 语言切换：修复 Docs 链接随语言同步（中文→`README.md`，英文→`README.en.md`）与初始化逻辑

## 构建与发布记录
- 版本：`0.0.1`
- 标识符：`com.xu81.votion`
- macOS 构建：`src-tauri/target/release/bundle/macos/Votion.app`，DMG 发布
- 分支与标签：`develop` → `master` 合并；`0.0.1` 标签创建/推送；GitHub/Gitee 双远端同步

## Tauri 配置要点
- `allowlist.dialog`: `open/save` 用于系统目录/文件选择
- `allowlist.window`: `startDragging/minimize/maximize/unmaximize/close`
- `bundle.active: true`；`bundle.icon`: 引用 `icons/icon.icns/icon.ico/icon.png`
- 无边框窗体：`decorations: false` + 自定义标题栏拖动区

## 后续建议
- 为 Pages 增加 Release 自动列表与版本变更日志
- Windows 打包在 Windows 环境执行并上传 Zip/Installer
- 进一步统一按钮与输入的响应式尺寸，完善空态与加载态