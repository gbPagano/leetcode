# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        items = []
        while head.next is not None:
            items.append(head)
            head = head.next
        
        res = head
        for item in items[::-1]:
            head.next = item
            head = item
        head.next = None
        return res

