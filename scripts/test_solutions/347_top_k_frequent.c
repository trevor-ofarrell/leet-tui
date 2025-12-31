typedef struct { int num; int count; } Pair;

int cmpPair(const void* a, const void* b) {
    return ((Pair*)b)->count - ((Pair*)a)->count;
}

int* topKFrequent(int* nums, int numsSize, int k, int* returnSize) {
    Pair* pairs = (Pair*)calloc(20001, sizeof(Pair));
    int uniqueCount = 0;
    for (int i = 0; i < numsSize; i++) {
        int idx = nums[i] + 10000;
        if (pairs[idx].count == 0) {
            pairs[idx].num = nums[i];
            uniqueCount++;
        }
        pairs[idx].count++;
    }
    Pair* sorted = (Pair*)malloc(uniqueCount * sizeof(Pair));
    int j = 0;
    for (int i = 0; i < 20001; i++) {
        if (pairs[i].count > 0) sorted[j++] = pairs[i];
    }
    qsort(sorted, uniqueCount, sizeof(Pair), cmpPair);
    *returnSize = k;
    int* result = (int*)malloc(k * sizeof(int));
    for (int i = 0; i < k; i++) result[i] = sorted[i].num;
    free(pairs); free(sorted);
    return result;
}
