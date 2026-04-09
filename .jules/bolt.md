## 2025-01-20 - [Optimize Directory Traversal]
**Learning:** Using `entry.path().is_file()` during `fs::read_dir` traversal incurs unnecessary overhead from `PathBuf` allocations and a `stat` syscall. Using `entry.file_type()` directly avoids both.
**Action:** Prefer `entry.file_type()` when iterating over `fs::ReadDir` items for better performance.
