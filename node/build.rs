use std::env;
use std::fs;
use std::path::PathBuf;
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
	generate_cargo_keys();

	rerun_if_git_head_changed();

	// 应用 librocksdb-sys 的补丁
	apply_rocksdb_patches();
}

fn apply_rocksdb_patches() {
	// 查找 librocksdb-sys 的源码目录
	let cargo_home = env::var("CARGO_HOME").unwrap_or_else(|_| {
		let home = env::var("HOME").expect("HOME environment variable not set");
		format!("{}/.cargo", home)
	});

	// 查找可能的 librocksdb-sys 路径
	let registry_path = PathBuf::from(&cargo_home).join("registry/src");
	
	if let Ok(entries) = fs::read_dir(&registry_path) {
		for entry in entries.flatten() {
			let dir_name = entry.file_name();
			if dir_name.to_string_lossy().starts_with("index.crates.io-") {
				let librocksdb_path = entry.path().join("librocksdb-sys-0.11.0+8.1.1");
				if librocksdb_path.exists() {
					fix_blob_file_meta(&librocksdb_path);
					fix_trace_record(&librocksdb_path);
					return;
				}
			}
		}
	}

	// 也检查 git checkouts
	let git_path = PathBuf::from(&cargo_home).join("git/checkouts");
	if let Ok(entries) = fs::read_dir(&git_path) {
		for entry in entries.flatten() {
			let librocksdb_path = entry.path().join("librocksdb-sys-0.11.0+8.1.1");
			if librocksdb_path.exists() {
				fix_blob_file_meta(&librocksdb_path);
				fix_trace_record(&librocksdb_path);
				return;
			}
		}
	}
}

fn fix_blob_file_meta(base_path: &PathBuf) {
	let file_path = base_path.join("rocksdb/db/blob/blob_file_meta.h");
	
	if !file_path.exists() {
		return;
	}

	if let Ok(mut content) = fs::read_to_string(&file_path) {
		// 检查是否已经包含 cstdint
		if content.contains("#include <cstdint>") {
			return;
		}

		// 添加 cstdint include - 在 cassert 之后
		if content.contains("#include <cassert>") && !content.contains("#include <cstdint>") {
			content = content.replace(
				"#include <cassert>\n#include <iosfwd>",
				"#include <cassert>\n#include <cstdint>\n#include <iosfwd>",
			);

							if let Err(e) = fs::write(&file_path, content) {
								eprintln!("cargo:warning=修补 blob_file_meta.h 失败: {}", e);
							} else {
								println!("cargo:warning=已应用补丁到 blob_file_meta.h");
							}		}
	}
}

fn fix_trace_record(base_path: &PathBuf) {
	let file_path = base_path.join("rocksdb/include/rocksdb/trace_record.h");
	
	if !file_path.exists() {
		return;
	}

	if let Ok(mut content) = fs::read_to_string(&file_path) {
		// 检查是否已经包含 cstdint
		if content.contains("#include <cstdint>") {
			return;
		}

		// 添加 cstdint include - 在 memory 之前
		if content.contains("#include <memory>") && !content.contains("#include <cstdint>") {
			content = content.replace(
				"#pragma once\n\n#include <memory>",
				"#pragma once\n\n#include <cstdint>\n#include <memory>",
			);

			if let Err(e) = fs::write(&file_path, content) {
				eprintln!("cargo:warning=修补 trace_record.h 失败: {}", e);
			} else {
				println!("cargo:warning=已应用补丁到 trace_record.h");
			}
		}
	}
}
