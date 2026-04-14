## 2024-04-14 - Optimize PATH directory traversal
**Learning:** Using `entry.path().is_file()` during `fs::read_dir` traversal incurs unnecessary `stat` syscalls and `PathBuf` allocations. `entry.file_type()` uses cached metadata from reading the directory.
**Action:** Use `entry.file_type()` over `entry.path().is_file()` and `entry.file_name()` over `entry.path().file_name()` to avoid `stat` calls and `PathBuf` allocations. Always handle symlink fallbacks using `entry.path().is_file()` since `file_type()` doesn't follow symlinks.
