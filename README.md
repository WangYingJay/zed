# Zed（精简二开）

本仓库是基于 [Zed](https://github.com/zed-industries/zed) 的**精简二开**，面向更低内存占用与本地编辑器场景（精简协作/云端/AI 等默认路径，并做了界面汉化）。**不是** [Zed Industries](https://zed.dev) 的官方发行版，也不代表官方。

## 致谢

感谢 [Zed Industries](https://zed.dev) 将 Zed 以开源形式发布。本仓的代码、架构与大量能力都建立在官方项目之上；若你需要完整、官方维护的 Zed，请前往：

**https://github.com/zed-industries/zed**

## 安装包（本仓 Releases）

本仓 [Releases](https://github.com/WangYingJay/zed/releases) 提供可直接安装的构建产物（随发布更新）：

- **macOS**：arm64 `.dmg`
- **Windows**：x64 `.exe`

也可自行从源码构建（见下方）。

### 从源码构建

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### 许可

上游 Zed 源码主要以 GPL-3.0-or-later 授权，部分组件为 Apache-2.0（以源码标注为准）。第三方依赖许可须符合 CI 要求；细节见上游文档与 `script/licenses/`。

本二开同样遵循相应开源许可；使用或再分发前请自行确认合规。
