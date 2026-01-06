class Solution {
public:
    ListNode* reverseKGroup(ListNode* head, int k) {
        int count = 0;
        ListNode* curr = head;

        while (curr && count < k) {
            curr = curr->next;
            count++;
        }

        if (count < k) {
            return head;
        }

        ListNode* prev = reverseKGroup(curr, k);

        while (count > 0) {
            ListNode* next_node = head->next;
            head->next = prev;
            prev = head;
            head = next_node;
            count--;
        }

        return prev;
    }
};
