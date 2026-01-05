var copyRandomList = function(head) {
    if (!head) return null;
    const map = new Map();
    let curr = head;
    while (curr) { map.set(curr, { val: curr.val, next: null, random: null }); curr = curr.next; }
    curr = head;
    while (curr) {
        map.get(curr).next = map.get(curr.next) || null;
        map.get(curr).random = map.get(curr.random) || null;
        curr = curr.next;
    }
    return map.get(head);
};
