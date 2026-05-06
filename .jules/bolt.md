## 2025-02-13 - Short-circuit expensive operations early
**Learning:** In `src/commands.rs`, calculating Levenshtein distance O(N*M) is a known performance bottleneck for fuzzy matching against all available `$PATH` commands. Calculating length difference is O(1).
**Action:** Always place fast O(1) checks (like string length difference) before expensive operations (like edit distances) when iterating over large datasets to aggressively short-circuit and improve execution speed.
