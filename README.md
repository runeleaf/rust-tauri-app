# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Installation

```bash
$ cargo install create-tauri-app
$ cargo create-tauri-app
Project name: rust-tauri-app
Choose which language to use for your frontend: TypeScript
Choose your package manager: pnpm
Choose your UI template: React
Choose your UI flavor: TypeScript
```

## Run

```bash
$ cd rust-tauri-app
$ pnpm install
$ pnpm tauri dev
```

## Usage

```
フロントエンドツールVite
src/main.tsx

Reactコンポーネント
src/App.tsx

コアプロセス(Rust)
src-tauri/src/main.rs
```
