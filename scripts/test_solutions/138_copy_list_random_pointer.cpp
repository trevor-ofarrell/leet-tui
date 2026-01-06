class Solution {
public:
    RandomNode* copyRandomList(RandomNode* head) {
        if (!head) return nullptr;

        unordered_map<RandomNode*, RandomNode*> oldToNew;

        RandomNode* curr = head;
        while (curr) {
            oldToNew[curr] = new RandomNode(curr->val);
            curr = curr->next;
        }

        curr = head;
        while (curr) {
            oldToNew[curr]->next = oldToNew[curr->next];
            oldToNew[curr]->random = oldToNew[curr->random];
            curr = curr->next;
        }

        return oldToNew[head];
    }
};
