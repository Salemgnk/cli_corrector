## 2024-05-24 - File traversal overhead in PATH resolution
**Learning:** Checking `is_file()` on `PathBuf` directly inside directory traversal does a separate `stat` system call. The `DirEntry::file_type()` method returns cached metadata on many filesystems, significantly speeding up directory iteration when building the available commands list.
**Action:** Use `entry.file_type()` to check if an entry is a file, falling back to `entry.path().is_file()` only for symlinks, to avoid unnecessary stat calls.

## 2024-05-24 - Levenshtein calculation overhead
**Learning:** Calculating `levenshtein` distance before doing an O(1) length check wastes CPU cycles for commands that are obviously too long or short.
**Action:** Always place `length difference` checks before expensive distance calculations.
