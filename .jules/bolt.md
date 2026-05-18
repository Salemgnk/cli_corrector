## 2024-05-16 - Delaying Expensive Distance Calculations
**Learning:** In the CLI fuzzy matcher (`suggest_command_local`), calculating the Levenshtein distance before performing a simple length difference check introduces unnecessary latency. `levenshtein` has a time complexity of O(M*N), whereas checking string length difference is O(1).
**Action:** Always perform cheap filtering operations (like length or boundary checks) before invoking computationally expensive algorithms on large datasets.
