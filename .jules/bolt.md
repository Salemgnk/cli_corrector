## 2024-05-16 - Delaying Expensive Distance Calculations
**Learning:** In the CLI fuzzy matcher (`suggest_command_local`), calculating the Levenshtein distance before performing a simple length difference check introduces unnecessary latency. `levenshtein` has a time complexity of O(M*N), whereas checking string length difference is O(1).
**Action:** Always perform cheap filtering operations (like length or boundary checks) before invoking computationally expensive algorithms on large datasets.

## 2024-05-17 - Delaying Stat Calls During Path Traversal
**Learning:** During $PATH scanning in `load_available_commands`, checking `entry.path().is_file()` introduces significant latency by triggering expensive `stat` system calls for every file. Using `entry.file_type()` allows checking the file type via directory entry metadata, completely bypassing `stat` calls and increasing performance by ~6x.
**Action:** When scanning directories, always use `entry.file_type()` and `entry.file_name()` over methods on `entry.path()` to prevent hidden blocking system calls. Fall back to `entry.path().is_file()` only for resolving symlinks.
