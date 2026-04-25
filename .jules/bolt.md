## 2025-02-28 - [String comparison bottleneck identified]
**Learning:** We found that the `levenshtein` distance calculation was being computed before verifying string lengths, which is slow for non-matching inputs.
**Action:** Always verify string sizes (length diff constraint: O(1)) to skip the `levenshtein` constraint evaluation which is O(N*M).
