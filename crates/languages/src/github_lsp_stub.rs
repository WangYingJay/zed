//! B 档默认构建：不从 GitHub 下载 LSP 二进制；保留与 `github_releases`  crate 兼容的类型与签名。

use anyhow::{Context as _, Result, bail};
use gpui::{BackgroundExecutor, Task};
use http_client::HttpClient;
use std::{path::Path, sync::Arc};

const STUB_MESSAGE: &str = "B档：默认不从 GitHub 下载 LSP";

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
    pub digest: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssetKind {
    TarGz,
    TarBz2,
    Gz,
    Zip,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GithubBinaryMetadata {
    pub metadata_version: u64,
    pub digest: Option<String>,
}

impl GithubBinaryMetadata {
    pub async fn read_from_file(metadata_path: &Path) -> Result<GithubBinaryMetadata> {
        let metadata_content = async_fs::read_to_string(metadata_path)
            .await
            .with_context(|| format!("reading metadata file at {metadata_path:?}"))?;
        serde_json::from_str(&metadata_content)
            .with_context(|| format!("parsing metadata file at {metadata_path:?}"))
    }

    pub async fn write_to_file(&self, metadata_path: &Path) -> Result<()> {
        let metadata_content = serde_json::to_string(self)
            .with_context(|| format!("serializing metadata for {metadata_path:?}"))?;
        async_fs::write(metadata_path, metadata_content.as_bytes())
            .await
            .with_context(|| format!("writing metadata file at {metadata_path:?}"))?;
        Ok(())
    }
}

pub async fn latest_github_release(
    _repo_name_with_owner: &str,
    _require_assets: bool,
    _pre_release: bool,
    _http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    bail!("{STUB_MESSAGE}")
}

#[derive(serde::Deserialize, Debug)]
pub struct GithubRelease {
    pub tag_name: String,
    #[serde(rename = "prerelease")]
    pub pre_release: bool,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub digest: Option<String>,
}

pub fn build_asset_url(_repo_name_with_owner: &str, _tag: &str, _kind: AssetKind) -> Result<String> {
    bail!("{STUB_MESSAGE}")
}

pub fn download_server_binary(
    _executor: &BackgroundExecutor,
    _http_client: Arc<dyn HttpClient>,
    _url: &str,
    _digest: Option<&str>,
    _destination_path: &Path,
    _asset_kind: AssetKind,
) -> Task<Result<()>> {
    Task::ready(Err(anyhow::anyhow!("{STUB_MESSAGE}")))
}
