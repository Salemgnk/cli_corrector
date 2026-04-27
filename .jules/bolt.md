## 2024-04-27 - Fast Directory Traversal & String Matching
**Learning:** Checking string length difference O(1) before a Levenshtein distance calculation O(N*M) is a crucial pattern for fuzzy matching. Similarly, `DirEntry::file_type()` caches `stat` results avoiding an extra OS system call compared to `PathBuf::is_file()`, and `DirEntry::file_name()` avoids `PathBuf` allocations.
**Action:** Always verify if expensive similarity checks can be pruned with cheap boundary conditions, and use `std::fs::DirEntry` directly for metadata checks instead of converting to `std::path::PathBuf`.
