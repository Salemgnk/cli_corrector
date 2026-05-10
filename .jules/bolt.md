## 2024-05-10 - O(1) Short-circuiting for Levenshtein Distance
**Learning:** In string similarity algorithms like Levenshtein distance, performing the expensive O(N*M) calculation on every candidate string can be a significant bottleneck. This codebase correctly identified that strings with a length difference greater than the maximum allowed edit distance can be ignored, but performed this check *after* calculating the distance.
**Action:** Always perform O(1) length difference checks *before* calling expensive edit distance functions to short-circuit the calculation.
