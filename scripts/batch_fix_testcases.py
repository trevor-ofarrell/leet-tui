#!/usr/bin/env python3
"""Batch fix incorrect test case expected values."""

import json
from pathlib import Path

TESTCASES_DIR = Path(__file__).parent.parent / "testcases"

def fix_testcase(filename, condition_fn, fix_fn):
    """Fix test cases in a file based on condition."""
    filepath = TESTCASES_DIR / filename
    if not filepath.exists():
        print(f"  {filename}: not found")
        return 0

    with open(filepath) as f:
        data = json.load(f)

    fixed = 0
    for section in ['run_tests', 'submit_tests']:
        for tc in data.get(section, []):
            if condition_fn(tc):
                old_expected = tc['expected']
                new_expected = fix_fn(tc)
                if old_expected != new_expected:
                    tc['expected'] = new_expected
                    fixed += 1

    if fixed > 0:
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"  {filename}: fixed {fixed} cases")
    return fixed

# Helper functions for computing correct answers
def two_sum(nums, target):
    seen = {}
    for i, n in enumerate(nums):
        if target - n in seen:
            return [seen[target - n], i]
        seen[n] = i
    return []

def two_sum_sorted(nums, target):
    left, right = 0, len(nums) - 1
    while left < right:
        s = nums[left] + nums[right]
        if s == target:
            return [left + 1, right + 1]  # 1-indexed
        elif s < target:
            left += 1
        else:
            right -= 1
    return []

def max_area(height):
    left, right = 0, len(height) - 1
    max_water = 0
    while left < right:
        h = min(height[left], height[right])
        max_water = max(max_water, h * (right - left))
        if height[left] < height[right]:
            left += 1
        else:
            right -= 1
    return max_water

def length_of_longest_substring(s):
    char_index = {}
    max_len = 0
    start = 0
    for i, c in enumerate(s):
        if c in char_index and char_index[c] >= start:
            start = char_index[c] + 1
        char_index[c] = i
        max_len = max(max_len, i - start + 1)
    return max_len

def trap(height):
    if not height:
        return 0
    n = len(height)
    left_max = [0] * n
    right_max = [0] * n
    left_max[0] = height[0]
    for i in range(1, n):
        left_max[i] = max(left_max[i-1], height[i])
    right_max[n-1] = height[n-1]
    for i in range(n-2, -1, -1):
        right_max[i] = max(right_max[i+1], height[i])
    return sum(min(left_max[i], right_max[i]) - height[i] for i in range(n))

def largest_rectangle_area(heights):
    stack = []
    max_area = 0
    heights = heights + [0]  # Add sentinel
    for i, h in enumerate(heights):
        while stack and heights[stack[-1]] > h:
            height = heights[stack.pop()]
            width = i if not stack else i - stack[-1] - 1
            max_area = max(max_area, height * width)
        stack.append(i)
    return max_area

def is_happy(n):
    seen = set()
    while n != 1 and n not in seen:
        seen.add(n)
        n = sum(int(d)**2 for d in str(n))
    return n == 1

def num_islands(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    count = 0

    def dfs(r, c):
        if r < 0 or r >= rows or c < 0 or c >= cols or grid[r][c] != '1':
            return
        grid[r][c] = '0'
        dfs(r+1, c)
        dfs(r-1, c)
        dfs(r, c+1)
        dfs(r, c-1)

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == '1':
                count += 1
                dfs(r, c)
    return count

def hamming_weight(n):
    return bin(n).count('1')

def reverse_bits(n):
    result = 0
    for _ in range(32):
        result = (result << 1) | (n & 1)
        n >>= 1
    return result

print("Fixing test case data...")

# Fix 001 twoSum
fix_testcase('001_two_sum.json',
    lambda tc: isinstance(tc.get('input'), list) and len(tc['input']) == 2,
    lambda tc: two_sum(tc['input'][0], tc['input'][1]))

# Fix 003 lengthOfLongestSubstring
fix_testcase('003_longest_substring_without_repeating_characters.json',
    lambda tc: True,
    lambda tc: length_of_longest_substring(tc['input'][0] if isinstance(tc['input'], list) else tc['input'].get('s', tc['input'])))

# Fix 011 maxArea
fix_testcase('011_container_with_most_water.json',
    lambda tc: True,
    lambda tc: max_area(tc['input'].get('height') if isinstance(tc['input'], dict) else tc['input'][0]))

# Fix 042 trap
fix_testcase('042_trapping_rain_water.json',
    lambda tc: True,
    lambda tc: trap(tc['input'][0] if isinstance(tc['input'], list) else tc['input'].get('height', [])))

# Fix 084 largestRectangleArea
fix_testcase('084_largest_rectangle_in_histogram.json',
    lambda tc: True,
    lambda tc: largest_rectangle_area(tc['input'][0] if isinstance(tc['input'], list) else tc['input'].get('heights', [])))

# Fix 167 twoSum II (sorted)
fix_testcase('167_two_sum_ii_input_array_is_sorted.json',
    lambda tc: isinstance(tc.get('input'), list) and len(tc['input']) == 2,
    lambda tc: two_sum_sorted(tc['input'][0], tc['input'][1]))

# Fix 191 hammingWeight
fix_testcase('191_number_of_1_bits.json',
    lambda tc: True,
    lambda tc: hamming_weight(tc['input'][0] if isinstance(tc['input'], list) else tc['input']))

# Fix 190 reverseBits
fix_testcase('190_reverse_bits.json',
    lambda tc: True,
    lambda tc: reverse_bits(tc['input'][0] if isinstance(tc['input'], list) else tc['input']))

# Fix 200 numIslands - recompute for grids
def fix_200():
    filepath = TESTCASES_DIR / '200_number_of_islands.json'
    if not filepath.exists():
        return
    with open(filepath) as f:
        data = json.load(f)

    fixed = 0
    for section in ['run_tests', 'submit_tests']:
        for tc in data.get(section, []):
            grid = tc.get('input', {}).get('grid', [])
            if grid:
                # Deep copy for computation
                grid_copy = [row[:] for row in grid]
                correct = num_islands(grid_copy)
                if tc['expected'] != correct:
                    tc['expected'] = correct
                    fixed += 1

    if fixed > 0:
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"  200_number_of_islands.json: fixed {fixed} cases")

fix_200()

# Fix 202 isHappy
fix_testcase('202_happy_number.json',
    lambda tc: True,
    lambda tc: is_happy(tc['input'][0] if isinstance(tc['input'], list) else tc['input'].get('n', tc['input'])))

print("\nDone!")
