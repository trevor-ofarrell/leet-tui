var mergeKLists = function(lists) {
    if (!lists.length) return null;
    function merge(l1, l2) {
        let dummy = { val: 0, next: null };
        let curr = dummy;
        while (l1 && l2) {
            if (l1.val <= l2.val) { curr.next = l1; l1 = l1.next; }
            else { curr.next = l2; l2 = l2.next; }
            curr = curr.next;
        }
        curr.next = l1 || l2;
        return dummy.next;
    }
    while (lists.length > 1) {
        let merged = [];
        for (let i = 0; i < lists.length; i += 2) {
            let l1 = lists[i];
            let l2 = i + 1 < lists.length ? lists[i + 1] : null;
            merged.push(merge(l1, l2));
        }
        lists = merged;
    }
    return lists[0];
};
