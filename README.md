# Substrate 单链节点模板 (SGC 定制版)

一个基于 [Substrate](https://substrate.io/) 框架的区块链节点模板，已针对 SGC 网络进行定制。🚀

本模板是 [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) 中 Solochain 模板的独立版本。

## ✨ 核心功能与定制

本项目在原版模板基础上进行了以下定制：

### 🪙 代币经济 (SGC)
- **代币符号**: `SGC`
- **精度**: 12 (1 SGC = 1,000,000,000,000)
- **初始供应**:
  - 开发模式 (`--dev`): Alice, Bob 等预设账户各 1000 SGC
  - 本地测试网: 预设账户各 1000 SGC

### ⛏️ 区块奖励与激励
- **区块奖励**: 10 SGC / 区块
  - 奖励直接发放给打包该区块的节点（区块作者）
- **交易小费**:
  - 交易的基础费用 + 用户支付的小费全部发放给打包该区块的节点

### ⏱️ 网络参数
- **出块时间**: 7 秒 (原版为 6 秒)
- **时间单位**:
  - 1 分钟 ≈ 8 个区块
  - 1 小时 ≈ 480 个区块
  - 1 天 ≈ 11520 个区块

## 🛠️ 编译修复 (RocksDB)

本项目包含自动修复脚本，解决 `librocksdb-sys` 编译时缺少 `#include <cstdint>` 的问题。

- **自动修复**: `node/build.rs` 会在编译前自动检查并修复 RocksDB 头文件。
- **无需手动干预**: 直接运行 `cargo build` 即可。

## 🚀 快速开始



### 1. 编译节点

```sh
cargo build --release
```

### 2. 运行节点

**启动单节点开发链（数据不持久化）：**

```sh
./target/release/solochain-template-node --dev --tmp
```

- `--dev`: 启用开发模式（预设 Alice 验证者）
- `--tmp`: 每次启动清除数据，确保使用最新 Runtime

**启动持久化节点：**

```sh
mkdir my-chain-data
./target/release/solochain-template-node --dev --base-path ./my-chain-data
```

## 🔗 连接前端

启动节点后，打开 [Polkadot.js Apps](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) 连接到本地节点。

## 📂 项目结构

- **`node/`**: 节点逻辑（RPC, Service, CLI）
  - `chain_spec.rs`: 链配置（代币符号、初始账户）
  - `build.rs`: 包含 RocksDB 自动修复脚本
- **`runtime/`**: 链上逻辑（Runtime）
  - `src/lib.rs`: Runtime 配置（出块时间、Pallet 组合）
  - `src/genesis_config_presets.rs`: 创世区块配置（初始余额）
- **`pallets/template/`**: 自定义功能模块
  - `src/lib.rs`: 包含区块奖励和交易费处理逻辑

## 📄 许可证

本项目采用 MIT-0 许可证。
