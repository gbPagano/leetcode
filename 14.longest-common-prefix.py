from functools import reduce

class Solution(object):
    def longestCommonPrefix(self, strs):
        """
        :type strs: List[str]
        :rtype: str
        """
        return reduce(self._long_common_prefix, strs)

    def _long_common_prefix(self, a, b):
        if a == b:
            return a
        
        if not len(a) or not len(b) or a[0] != b[0]:
            return ""
        
        for i in range(len(a)-1, -1, -1):
            if a[:i+1] == b[:i+1]:
                return a[:i+1]

 
