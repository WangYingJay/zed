//! B档第九刀：Dev Container 建议与 CLI 自动打开路径 stub（已去掉 `dev_container` crate 依赖）。

use gpui::{Context, Window};
use project::{Project, WorktreeId};
use workspace::Workspace;
use worktree::UpdatedEntriesSet;

/// B档第九刀：不再扫描 devcontainer 配置或弹建议通知。
pub fn suggest_on_worktree_updated(
    workspace: &mut Workspace,
    _worktree_id: WorktreeId,
    _updated_entries: &UpdatedEntriesSet,
    _project: &gpui::Entity<Project>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if workspace.open_in_dev_container() {
        open_dev_container_from_cli(workspace, window, cx);
    }
}

/// B档第九刀：清掉 CLI 标志并记录日志，不 dispatch OpenDevContainer。
pub fn open_dev_container_from_cli(
    workspace: &mut Workspace,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    workspace.set_open_in_dev_container(false);
    log::warn!("--dev-container: Dev Containers are disabled in this build");
}
