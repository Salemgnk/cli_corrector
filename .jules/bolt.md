## 2024-05-24 - Expensive string distance calculations
**Learning:** In local matching loops, running expensive calculations like Levenshtein distance on all elements before applying O(1) filtering rules (like length difference checks) causes massive performance degradation.
**Action:** Always place O(1) early returns and simple filtering checks before expensive computations like Levenshtein string distance in loops.
