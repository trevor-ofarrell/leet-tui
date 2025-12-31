int missingNumber(int* nums, int numsSize) {
    int xorVal = numsSize;
    for (int i = 0; i < numsSize; i++) {
        xorVal ^= i ^ nums[i];
    }
    return xorVal;
}
