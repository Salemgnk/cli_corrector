## 2024-05-16 - Delaying Expensive Distance Calculations
**Learning:** In the CLI fuzzy matcher (`suggest_command_local`), calculating the Levenshtein distance before performing a simple length difference check introduces unnecessary latency. `levenshtein` has a time complexity of O(M*N), whereas checking string length difference is O(1).
**Action:** Always perform cheap filtering operations (like length or boundary checks) before invoking computationally expensive algorithms on large datasets.

## 2024-05-20 - Fast Directory Traversal
**Learning:** Using `entry.path().is_file()` during deep directory traversal (like scanning the entire $PATH) is a performance anti-pattern. It forces Rust to allocate a `PathBuf` and triggers a blocking `stat` system call for every file. Using `entry.file_type()` accesses metadata already retrieved by `read_dir` on most Unix platforms, dropping execution time drastically.
**Action:** Always use `entry.file_type()` and `entry.file_name()` instead of `entry.path().is_file()` and `entry.path().file_name()` when scanning directories to avoid unnecessary allocations and `stat` syscalls. Fallback to `entry.path().is_file()` only for symlinks.
