## 2024-05-14 - Optimizing Path Traversal
**Learning:** Checking executable permissions using `entry.path().is_file()` or similar triggers expensive `stat`/`lstat` system calls. This happens significantly during `$PATH` traversal (e.g. in `load_available_commands`).
**Action:** Use `entry.file_type()` which utilizes the directory entry type without extra syscalls where possible, only falling back to `entry.path().is_file()` if the type is a symlink. Also, use `entry.file_name()` instead of `entry.path().file_name()` to prevent unnecessary `PathBuf` allocations.
