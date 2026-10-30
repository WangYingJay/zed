//! B档第九刀：DAP 已从默认依赖图切除；本模块为空壳，仅保留类型与签名供编译。

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use gpui::{
    App, AppContext, Context, Entity, EntityId, EventEmitter, SharedString, Task, WeakEntity,
};
use language::Buffer;
use rpc::AnyProtoClient;
use serde::{Deserialize, Serialize};
use text::Anchor;
use crate::buffer_store::BufferStore;
use crate::worktree_store::WorktreeStore;

/// 原 `dap` crate 中仍被 project/editor 引用的最小类型集合（本地定义，不拉 dap 依赖）。
pub mod dap_compat {
    use super::*;
    use std::fmt;

    pub type StackFrameId = u64;
    pub type VariableReference = u64;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct SessionId(pub u32);

    impl SessionId {
        pub fn from_proto(client_id: u64) -> Self {
            Self(client_id as u32)
        }

        pub fn to_proto(self) -> u64 {
            self.0 as u64
        }
    }

    /// 空占位，避免 project 里 `DebugAdapterClientState` 等类型无法编译。
    pub struct DebugAdapterClient;

    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deserialize,
        Serialize,
        schemars::JsonSchema,
    )]
    #[serde(transparent)]
    pub struct DebugAdapterName(pub SharedString);

    impl std::ops::Deref for DebugAdapterName {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::convert::AsRef<str> for DebugAdapterName {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl std::borrow::Borrow<str> for DebugAdapterName {
        fn borrow(&self) -> &str {
            &self.0
        }
    }

    impl std::fmt::Display for DebugAdapterName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.0, f)
        }
    }

    impl From<DebugAdapterName> for SharedString {
        fn from(name: DebugAdapterName) -> Self {
            name.0
        }
    }

    impl From<SharedString> for DebugAdapterName {
        fn from(name: SharedString) -> Self {
            DebugAdapterName(name)
        }
    }

    impl<'a> From<&'a str> for DebugAdapterName {
        fn from(str: &'a str) -> Self {
            DebugAdapterName(str.to_string().into())
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum VariableScope {
        Local,
        Global,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum VariableLookupKind {
        Variable,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct InlineValueLocation {
        pub variable_name: String,
        pub scope: VariableScope,
        pub lookup: VariableLookupKind,
        pub row: usize,
        pub column: usize,
    }

    #[derive(Clone, Debug, Default)]
    pub struct EvaluateResponse {
        pub result: String,
        pub type_: Option<String>,
        pub variables_reference: VariableReference,
    }

    #[derive(Clone, Debug)]
    pub struct Variable {
        pub name: String,
        pub value: String,
        pub type_: Option<String>,
        pub evaluate_name: Option<String>,
        pub variables_reference: VariableReference,
    }
}

pub use dap_compat::{
    DebugAdapterClient, DebugAdapterName, EvaluateResponse, InlineValueLocation, SessionId,
    Variable, VariableLookupKind, VariableReference, VariableScope,
};

pub mod breakpoint_store {
    use super::*;
    use dap_compat::StackFrameId;

    pub use super::dap_compat::SessionId;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BreakpointWithPosition {
        pub position: Anchor,
        pub bp: Breakpoint,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct BreakpointSessionState {
        pub id: u64,
        pub verified: bool,
    }

    #[derive(Clone, PartialEq)]
    pub struct ActiveStackFrame {
        pub session_id: SessionId,
        pub thread_id: super::session::ThreadId,
        pub stack_frame_id: StackFrameId,
        pub path: Arc<Path>,
        pub position: Anchor,
    }

    pub struct BreakpointStore {
        _mode: (),
    }

    impl BreakpointStore {
        /// RPC 注册在完整 DAP 路径启用；默认树为空操作。
        pub fn init(_client: &AnyProtoClient) {}

        pub fn local(
            _worktree_store: Entity<WorktreeStore>,
            _buffer_store: Entity<BufferStore>,
        ) -> Self {
            Self { _mode: () }
        }

        // 参数顺序与旧 BreakpointStore::remote 一致，避免改 project.rs 调用点。
        pub fn remote(
            _project_id: u64,
            _upstream_client: AnyProtoClient,
            _buffer_store: Entity<BufferStore>,
            _worktree_store: Entity<WorktreeStore>,
        ) -> Self {
            Self { _mode: () }
        }

        pub fn shared(&mut self, _project_id: u64, _downstream_client: AnyProtoClient) {}

        pub(crate) fn unshared(&mut self, _cx: &mut Context<Self>) {}

        pub(crate) fn broadcast(&self) {}

        pub fn abs_path_from_buffer(_buffer: &Entity<Buffer>, _cx: &App) -> Option<Arc<Path>> {
            None
        }

        pub fn toggle_breakpoint(
            &mut self,
            _buffer: Entity<Buffer>,
            _breakpoint: BreakpointWithPosition,
            _edit_action: BreakpointEditAction,
            _cx: &mut Context<Self>,
        ) {
        }

        pub fn on_file_rename(
            &mut self,
            _old_path: &Path,
            _new_path: &Path,
            _cx: &mut Context<Self>,
        ) {
        }

        pub fn clear_breakpoints(&mut self, _cx: &mut Context<Self>) {}

        pub fn breakpoints<'a>(
            &self,
            _buffer: &Entity<Buffer>,
            _range: Option<std::ops::Range<Anchor>>,
            _snapshot: &language::BufferSnapshot,
            _cx: &App,
        ) -> impl Iterator<Item = (BreakpointWithPosition, Option<BreakpointSessionState>)> + 'a {
            std::iter::empty()
        }

        pub fn active_position(&self) -> Option<&ActiveStackFrame> {
            None
        }

        pub fn active_debug_line_pane_id(&self) -> Option<EntityId> {
            None
        }

        pub fn set_active_debug_pane_id(&mut self, _pane_id: EntityId) {}

        pub fn remove_active_position(
            &mut self,
            _session_id: SessionId,
            _cx: &mut Context<Self>,
        ) {
        }

        pub fn set_active_position(
            &mut self,
            _position: ActiveStackFrame,
            _cx: &mut Context<Self>,
        ) {
        }

        pub fn breakpoint_at_row(
            &self,
            _buffer: &Entity<Buffer>,
            _row: u32,
            _cx: &App,
        ) -> Option<BreakpointWithPosition> {
            None
        }

        pub fn breakpoints_from_path(&self, _path: &Arc<Path>) -> Vec<BreakpointWithPosition> {
            Vec::new()
        }

        pub fn source_breakpoints_from_path(
            &self,
            _path: &Arc<Path>,
            _cx: &App,
        ) -> Vec<SourceBreakpoint> {
            Vec::new()
        }

        pub fn all_breakpoints(&self) -> BTreeMap<Arc<Path>, Vec<BreakpointWithPosition>> {
            BTreeMap::new()
        }

        pub fn all_source_breakpoints(
            &self,
            _cx: &App,
        ) -> BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> {
            BTreeMap::new()
        }

        pub fn with_serialized_breakpoints(
            &mut self,
            _breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
            _cx: &mut Context<Self>,
        ) -> Task<()> {
            // B档第九刀：断点持久化 no-op，仍返回可 await 的 Task 以兼容 workspace 恢复路径。
            Task::ready(())
        }

        #[cfg(any(test, feature = "test-support"))]
        pub(crate) fn breakpoint_paths(&self) -> Vec<Arc<Path>> {
            Vec::new()
        }
    }

    impl EventEmitter<BreakpointStoreEvent> for BreakpointStore {}

    #[derive(Clone, Copy)]
    pub enum BreakpointUpdatedReason {
        Toggled,
        FileSaved,
    }

    pub enum BreakpointStoreEvent {
        SetDebugLine,
        ClearDebugLines,
        BreakpointsUpdated(Arc<Path>, BreakpointUpdatedReason),
        BreakpointsCleared(Vec<Arc<Path>>),
    }

    #[derive(Clone, Debug)]
    pub enum BreakpointEditAction {
        Toggle,
        InvertState,
        EditLogMessage(Arc<str>),
        EditCondition(Arc<str>),
        EditHitCondition(Arc<str>),
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum BreakpointState {
        Enabled,
        Disabled,
    }

    impl BreakpointState {
        pub fn is_enabled(&self) -> bool {
            matches!(self, BreakpointState::Enabled)
        }

        pub fn is_disabled(&self) -> bool {
            matches!(self, BreakpointState::Disabled)
        }

        pub fn to_int(self) -> i32 {
            match self {
                BreakpointState::Enabled => 0,
                BreakpointState::Disabled => 1,
            }
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct Breakpoint {
        pub message: Option<Arc<str>>,
        pub hit_condition: Option<Arc<str>>,
        pub condition: Option<Arc<str>>,
        pub state: BreakpointState,
    }

    impl Breakpoint {
        pub fn new_standard() -> Self {
            Self {
                state: BreakpointState::Enabled,
                hit_condition: None,
                condition: None,
                message: None,
            }
        }

        pub fn new_condition(hit_condition: &str) -> Self {
            Self {
                state: BreakpointState::Enabled,
                condition: None,
                hit_condition: Some(hit_condition.into()),
                message: None,
            }
        }

        pub fn new_log(log_message: &str) -> Self {
            Self {
                state: BreakpointState::Enabled,
                hit_condition: None,
                condition: None,
                message: Some(log_message.into()),
            }
        }

        pub fn is_enabled(&self) -> bool {
            self.state.is_enabled()
        }

        pub fn is_disabled(&self) -> bool {
            self.state.is_disabled()
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct SourceBreakpoint {
        pub row: u32,
        pub path: Arc<Path>,
        pub message: Option<Arc<str>>,
        pub condition: Option<Arc<str>>,
        pub hit_condition: Option<Arc<str>>,
        pub state: BreakpointState,
    }
}

pub mod session {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct ThreadId(pub i64);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ThreadStatus {
        Running,
        Stepping,
        Stopped,
        Exited,
        Ended,
    }

    pub struct Session {
        id: SessionId,
    }

    impl Session {
        pub fn session_id(&self) -> SessionId {
            self.id
        }

        pub fn adapter(&self) -> DebugAdapterName {
            DebugAdapterName(SharedString::new_static("stub"))
        }

        pub fn label(&self) -> Option<SharedString> {
            None
        }

        pub fn is_started(&self) -> bool {
            true
        }

        pub fn is_building(&self) -> bool {
            false
        }

        pub fn evaluate_hover_expression(
            &mut self,
            _stack_frame_id: dap_compat::StackFrameId,
            _expression: String,
            _cx: &mut Context<Self>,
        ) -> Task<Option<dap_compat::EvaluateResponse>> {
            Task::ready(None)
        }

        pub fn load_hover_children(
            &mut self,
            _variables_reference: dap_compat::VariableReference,
            _cx: &mut Context<Self>,
        ) -> Task<Result<Vec<dap_compat::Variable>>> {
            Task::ready(Ok(Vec::new()))
        }

        pub fn add_watcher(
            &mut self,
            _expression: SharedString,
            _frame_id: u64,
            _cx: &mut Context<Self>,
        ) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }

        pub fn any_stopped_thread(&self) -> bool {
            false
        }
    }

    pub enum SessionEvent {
        InvalidateInlineValue,
        #[allow(dead_code)]
        Stopped(ThreadStatus),
    }

    pub enum SessionStateEvent {
        #[allow(dead_code)]
        Shutdown,
        #[allow(dead_code)]
        Running,
        #[allow(dead_code)]
        Restart,
    }

    pub struct SessionQuirks;

    impl EventEmitter<SessionEvent> for Session {}
    impl EventEmitter<SessionStateEvent> for Session {}
}

pub mod dap_store {
    use super::*;
    use dap_compat::StackFrameId;

    pub enum DapStoreEvent {
        #[allow(dead_code)]
        DebugClientStarted(SessionId),
        #[allow(dead_code)]
        DebugClientShutdown(SessionId),
        Notification(String),
    }

    pub struct LocalDapStore;
    pub struct RemoteDapStore;

    pub struct DapStore {
        breakpoint_store: Entity<breakpoint_store::BreakpointStore>,
    }

    impl DapStore {
        pub fn init(_client: &AnyProtoClient, _cx: &mut App) {}

        pub fn new_local(
            _http_client: Arc<dyn http_client::HttpClient>,
            _node_runtime: node_runtime::NodeRuntime,
            _fs: Arc<dyn fs::Fs>,
            _environment: Entity<crate::ProjectEnvironment>,
            _toolchain_store: Arc<dyn language::LanguageToolchainStore>,
            _worktree_store: Entity<WorktreeStore>,
            breakpoint_store: Entity<breakpoint_store::BreakpointStore>,
            _is_headless: bool,
            _cx: &mut Context<Self>,
        ) -> Self {
            Self { breakpoint_store }
        }

        pub fn new_remote(
            _project_id: u64,
            _remote_client: Entity<remote::RemoteClient>,
            breakpoint_store: Entity<breakpoint_store::BreakpointStore>,
            _worktree_store: Entity<WorktreeStore>,
            _node_runtime: node_runtime::NodeRuntime,
            _http_client: Arc<dyn http_client::HttpClient>,
            _fs: Arc<dyn fs::Fs>,
            _cx: &mut Context<Self>,
        ) -> Self {
            Self { breakpoint_store }
        }

        pub fn new_collab(
            _project_id: u64,
            _upstream_client: AnyProtoClient,
            breakpoint_store: Entity<breakpoint_store::BreakpointStore>,
            _worktree_store: Entity<WorktreeStore>,
            _fs: Arc<dyn fs::Fs>,
            _cx: &mut Context<Self>,
        ) -> Self {
            Self { breakpoint_store }
        }

        pub fn sessions(&self) -> impl Iterator<Item = &Entity<session::Session>> {
            std::iter::empty()
        }

        pub fn session_by_id(&self, _session_id: SessionId) -> Option<Entity<session::Session>> {
            None
        }

        pub fn debug_scenario_for_build_task(
            &self,
            _build: task::TaskTemplate,
            _adapter: DebugAdapterName,
            _label: SharedString,
            _cx: &mut App,
        ) -> Task<Option<task::DebugScenario>> {
            Task::ready(None)
        }

        pub fn resolve_inline_value_locations(
            &mut self,
            _session: Entity<session::Session>,
            _stack_frame_id: StackFrameId,
            _buffer: Entity<Buffer>,
            _inline_value_locations: Vec<dap_compat::InlineValueLocation>,
            _cx: &mut Context<Self>,
        ) -> Task<Result<Vec<crate::InlayHint>>> {
            Task::ready(Ok(Vec::new()))
        }

        pub fn shutdown_sessions(&mut self, _cx: &mut Context<Self>) -> Task<()> {
            Task::ready(())
        }

        pub fn shared(
            &mut self,
            _project_id: u64,
            _downstream_client: AnyProtoClient,
            _cx: &mut Context<Self>,
        ) {
        }

        pub fn unshared(&mut self, _cx: &mut Context<Self>) {}

        pub fn breakpoint_store(&self) -> &Entity<breakpoint_store::BreakpointStore> {
            &self.breakpoint_store
        }
    }

    impl EventEmitter<DapStoreEvent> for DapStore {}
}

pub mod dap_command {
    pub struct EvaluateCommand;
    pub struct DataBreakpointContext;
    pub struct ReadMemory;
}

pub mod memory {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MemoryCell(pub Option<u8>);
}

pub mod locators {}

#[cfg(any(feature = "test-support", test))]
pub mod test {
    use super::*;

    pub fn intercept_debug_sessions<T: Fn(&()) + 'static>(
        _cx: &mut gpui::TestAppContext,
        _configure: T,
    ) -> gpui::Subscription {
        gpui::Subscription::new(|| {})
    }
}

pub use memory::MemoryCell;
