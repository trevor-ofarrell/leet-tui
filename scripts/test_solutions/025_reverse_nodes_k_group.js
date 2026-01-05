var reverseKGroup = function(head, k) {
    let count = 0, curr = head;
    while (curr && count < k) { curr = curr.next; count++; }
    if (count < k) return head;
    let prev = reverseKGroup(curr, k);
    while (count-- > 0) {
        let next = head.next;
        head.next = prev;
        prev = head;
        head = next;
    }
    return prev;
};
