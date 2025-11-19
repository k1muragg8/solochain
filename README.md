# Substrate 单链节点模板

一个基于 [Substrate](https://substrate.io/) 框架的区块链节点模板，开箱即用 🚀

本模板是 [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) 中 Solochain 模板的独立版本。如需启动新项目，建议使用独立版本。所有问题、建议和功能请求请提交到 [Substrate 上游仓库](https://github.com/paritytech/polkadot-sdk/tree/master/substrate)。

## 快速开始

### 环境要求

根据操作系统和 Rust 版本，可能需要安装额外的依赖包。请查看 [Substrate 安装指南](https://docs.substrate.io/install/) 了解各平台的依赖要求。也可以使用 [替代安装方式](#替代安装方式)。

### 获取代码

```sh
git clone https://github.com/paritytech/polkadot-sdk-solochain-template.git solochain-template

cd solochain-template
```

### 编译

使用以下命令编译节点（不启动）：

```sh
cargo build --release
```

> **注意**：
> - 如果看到 `trie-db` 的 future incompatibility 警告，可以忽略，这来自依赖项，不影响编译和运行
> - 项目已包含自动修复脚本，会在编译时自动修复 `librocksdb-sys` 的编译问题（详见 [ROCKSDB_PATCH.md](./ROCKSDB_PATCH.md)）

### 查看帮助

编译完成后，可以使用以下命令查看节点的参数和子命令：

```sh
./target/release/solochain-template-node -h
```

### 生成文档

生成并查看 Rust 文档：

```sh
cargo +nightly doc --open
```

## 运行节点

### 单节点开发链

启动一个不持久化状态的单节点开发链：

```sh
./target/release/solochain-template-node --dev
```

清除开发链状态：

```sh
./target/release/solochain-template-node purge-chain --dev
```

启用详细日志：

```sh
RUST_BACKTRACE=1 ./target/release/solochain-template-node -ldebug --dev
```

**开发链特性**：
- 节点运行期间状态保存在 `tmp` 文件夹
- 使用 **Alice** 和 **Bob** 账户作为默认验证者
- 使用 **Alice** 账户作为默认 `sudo` 账户
- 预配置了包含多个预充值开发账户的创世状态（见 `/node/src/chain_spec.rs`）

### 持久化状态

如需在多次运行间保持链状态，可以指定基础路径：

```sh
# 创建用于存储链状态的文件夹
mkdir my-chain-state

# 使用该文件夹存储链状态
./target/release/solochain-template-node --dev --base-path ./my-chain-state/

# 查看运行后创建的文件结构
ls ./my-chain-state
# chains
ls ./my-chain-state/chains/
# dev
ls ./my-chain-state/chains/dev
# db keystore network
```

### 连接前端界面

启动本地节点后，可以通过 [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) 连接到本地节点端点进行交互。也可以使用 [IPFS 托管版本](https://dotapps.io/)。前端源码和部署说明见 [`polkadot-js/apps`](https://github.com/polkadot-js/apps) 仓库。

### 多节点本地测试网

如需查看多节点共识算法运行情况，请参考 [模拟网络教程](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/)。

## 项目结构

Substrate 项目由分布在多个目录中的组件组成。

### 节点 (Node)

区块链节点是允许用户参与区块链网络的应用。基于 Substrate 的节点提供以下功能：

- **网络**：使用 [`libp2p`](https://libp2p.io/) 网络栈实现节点间通信
- **共识**：支持自定义共识引擎，内置基于 [Web3 Foundation 研究](https://research.web3.foundation/Polkadot/protocols/NPoS) 的多种共识机制
- **RPC 服务器**：提供远程过程调用接口与节点交互

`node` 目录中的重要文件：

- **`chain_spec.rs`**：定义链的初始（创世）状态。注意 `development_config` 和 `testnet_genesis` 函数，它们定义了本地开发链的创世状态，使用[已知账户](https://docs.substrate.io/reference/command-line-tools/subkey/)配置区块链的初始状态
- **`service.rs`**：定义节点实现，包含共识相关功能，如区块最终化和分叉处理，以及 Aura（区块生产）和 GRANDPA（最终性）等共识机制

### 运行时 (Runtime)

在 Substrate 中，"运行时"和"状态转换函数"是同义词，指区块链的核心逻辑，负责验证区块并执行状态变更。本项目使用 [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) 构建运行时。

查看 [`runtime/src/lib.rs`](./runtime/src/lib.rs) 了解：

- 运行时配置了多个 pallet，每个 pallet 的配置通过 `impl $PALLET_NAME::Config for Runtime` 代码块定义
- 通过 [`#[runtime]`](https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.runtime.html) 宏将所有 pallet 组合成单一运行时

### Pallets（功能模块）

运行时由多个 FRAME pallet 构建，这些 pallet 来自 [Substrate 仓库](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame)，以及一个在 [`pallets`](./pallets/template/src/lib.rs) 目录中定义的模板 pallet。

FRAME pallet 包含以下区块链原语：

- **存储**：强大的[存储抽象](https://docs.substrate.io/build/runtime-storage/)，便于使用 Substrate 的高效键值数据库管理区块链状态
- **可调度函数**：可以从运行时外部调用以更新状态的函数
- **事件**：用于通知用户重要的状态变更
- **错误**：可调度函数失败时返回的错误类型

每个 pallet 都有自己的 `Config` trait，作为配置接口，用于泛型定义其依赖的类型和参数。

## 替代安装方式

### Nix

安装 [nix](https://nixos.org/) 和 [nix-direnv](https://github.com/nix-community/nix-direnv) 可获得即插即用的开发环境。运行 `direnv allow` 激活 direnv 以获取所有正确的依赖。

### Docker

请按照 [Substrate Docker 说明](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/docker/README.md) 构建包含 Substrate 节点模板二进制文件的 Docker 容器。

## 编译问题修复

项目已包含自动修复脚本，会在编译时自动修复 `librocksdb-sys` 的编译问题。详情请参考 [ROCKSDB_PATCH.md](./ROCKSDB_PATCH.md)。

## 许可证

本项目采用 MIT-0 许可证。
