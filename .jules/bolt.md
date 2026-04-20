## 2024-03-24 - File System Traversal Performance
**Learning:** `std::fs::DirEntry::path().is_file()` does a redundant stat call, whereas `std::fs::DirEntry::file_type()` gets the file type from the directory entry itself, saving significant time during path traversal (like scanning `$PATH`).
**Action:** Use `entry.file_type()` when reading directories, but fallback to `entry.path().is_file()` if it's a symlink because `file_type()` doesn't follow symlinks. Also, use `entry.file_name()` instead of `entry.path().file_name()` to save a `PathBuf` allocation.
