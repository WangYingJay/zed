use gpui::{AsyncApp, actions};

// B档第七刀：常量内联，避免 install_cli → client。
const ZED_URL_SCHEME: &str = "zed";

actions!(
    cli,
    [
        /// Registers the zed:// URL scheme handler.
        RegisterZedScheme
    ]
);

pub async fn register_zed_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    cx.update(|cx| cx.register_url_scheme(ZED_URL_SCHEME)).await
}
