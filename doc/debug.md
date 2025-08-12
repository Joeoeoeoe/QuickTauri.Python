# debug.md —— 编程过程中有价值的报错和解决日志

## tailwind

1. 理论操作：
```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

2. `npx tailwindcss init -p` 报错
```bash
E:\coding\tauri\MyToolBox>npx tailwindcss init
npm ERR! could not determine executable to run

npm ERR! A complete log of this run can be found in: D:\Program Files\nodejs\node_cache\_logs\2025-08-10T08_25_18_853Z-debug-0.log
```

3. 报错原因——tailwindcss和postcss版本冲突

4. 解决
```bash
npm uninstall tailwindcss
npm install tailwindcss@3.4.17 -D
```

## Rust模块引入错误

1. 报错信息：

    ```bash
    unresolved import `crate::cache`
    could not find `cache` in the crate root
    ```

2. 解决方式：在 `main.rs` 和 `lib.rs` 中同时使用 `mod cache;`

3. 说明

    >  Rust 项目通常有两种主要的顶级目标：
    > 
    > - 二进制 crate (Executable): 包含 src/main.rs 文件。这是一个可执行程序。
    > - 库 crate (Library): 包含 src/lib.rs 文件。这是一个可以被其他 crate 引用的库。

## 找不到模块“@tauri-apps/api/tauri”

1. 问题来源
   - 这似乎是一个tauri自身的bug，[Cannot import @tauri-apps/api/tauri #1512
](https://github.com/tauri-apps/tauri/issues/1512)

2. 问题表现形式
   - 报错提示：找不到模块“@tauri-apps/api/tauri”或其相应的类型声明。ts(2307)
   - 编译错误：[vite] Internal server error: Failed to resolve import "@tauri-apps/api/tauri" from "src/services/rustService.js". Does the file exist?

3. 解决方式
   ```js
   // 用下述语句进行替换
   import { invoke } from '@tauri-apps/api/core'
   ```

## rust中not used警告

- 添加 #[allow(dead_code)]