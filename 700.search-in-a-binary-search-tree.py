# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from collections import deque


class Solution:
    def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        queue = deque()
        queue.append(root)
        while queue:
            curr = queue.popleft()
            if curr.val == val:
                return curr
            elif curr.val > val and curr.left is not None:
                queue.append(curr.left)
            elif curr.right is not None:
                queue.append(curr.right)

        return None


