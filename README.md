# Tauri Python Quickstart

本项目旨在提供一个快速开发 [Tauri](https://tauri.app/) + [Python](https://www.python.org/) 应用的脚手架，适合熟悉 Vue.js 和 Python 的开发者。利用 `pyo3` ，将 Python 脚本无缝集成到 Tauri 桌面应用中，并搭配 Vue 3 构建前端界面。

## 特性

* **Tauri + Python 集成**：通过 `pyo3` embed_python 方式深度集成，提供原生应用级的体验。
* **Vue 3 前端**：采用 Vue 3 作为前端框架，享受其高性能和开发便捷性。
* **快速启动**：预设了基础项目结构和配置，节省从零开始搭建的时间。
* **易于扩展**：前后端分离但又紧密集成，方便后续功能扩展和维护。

## 目录结构

* `src-tauri/`：Tauri 应用的主目录，包含 Rust 后端。
* `src/`：Vue 3 前端应用源代码。
* `src-tauri/python_scripts`：Python 脚本位置。
* `doc/`：项目相关文档，包括使用方式、架构说明等。

## 快速开始

### 1. 环境准备

在开始之前，请确保您的系统已安装以下工具：

* **Rust**：推荐使用 [rustup](https://rustup.rs/) 安装。
* **Node.js & npm/yarn**：推荐使用 [nvm](https://github.com/nvm-sh/nvm) 管理 Node.js 版本。
* **Python 3.x**：确保安装了 Python 3。
* **Tauri 前置依赖**: 根据您的操作系统，可能需要安装额外的构建工具。请参考 [Tauri 官方文档](https://tauri.studio/v1/guides/getting-started/prerequisites) 获取详细信息。

### 2. 安装依赖

#### 安装 Python 依赖

```bash
cd src-tauri/python_scripts
python -m venv .venv
.venv\Scripts\activate.bat
pip install -r requirements.txt
```

#### 安装 vue3 依赖

```bash
npm install
```

#### 安装 Tauri Rust 依赖

```bash
cd src-tauri
cargo build # 第一次构建可能需要下载大量依赖
```

### 3. 本地开发

#### 单独启动前端开发服务器

在 `src/` 目录下运行：

```bash
cd src
npm run dev
```

#### 单独编译后端rust

```bash
cd src-tauri
cargo build
```

#### 启动应用

```bash
npm run tauri dev # tauri应用中，前端vue会自动热重载，后端rust会自动差量编译
```


### 4. 构建发布版本


```bash
cargo tauri build
```

这将在 `src-tauri/target/` 目录下生成可安装的应用程序包。

## 文档

更多详细的使用说明、架构设计和常见问题解答，请查阅 `doc/` 文件夹下的文档：
