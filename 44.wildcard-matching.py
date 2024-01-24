class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        i = 0
        j = 0
        i_star = None
        j_star = None
        while i < len(s):

            if j < len(p) and (s[i] == p[j] or p[j] == "?"):
                i += 1
                j += 1
            elif j < len(p) and p[j] == "*":
                j_star = j
                i_star = i
                j += 1
            elif j_star is not None:
                i = i_star + 1
                j = j_star + 1
                i_star += 1
            else:
                return False

        while j < len(p) and p[j] == "*":
            j += 1

        return j == len(p)
