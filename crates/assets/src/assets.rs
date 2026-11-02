// This crate was essentially pulled out verbatim from main `zed` crate to avoid having to run RustEmbed macro whenever zed has to be rebuilt. It saves a second or two on an incremental build.

use anyhow::Context as _;
use gpui::{App, AssetSource, Result, SharedString};

// Release builds embed the assets; dev builds read them from the checkout at
// runtime so edits show up on the next launch without a rebuild and no
// build-time path is baked in (which corgi's sandbox rejects). See
// `util::fs_embed!`.
//
// B档启动内存刀：默认不嵌入 calls/AI 专用声音与 prompts；字体只嵌首屏必需字重；
// 排除已砍功能相关图标/营销图，减小二进制映射与冷启动常驻。
util::fs_embed! {
    pub struct Assets,
    crate_relative = "../../assets",
    root_relative = "assets",
    include = [
        // UI：Regular + SemiBold；缓冲：Regular + Bold + Italic（斜体用于注释等）
        "fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf",
        "fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf",
        "fonts/ibm-plex-sans/license.txt",
        "fonts/lilex/Lilex-Regular.ttf",
        "fonts/lilex/Lilex-Bold.ttf",
        "fonts/lilex/Lilex-Italic.ttf",
        "fonts/lilex/OFL.txt",
        "icons/**/*",
        "images/zed_logo.svg",
        "images/grid.svg",
        "themes/**/*",
        "*.md",
    ],
    exclude = [
        "themes/src/*",
        "*.DS_Store",
        // 已卸 AI / 协作 / 编辑预测相关图标（IconName 仍可能引用；默认路径不渲染）
        "icons/ai_*.svg",
        "icons/zed_agent*.svg",
        "icons/zed_predict*.svg",
        "icons/copilot*.svg",
        "icons/on_call.svg",
        "icons/mic.svg",
        "icons/mic_mute.svg",
    ],
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Self::get(path)
            .map(|f| Some(f.data))
            .with_context(|| format!("loading asset at path {path:?}"))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Assets {
    /// Populate the [`TextSystem`] of the given [`AppContext`] with all `.ttf` fonts in the `fonts` directory.
    pub fn load_fonts(&self, cx: &App) -> anyhow::Result<()> {
        let font_paths = self.list("fonts")?;
        let mut embedded_fonts = Vec::new();
        for font_path in font_paths {
            if font_path.ends_with(".ttf") {
                let font_bytes = cx
                    .asset_source()
                    .load(&font_path)?
                    .expect("Assets should never return None");
                embedded_fonts.push(font_bytes);
            }
        }

        cx.text_system().add_fonts(embedded_fonts)
    }

    pub fn load_test_fonts(&self, cx: &App) {
        cx.text_system()
            .add_fonts(vec![
                self.load("fonts/lilex/Lilex-Regular.ttf").unwrap().unwrap(),
            ])
            .unwrap()
    }
}
