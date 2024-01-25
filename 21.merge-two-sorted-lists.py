# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if list1 is None:
            return list2
        if list2 is None:
            return list1
        
        if list1.val <= list2.val:
            head = list1
            list1 = list1.next
        else:
            head = list2
            list2 = list2.next
        
        head.next = None
        curr = head
        while list1 and list2:
            if (list2 is None) or (list1 is not None and list1.val <= list2.val):
                curr.next = list1
                list1 = list1.next
            else:
                curr.next = list2
                list2 = list2.next
            
            curr = curr.next
            curr.next = None

        while list1:
            curr.next = list1
            list1 = list1.next

            curr = curr.next
            curr.next = None

        while list2:
            curr.next = list2
            list2 = list2.next

            curr = curr.next
            curr.next = None
        
        return head

