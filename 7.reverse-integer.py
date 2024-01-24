class Solution:
    def reverse(self, x: int) -> int:
        negative = (x < 0)
        
        x = int(str(abs(x))[::-1])

        if negative:
            x *= -1
        
        min = -2**31
        max = 2**31 - 1

        if min <= x <= max:
            return x

        return 0

