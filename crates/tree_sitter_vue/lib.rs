//! Vendored [tree-sitter-vue](https://github.com/tree-sitter-grammars/tree-sitter-vue) bindings.
//!
//! B档精简不加载 extension_host，无法安装官方 Vue 扩展；把 grammar 收进内置白名单供 `.vue` 识别与高亮。

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_vue() -> *const ();
}

/// tree-sitter LanguageFn for Vue SFC.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_vue) };

/// Static node-types.json from the upstream grammar.
pub const NODE_TYPES: &str = include_str!("src/node-types.json");
