# Votion

A cross-platform local knowledge and conversation assistant built with Tauri + Vue 3. It supports Markdown note editing/preview, vector search, and AI Q&A; runs locally and can be self-hosted.

## Features
- Left vertical toolbar, unified dark sci‑fi theme and custom scrollbar
- Chat: streaming generation, model badge, thinking placeholder and spinner; capsule input with @/# shortcuts, Enter to send, Shift+Enter for newline
- Editor: safe rendering (DOMPurify), code highlighting (highlight.js), internal link navigation; three‑section capsule toolbar; view modes (Preview‑only / Editor‑only / Both); resizable sidebar
- Settings: OpenAI/Ollama provider configuration (chat + embeddings), system directory picker for notes; test section removed
- Backend: SQLite index and embeddings table; index update event broadcast; vector‑first retrieval with keyword fallback
- The entire application is developed using the `trae SOLO Coder` mode

## Install
Prerequisites:
- Node.js ≥ 16
- Rust toolchain + Tauri environment (see https://tauri.app/)

Install dependencies:
```
npm install
```

## Development
Web dev:
```
npm run dev
```

Tauri app dev:
```
npm run tauri:dev
```

## Build
Bundle frontend:
```
npm run build
```

## Usage
- Switch pages from the left toolbar: Chat / Editor / Settings
- Chat: type your question and press Enter; collapsing “References” expands chat to full width; shows thinking state and streams the answer
- Editor: header buttons show tooltips; file list supports sorting (time desc/asc, name desc/asc); toggle view modes; drag the splitter to resize the sidebar
- Settings: pick the notes directory via system dialog; configure provider and models/embedding models
- 

## Configuration
Set in Settings:
- `provider`: `openai` or `ollama`
- `openai_api_key`, `openai_base_url`, `openai_model`, `openai_embed_model`
- `ollama_base_url`, `ollama_model`, `ollama_embed_model`
- `notes_dir` (notes root), knowledge base parameters (chunk size/overlap, Top‑K, minimum similarity)

## Roadmap
- More responsive refinements and quick actions
- Reference cards with file‑type icons and similarity bars
- More export and batch operations

## Contribute
Issues and PRs are welcome; please base changes on the `develop` branch.

## License
MIT
