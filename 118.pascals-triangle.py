class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        res = []
        for i in range(numRows):
            if i == 0:
                res.append([1])
                continue
            row = [1]
            for j in range(1, i):
                k = res[i-1][j] + res[i-1][j-1]
                row.append(k)
            row.append(1)
            res.append(row)

        return res
