## 2024-05-24 - Optimize PATH traversal by reducing stat syscalls
**Learning:** `fs::read_dir` traversal using `entry.path().is_file()` incurs an extra `stat` syscall. Using `entry.file_type()` directly reads from directory entries (where supported), offering significant performance improvements during traversal without additional syscalls.
**Action:** When traversing directories, prefer `entry.file_type()` and `entry.file_name()` over `entry.path().is_file()` and `entry.path().file_name()` to reduce syscall overhead and unnecessary `PathBuf` allocations.
