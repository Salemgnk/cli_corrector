## 2024-05-16 - Delaying Expensive Distance Calculations
**Learning:** In the CLI fuzzy matcher (`suggest_command_local`), calculating the Levenshtein distance before performing a simple length difference check introduces unnecessary latency. `levenshtein` has a time complexity of O(M*N), whereas checking string length difference is O(1).
**Action:** Always perform cheap filtering operations (like length or boundary checks) before invoking computationally expensive algorithms on large datasets.

## 2024-05-16 - Expensive Directory Traversal
**Learning:** During directory traversal (e.g., in `$PATH` scanning), using `entry.path().is_file()` introduces significant latency because it triggers blocking `stat`/`lstat` system calls and requires `PathBuf` allocations for each entry.
**Action:** When scanning directories, utilize the cached `entry.file_type()` and `entry.file_name()` instead. Only fallback to `entry.path().is_file()` if `entry.file_type().is_symlink()` is true.
