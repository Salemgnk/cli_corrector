## 2024-05-24 - Optimization Order Matters
**Learning:** Checking string length difference O(1) before calculating Levenshtein distance O(N*M) provides a massive performance boost when scanning large lists of available PATH commands.
**Action:** Always consider fast, cheap checks (like length comparisons) to prune impossible search space combinations before executing expensive distance or matching algorithms.
