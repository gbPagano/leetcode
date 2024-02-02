class Solution:
    def isPalindrome(self, s: str) -> bool:
        a, b = 0, len(s) - 1
        while a < b:
            if not s[a].isalnum():
                a += 1
                continue
            if not s[b].isalnum():
                b -= 1
                continue
            if not s[a].lower() == s[b].lower():
                return False
            a += 1
            b -= 1
        return True

    def isPalindrome_b(self, s: str) -> bool:
        s = "".join(filter(lambda c: c.isalnum(), s)).lower()
        return s == s[::-1]
