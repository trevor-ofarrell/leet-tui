class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def reverseKGroup(head, k):
    count = 0
    curr = head
    while curr and count < k:
        curr = curr.next
        count += 1
    if count < k:
        return head
    prev = reverseKGroup(curr, k)
    while count > 0:
        next_node = head.next
        head.next = prev
        prev = head
        head = next_node
        count -= 1
    return prev
