## 2024-05-18 - Early pruning with length diff checks
**Learning:** In string similarity search (e.g. `suggest_command_local`), performing expensive O(M*N) Levenshtein distance calculations BEFORE checking for length difference thresholds causes significant performance regression on large command sets. Checking length first prunes invalid options without triggering the expensive distance calculation.
**Action:** Always place O(1) checks (like length difference) before O(M*N) string algorithms (like Levenshtein distance) in search loops.
