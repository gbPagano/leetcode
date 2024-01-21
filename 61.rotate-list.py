# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution(object):
    def rotateRight(self, head, k):
        """
        :type head: ListNode
        :type k: int
        :rtype: ListNode
        """
        
        if not k or not head or head.next is None:
            return head
        
        curr = head
        node_size = 1
        while curr.next is not None:
            curr = curr.next
            node_size += 1
        
        k = node_size - (k % node_size)    
        
        curr.next = head
        for _ in range(k):
            curr = curr.next

        head, curr.next = curr.next, None
        return head

    
