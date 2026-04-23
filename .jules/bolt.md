## 2024-06-25 - Avoid Expensive O(N*M) Operations with Early O(1) Pruning
**Learning:** In string similarity algorithms like Levenshtein distance (which is O(N*M)), evaluating inexpensive conditions (like a simple string length difference constraint) before calling the algorithm can significantly speed up search over large datasets, such as all commands available in the `PATH`.
**Action:** When filtering or scoring items, always position O(1) checks (e.g. string length bounds) before more computationally expensive operations.
