## 2024-05-18 - Rust `fs::read_dir` performance optimization in $PATH directories
**Learning:** Checking `entry.path().is_file()` for every file in directories like `/usr/bin` (from `$PATH`) is very slow (~900ms) because it calls `stat` for each entry. The directory entry itself often contains the file type.
**Action:** Use `entry.file_type()` instead. Fallback to `entry.path().is_file()` only if `file_type.is_symlink()` is true. This can reduce directory scanning time by >90% (e.g. 900ms -> 18ms).
