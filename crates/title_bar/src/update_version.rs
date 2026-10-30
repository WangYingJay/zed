use gpui::{Empty, Render};
use ui::prelude::*;

/// B档第五刀：自动更新已从默认路径卸下，标题栏更新条恒空。
pub struct UpdateVersion;

impl UpdateVersion {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn update_simulation(&mut self, _cx: &mut Context<Self>) {}

    pub fn show_update_in_menu_bar(&self) -> bool {
        false
    }
}

impl Render for UpdateVersion {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
