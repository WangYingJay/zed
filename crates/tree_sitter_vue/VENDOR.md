# Vendored tree-sitter-vue

来源：https://github.com/tree-sitter-grammars/tree-sitter-vue  
提交：`7e48557b903a9db9c38cea3b7839ef7e1f36c693`  
许可证：MIT（见上游 LICENSE）

仅 vendor `src/parser.c` / `scanner.c` / `node-types.json` / `tree_sitter/*.h`。
Rust 绑定按本仓 tree-sitter（LanguageFn）手写，因上游 `Cargo.toml` 仍钉旧版 `tree-sitter ~0.20`，无法直接当 git 依赖用。

用途：B档不加载 `extension_host`，官方 Vue 扩展进不了进程；把 `.vue` 收进内置语法白名单。
