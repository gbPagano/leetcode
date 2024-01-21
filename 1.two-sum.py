class Solution(object):
    def twoSum(self, nums, target):
        hashmap = dict()
        for i, n in enumerate(nums):
            diff = target - n
            j = hashmap.get(diff)
            if j is not None:
                return [i, j]
            hashmap[n] = i
        
