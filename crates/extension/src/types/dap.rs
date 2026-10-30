//! B档第九刀：extension 不再依赖 dap crate；以下为 WASM/扩展 API 所需的最小类型占位。

use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use gpui::SharedString;
use serde::{Deserialize, Serialize};
pub use task::{
    AttachRequest, BuildTaskDefinition, DebugRequest, DebugScenario, LaunchRequest,
    TaskTemplate as BuildTaskTemplate, TcpArgumentsTemplate,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartDebuggingRequestArgumentsRequest {
    Launch,
    Attach,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartDebuggingRequestArguments {
    pub configuration: serde_json::Value,
    pub request: StartDebuggingRequestArgumentsRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpArguments {
    pub host: IpAddr,
    pub port: u16,
    pub timeout: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugTaskDefinition {
    pub label: SharedString,
    pub adapter: SharedString,
    pub config: serde_json::Value,
    pub tcp_connection: Option<TcpArgumentsTemplate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugAdapterBinary {
    pub command: Option<String>,
    pub arguments: Vec<String>,
    pub envs: HashMap<String, String>,
    pub cwd: Option<PathBuf>,
    pub connection: Option<TcpArguments>,
    pub request_args: StartDebuggingRequestArguments,
}
