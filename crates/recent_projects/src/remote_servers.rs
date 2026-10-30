//! B档第六刀：RemoteServerProjects 降为 stub，不再硬链 remote_connection。

use std::sync::Arc;

use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, WeakEntity,
    Window,
};
use project::Fs;
use settings::Settings;
use ui::prelude::*;
use workspace::{AppState, DismissDecision, ModalView, Workspace};

use crate::remote_connections::RemoteSettings;

pub struct RemoteServerProjects {
    focus_handle: FocusHandle,
}

impl EventEmitter<DismissEvent> for RemoteServerProjects {}

impl Focusable for RemoteServerProjects {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RemoteServerProjects {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }
}

impl RemoteServerProjects {
    #[cfg(target_os = "windows")]
    pub fn wsl(
        _create_new_window: bool,
        _fs: Arc<dyn Fs>,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn new(
        _create_new_window: bool,
        _fs: Arc<dyn Fs>,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        // 确保 RemoteSettings 已注册（settings 侧仍可读 ssh_connections 等配置）
        let _ = RemoteSettings::get_global(cx);
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn new_dev_container(
        _fs: Arc<dyn Fs>,
        _configs: Vec<dev_container::DevContainerConfig>,
        _app_state: Arc<AppState>,
        _dev_container_context: Option<dev_container::DevContainerContext>,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn popover(
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        create_new_window: Option<bool>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let create_new_window =
            create_new_window.unwrap_or_else(|| crate::default_open_in_new_window(cx));
        cx.new(|cx| {
            let server = Self::new(create_new_window, fs, window, workspace, cx);
            server.focus_handle(cx).focus(window, cx);
            server
        })
    }
}

impl Render for RemoteServerProjects {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = RemoteSettings::get_global(cx);
        v_flex()
            .track_focus(&self.focus_handle(cx))
            .w(px(420.))
            .p_4()
            .gap_2()
            .child(Label::new("Remote projects disabled").size(LabelSize::Large))
            .child(
                Label::new("This build does not include remote connection UI.")
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
    }
}
