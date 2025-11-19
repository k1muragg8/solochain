# RocksDB 编译修复说明

## 问题描述

`librocksdb-sys` 包中的某些头文件缺少 `#include <cstdint>`，导致编译时出现 `uint64_t` 和 `uint32_t` 未定义的错误。

## 解决方案

项目在 `node/build.rs` 中包含了自动补丁脚本，会在每次编译前自动修复以下文件：

1. `rocksdb/db/blob/blob_file_meta.h` - 添加 `#include <cstdint>`
2. `rocksdb/include/rocksdb/trace_record.h` - 添加 `#include <cstdint>`

## 工作原理

构建脚本会在以下位置查找 `librocksdb-sys` 源码：
- `~/.cargo/registry/src/index.crates.io-*/librocksdb-sys-0.11.0+8.1.1`
- `~/.cargo/git/checkouts/*/librocksdb-sys-0.11.0+8.1.1`

如果找到文件且尚未修复，会自动应用补丁。

## 注意事项

- 补丁会在每次编译时自动应用，无需手动操作
- 如果文件已经包含 `#include <cstdint>`，脚本会跳过修复
- 如果找不到 `librocksdb-sys` 源码（例如首次编译时依赖尚未下载），脚本会静默跳过，不影响编译流程

