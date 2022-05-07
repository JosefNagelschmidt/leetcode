# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        leading_one = 0
        summa = l1.val + l2.val + leading_one
        decimal = summa % 10
        leading_one = summa // 10

        res = ListNode(decimal)
        res_tail = res
        
        active = []
        
        if l1.next is not None:
            l1 = l1.next
            active.append(l1)
        if l2.next is not None:
            l2 = l2.next
            active.append(l2)
        
        while active:
            print(active)
            nums = [leading_one]
            if l1 in active:
                nums.append(l1.val)
                if l1.next is not None:
                    active.remove(l1)
                    l1 = l1.next
                    active.append(l1)
                else:
                    active.remove(l1)
            if l2 in active:
                nums.append(l2.val)
                if l2.next is not None:
                    active.remove(l2)
                    l2 = l2.next
                    active.append(l2)
                else:
                    active.remove(l2)
            summa = sum(nums)
            decimal = summa % 10
            res_tail.next = ListNode(decimal)
            res_tail = res_tail.next
            leading_one = summa // 10
            
        if leading_one != 0:
            res_tail.next = ListNode(leading_one)
            res_tail = res_tail.next
        return res
  
        
            
        