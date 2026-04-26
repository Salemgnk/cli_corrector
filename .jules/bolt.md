## 2024-05-24 - [Levenshtein length pruning]
**Learning:** In fuzzy matching algorithms, checking length differences prior to Levenshtein calculation prunes a massive amount of unnecessary distance calculations since distance >= absolute length difference.
**Action:** Always place O(1) length constraint checks before executing O(N*M) edit distance functions on large search domains.
