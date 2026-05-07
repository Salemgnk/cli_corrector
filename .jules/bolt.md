## 2024-05-07 - Rust DirEntry Traversal Optimization
**Learning:** `DirEntry::path().is_file()` in Rust allocates a `PathBuf` and executes an additional `stat` system call, adding significant overhead when scanning large directories like those in `$PATH`.
**Action:** Use `DirEntry::file_type()` directly. It retrieves file metadata cached from the `read_dir` operation on supported platforms, bypassing the extra system call. Similarly, use `entry.file_name()` over `entry.path().file_name()` to prevent unnecessary heap allocations.
