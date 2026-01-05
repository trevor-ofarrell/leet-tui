# Plan: Class-Based Test Harnesses

## Current Status
- **116/131 problems passing** (89%)
- **15 problems skipped** - require custom test harnesses for class-based or special input/output handling

## Skipped Problems by Category

### Category 1: Class-Based Design Problems (9 problems)
These use a method-call sequence format: `[["ClassName", "method1", "method2"], [[], [args1], [args2]]]`

| Problem | File | Methods |
|---------|------|---------|
| 146 LRUCache | `146_lru_cache.json` | `LRUCache(capacity)`, `get(key)`, `put(key, value)` |
| 155 MinStack | `155_min_stack.json` | `MinStack()`, `push(val)`, `pop()`, `top()`, `getMin()` |
| 208 Trie | `208_implement_trie.json` | `Trie()`, `insert(word)`, `search(word)`, `startsWith(prefix)` |
| 211 WordDictionary | `211_add_search_words.json` | `WordDictionary()`, `addWord(word)`, `search(word)` |
| 295 MedianFinder | `295_find_median_data_stream.json` | `MedianFinder()`, `addNum(num)`, `findMedian()` |
| 355 Twitter | `355_design_twitter.json` | `Twitter()`, `postTweet()`, `getNewsFeed()`, `follow()`, `unfollow()` |
| 703 KthLargest | `703_kth_largest_stream.json` | `KthLargest(k, nums)`, `add(val)` |
| 981 TimeMap | `981_time_based_key_value_store.json` | `TimeMap()`, `set(key, value, timestamp)`, `get(key, timestamp)` |
| 2013 DetectSquares | `2013_detect_squares.json` | `DetectSquares()`, `add(point)`, `count(point)` |

**Harness Pattern:**
```javascript
const [methods, args] = testCase.input;
const results = [];
let instance = null;
for (let i = 0; i < methods.length; i++) {
    const method = methods[i];
    const arg = args[i];
    if (method === 'ClassName') {
        instance = new ClassName(...arg);
        results.push(null);
    } else {
        results.push(instance[method](...arg));
    }
}
return results;
```

### Category 2: Serialize/Deserialize (2 problems)

| Problem | File | Description |
|---------|------|-------------|
| 297 Codec | `297_serialize_deserialize_tree.json` | Serialize/deserialize binary tree |
| 271 Encode/Decode | `271_encode_decode_strings.json` | Encode/decode string array |

**297 Codec Pattern:** Test that `decode(encode(tree)) === tree`
```javascript
const codec = new Codec();
const tree = arrayToTree(input);
const result = treeToArray(codec.deserialize(codec.serialize(tree)));
return compareArrays(result, expected);
```

**271 Pattern:** Test that `decode(encode(strs)) === strs`
```javascript
const result = decode(encode(input));
return arraysEqual(result, expected);
```

### Category 3: Graph/Linked List with Special Structures (4 problems)

| Problem | File | Description |
|---------|------|-------------|
| 133 cloneGraph | `133_clone_graph.json` | Clone graph with adjacency list input |
| 138 copyRandomList | `138_copy_list_with_random_pointer.json` | Copy linked list with random pointers |
| 141 hasCycle | `141_linked_list_cycle.json` | Detect cycle in linked list |
| 23 mergeKLists | `023_merge_k_sorted_lists.json` | Merge k sorted linked lists |

**133 cloneGraph Pattern:**
```javascript
// Input: adjacency list [[1,2],[0,2],[0,1]] means node 0 connects to 1,2, etc.
const buildGraph = (adj) => {
    if (!adj || adj.length === 0) return null;
    const nodes = adj.map((_, i) => ({ val: i, neighbors: [] }));
    adj.forEach((neighbors, i) => {
        nodes[i].neighbors = neighbors.map(n => nodes[n]);
    });
    return nodes[0];
};
const graphToAdj = (node) => { /* BFS to rebuild adjacency list */ };
```

**138 copyRandomList Pattern:**
```javascript
// Input: [[val, random_index], ...] e.g. [[7,null],[13,0],[11,4]]
const buildList = (arr) => {
    const nodes = arr.map(([val]) => ({ val, next: null, random: null }));
    arr.forEach(([val, randomIdx], i) => {
        if (i < arr.length - 1) nodes[i].next = nodes[i + 1];
        if (randomIdx !== null) nodes[i].random = nodes[randomIdx];
    });
    return nodes[0];
};
```

**141 hasCycle Pattern:**
```javascript
// Input: [array, pos] where pos is the index the tail connects to (-1 for no cycle)
const buildCyclicList = (arr, pos) => {
    if (!arr.length) return null;
    const nodes = arr.map(v => ({ val: v, next: null }));
    for (let i = 0; i < nodes.length - 1; i++) nodes[i].next = nodes[i + 1];
    if (pos >= 0) nodes[nodes.length - 1].next = nodes[pos];
    return nodes[0];
};
```

**23 mergeKLists Pattern:**
```javascript
// Input: array of arrays, each representing a linked list
const lists = input.map(arr => arrayToList(arr));
const result = mergeKLists(lists);
return listToArray(result);
```

### Category 4: Tree with Node Reference (1 problem)

| Problem | File | Description |
|---------|------|-------------|
| 235 LCA BST | `p235_lowest_common_ancestor_of_bst.json` | Find LCA given tree + two node values |

**Pattern:**
```javascript
// Input: [treeArray, p_val, q_val]
const root = arrayToTree(treeArray);
const findNode = (node, val) => { /* BFS/DFS to find node by value */ };
const p = findNode(root, p_val);
const q = findNode(root, q_val);
const result = lowestCommonAncestor(root, p, q);
return result.val;
```

## Implementation Plan

### Step 1: Create Generic Class Harness
Add to `fix_and_validate.py` a generic handler for class-based problems:

```python
class_funcs = {
    'LRUCache': 'LRUCache',
    'MinStack': 'MinStack',
    'Trie': 'Trie',
    'WordDictionary': 'WordDictionary',
    'MedianFinder': 'MedianFinder',
    'Twitter': 'Twitter',
    'KthLargest': 'KthLargest',
    'TimeMap': 'TimeMap',
    'DetectSquares': 'DetectSquares'
}
```

Generate JavaScript test code:
```javascript
const [methods, args] = tc.input;
const results = [];
let instance = null;
for (let i = 0; i < methods.length; i++) {
    if (methods[i] === 'ClassName') {
        instance = new ClassName(...args[i]);
        results.push(null);
    } else {
        results.push(instance[methods[i]](...args[i]));
    }
}
```

### Step 2: Add Serialize/Deserialize Harness
For Codec and encode/decode, test roundtrip equality.

### Step 3: Add Graph/LinkedList Constructors
Add helper functions for:
- Building cyclic linked lists
- Building graphs from adjacency lists
- Building linked lists with random pointers

### Step 4: Add LCA Harness
Find nodes by value in tree, then call LCA function.

## Estimated Effort
- Generic class harness: 30 lines of Python
- Codec harness: 20 lines
- Graph constructors: 40 lines JavaScript helpers
- LCA harness: 15 lines
- Testing and debugging: varies

## Files to Modify
1. `scripts/fix_and_validate.py` - Add new harness logic
2. Potentially create `scripts/test_harness_helpers.js` for reusable JavaScript helpers
