int compare(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

bool containsDuplicate(int* nums, int numsSize) {
    int* sorted = (int*)malloc(numsSize * sizeof(int));
    memcpy(sorted, nums, numsSize * sizeof(int));
    qsort(sorted, numsSize, sizeof(int), compare);
    for (int i = 1; i < numsSize; i++) {
        if (sorted[i] == sorted[i-1]) {
            free(sorted);
            return true;
        }
    }
    free(sorted);
    return false;
}
