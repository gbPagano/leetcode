class Solution:
    def myAtoi(self, s: str) -> int:
        s = s.strip()
        if not s:
            return 0
        if s[0] not in "1234567890-+":
            return 0
        new_s = s[0] if s[0] != "-" else "-"
        for c in s[1:]:
            if c not in "1234567890":
                break
            new_s += c
        if len(new_s) == 1 and new_s not in "1234567890":
            return 0
        return max(min(int(new_s), 2**31 - 1), -2**31) 
                

