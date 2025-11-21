# 修改记录

本文档记录项目的重要修改内容。

## 2025-11-21

### 编译错误和警告修复
- 修复所有 `cargo test` 编译错误
  - 在 `pallets/template/src/lib.rs` 中添加 `extern crate alloc` 声明
  - 添加 `use alloc::vec::Vec` 导入以支持 no_std 环境
  - 在 `pallets/template/src/tests.rs` 中添加 `use frame_support::traits::Hooks` 导入
  
- 修复所有 Clippy 警告
  - 将 `node/build.rs` 中的函数参数从 `&PathBuf` 改为 `&std::path::Path`
  - 移除 `pallets/template/src/lib.rs` 中不必要的 `let _` 绑定
  - 为 Substrate 框架的大错误变体添加 `#[allow(clippy::result_large_err)]` 属性：
    - `node/src/benchmarking.rs`
    - `node/src/command.rs`
    - `node/src/service.rs`
    - `node/src/main.rs`

- 测试结果：✅ 所有 7 个测试通过，无编译错误，无 Clippy 警告

### 功能修复
- 修复区块奖励和交易小费分配逻辑
  - 将奖励发放从 `on_initialize` 移至 `on_finalize`，确保能正确获取区块作者
  - 使用 `frame_system` 的 digest logs 来查找区块作者，解决了无法获取作者导致奖励不发放的问题

## 2025-11-20

### 区块奖励和交易小费修复
- 修复 `pallet_template` 中获取区块作者的逻辑错误
- 实现 `FindAuthor` trait 以正确从 Aura digest 中获取作者
- 确保打包区块交易的小费正确分配给打包区块者

## 2025-11-19

### 代币经济模型配置
- 修改代币名称为 SGC
  - 修改文件：`node/src/chain_spec.rs`
  - 将链名称从 "Development" 和 "Local Testnet" 改为 "SGC Development" 和 "SGC Local Testnet"
  
- 修改初始代币金额
  - 所有环境下每个人的初始金额为 1000 个代币
  - 开发版初始代币金额为 10000 SGC
  - 修改文件：`runtime/src/genesis_config_presets.rs`
  - 将初始代币从 `1u128 << 60` 改为 `10_000 * crate::UNIT`

- 新增区块奖励功能
  - 修改打包区块奖励为 10 个代币
  - 修改文件：`pallets/template/src/lib.rs`
  - 在 `on_initialize` hook 中实现区块作者奖励机制
  - **已修复**：解决类型转换问题，现在区块奖励功能已正常工作

- 确认交易小费功能已启用

### 出块时间调整
- 将出块时间从 6 秒调整为 7 秒
- 修改文件：`runtime/src/lib.rs`
- 修改内容：`MILLI_SECS_PER_BLOCK` 从 6000 毫秒改为 7000 毫秒
- 注意：时间常量（MINUTES、HOURS、DAYS）会根据出块时间自动重新计算
  - 原来：MINUTES = 10 个区块（60 秒），HOURS = 600，DAYS = 14400
  - 现在：MINUTES = 8 个区块（56 秒），HOURS = 480，DAYS = 11520
  - 由于整数除法，MINUTES 实际对应 56 秒而不是精确的 60 秒

### 编译问题修复
- 修复 `librocksdb-sys` 编译错误（缺少 `#include <cstdint>`）
- 添加自动补丁脚本到 `node/build.rs`，编译时自动修复头文件
- 修复文件：
  - `rocksdb/db/blob/blob_file_meta.h`
  - `rocksdb/include/rocksdb/trace_record.h`
- 创建说明文档：`ROCKSDB_PATCH.md`

### 代码警告修复
- 修复 `runtime/src/genesis_config_presets.rs` 中不必要的括号警告
- 在 `runtime/src/lib.rs` 中添加 `#![allow(unused_parens)]` 屏蔽相关警告

### 文档更新
- 将 `README.md` 更新为中文版本，内容更详细且简洁
- 添加编译问题说明和注意事项
