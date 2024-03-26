class Solution:
    def largestAltitude(self, gain: List[int]) -> int:
        highest, curr = 0, 0
        for alt in gain:
            curr += alt
            highest = max(highest, curr)
        return highest
