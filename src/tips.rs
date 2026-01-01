use rand::Rng;

pub const TIPS: &[&str] = &[
    // Blind 75 & Algorithm Tips
    "Two Sum: Use a Hash Map to store {value: index}. Check if target - current exists in the map.",
    "Floyd's Cycle Finding: The \"Tortoise and Hare\" algorithm detects cycles in a Linked List in O(N) time and O(1) space.",
    "Binary Search: Always calculate mid as low + (high - low) / 2 to avoid integer overflow.",
    "Sliding Window: Expand right to satisfy the condition, then shrink left to optimize the result.",
    "Valid Parentheses: Use a Stack. Push opening brackets, pop and match when you see a closing bracket.",
    "Kadane's Algorithm: For Maximum Subarray, keep a running sum. If the sum drops below zero, reset it to zero.",
    "Reverse Linked List: You need three pointers: prev, curr, and next. Don't lose the reference to next!",
    "Merge Intervals: Sort intervals by start time first. If current.start < previous.end, they overlap.",
    "Invert Binary Tree: Swap the left and right child pointers, then recurse down both sides.",
    "Bit Manipulation: n & (n - 1) drops the lowest set bit of n. Useful for counting bits.",
    "XOR Trick: x ^ x = 0 and x ^ 0 = x. Use this to find the single unique number in an array of duplicates.",
    "Heap: Finding the Kth largest element takes O(N log K) using a Min-Heap of size K.",
    "Trie (Prefix Tree): The go-to data structure for autocomplete or word search problems.",
    "Fast & Slow Pointers: Use them to find the middle of a Linked List in a single pass.",
    "Container With Most Water: Start pointers at both ends. Always move the pointer with the shorter height.",
    "Group Anagrams: Sort strings to use as Hash Map keys, or use a character count array.",
    "Rotate Image: Transpose the matrix (swap [i][j] with [j][i]), then reverse each row.",
    "Spiral Matrix: Define boundaries (top, bottom, left, right) and shrink them as you loop.",
    "Longest Consecutive Sequence: Put all numbers in a Set. Only start counting a sequence if num - 1 is missing.",
    "Clone Graph: Use a Hash Map old_node -> new_node to track visited nodes and avoid infinite loops.",
    "Number of Islands: Iterate the grid. If you see a '1', increment count and run DFS/BFS to sink the island to '0'.",
    "Pacific Atlantic Water Flow: Run DFS from the ocean borders inward. The answer is the intersection of reachable cells.",
    "Topological Sort: Essential for dependency problems (like Course Schedule). Use Kahn's Algorithm (indegrees).",
    "Union-Find: The best structure for tracking Connected Components in a graph.",
    "Insert Interval: Handle non-overlapping before, merge overlapping, then handle non-overlapping after.",
    "House Robber: rob = max(prev_rob, prev_prev_rob + current).",
    "Coin Change: A classic Bottom-Up DP problem. dp[amount] = min(dp[amount], dp[amount - coin] + 1).",
    "Longest Common Subsequence: Use a 2D DP grid. If chars match: 1 + diagonal. Else: max(up, left).",
    "Word Break: dp[i] is true if dp[j] is true AND s[j:i] exists in the dictionary.",
    "Backtracking: Always remember to \"undo\" your change (pop from list, reset variable) after the recursive call returns.",
    "Combination Sum: In backtracking, you can reuse the same element index for the next recursive call.",
    "Subsets: For every element, you have two choices: include it in the current subset or exclude it.",
    "Serialize Tree: Use Preorder traversal (Root, Left, Right) and a special character (like '#') for nulls.",
    "Median from Data Stream: Maintain two heaps: a Max-Heap for the lower half and a Min-Heap for the upper half.",
    "Trapping Rain Water: Precompute max_left and max_right arrays, or use Two Pointers to save space.",
    "Best Time to Buy/Sell Stock: Track the min_price seen so far and calculate current_price - min_price at every step.",
    "Alien Dictionary: This is actually a graph problem. Build an adjacency list and run Topological Sort.",
    "Graph Valid Tree: A graph is a tree if it has no cycles and is fully connected.",
    "Meeting Rooms II: Sort start times and end times separately, or use a Min-Heap to track ending meetings.",
    "Decode Ways: Watch out for leading zeros! '06' cannot be decoded.",
    "Search in Rotated Sorted Array: Determine which half is sorted (left or right), then check if target lies within that range.",
    "Min Stack: Push pairs (value, current_min) onto the stack to retrieve the minimum in O(1).",
    "Kth Smallest in BST: An Inorder traversal of a BST yields sorted values. Stop at the Kth element.",
    "Lowest Common Ancestor (BST): If both nodes are smaller than root, go left. If larger, go right. Otherwise, split = LCA.",
    "Implement Trie: You need a TrieNode class with a children map and a isEndOfWord boolean.",
    "Design Add and Search Words: Use DFS. If the char is '.', check all children of the current node.",
    "Word Search II: Combine Backtracking with a Trie for efficiency. Prune the Trie as you find words.",
    "Longest Palindromic Substring: Expand around the center. Remember to handle both odd (aba) and even (abba) centers.",
    "Reorder List: Find the middle, reverse the second half, and merge the two halves alternately.",
    "Maximum Product Subarray: Track both max_prod and min_prod because a negative number flips the sign.",

    // Neovim & Vim Motions
    ". (Dot): Repeats the last change. It is the most powerful command in Vim.",
    "ciw: \"Change Inner Word\". Deletes the word under the cursor and puts you in Insert mode.",
    "%: Jumps between matching parentheses (), brackets [], or braces {}.",
    "dt\": \"Delete Till Quote\". Deletes everything from the cursor up to the next \".",
    "*: Searches forward for the word currently under the cursor.",
    "fx: \"Find x\". Jumps to the next occurrence of character x on the current line.",
    ";: Repeats the last f, t, F, or T movement.",
    "zz: Centers the current line on the screen.",
    "Ctrl + o: Jumps back to the previous cursor position (Jump List).",
    "Ctrl + i: Jumps forward in the Jump List.",
    ">>: Indents the current line.",
    "=: Auto-indents the selected text or current line.",
    "vip: \"Visually select Inner Paragraph\". Great for selecting a whole block of code.",
    "yyp: Duplicates the current line (Yank, then Paste).",
    "dd: Deletes (cuts) the current line.",
    "~: Toggles the case of the character under the cursor.",
    "I: Enters Insert mode at the very beginning of the line.",
    "A: Enters Insert mode at the very end of the line.",
    "o: Opens a new line below the cursor and enters Insert mode.",
    "O: Opens a new line above the cursor and enters Insert mode.",
    "cw: \"Change Word\". Changes from the cursor to the end of the word.",
    "D: Deletes from the cursor to the end of the line.",
    "C: Changes from the cursor to the end of the line.",
    "gg: Jumps to the top of the file.",
    "G: Jumps to the bottom of the file.",
    ":noh: Clears the current search highlighting.",
    "qa: Starts recording a macro into register a. Press q to stop. @a to replay.",
    "xp: Transposes two characters (deletes current, pastes after).",
    "J: Joins the current line with the next line.",
    "guu: Lowercases the entire line.",
    "gUU: Uppercases the entire line.",
    "gf: \"Go File\". Opens the file path under the cursor.",
    "Ctrl + v: Enters Visual Block mode. Perfect for editing multiple lines vertically.",
    "gv: Reselects the last visual selection.",
    ":s/old/new/g: Replaces all occurrences of 'old' with 'new' in the current line.",
    ":%s/old/new/g: Replaces all occurrences of 'old' with 'new' in the entire file.",
    "ea: Appends text at the end of the current word.",
    "vib: Visually selects inside brackets ().",
    "viB: Visually selects inside braces {}.",
    "cit: \"Change Inner Tag\". Changes the content inside an HTML/XML tag.",
    "ma: Sets mark a at the current position. 'a jumps back to it.",
    "H, M, L: Moves cursor to High, Middle, or Low part of the screen.",
    "zt: Scrolls the screen so the current line is at the Top.",
    "zb: Scrolls the screen so the current line is at the Bottom.",
    "daw: \"Delete A Word\". Deletes the word and the trailing space.",
    "diw: \"Delete Inner Word\". Deletes the word but keeps the trailing space.",
    "Ctrl + w, v: Splits the window vertically.",
    "Ctrl + w, s: Splits the window horizontally.",
    "Ctrl + w, =: Equalizes the size of all split windows.",
    ":w !sudo tee %: The trick to save a file when you forgot to open it with sudo.",

    // Foundational CS & Data Structures
    "Big O Notation: O(1) is constant, O(log N) is logarithmic, O(N) is linear, O(N^2) is quadratic.",
    "Logarithms: In CS, log usually means log base 2. O(log N) implies the problem space is halved at every step.",
    "Space Complexity: Don't forget that recursive calls consume stack memory. A tree of depth N uses O(N) stack space.",
    "Arrays: Access is O(1), but inserting or deleting from the middle is O(N) because elements must shift.",
    "Linked Lists: Inserting is O(1) if you have the pointer, but finding that pointer takes O(N).",
    "Hash Map Collisions: When two keys hash to the same index, they are usually stored in a linked list (chaining).",
    "Stack (LIFO): Last In, First Out. Think of a stack of plates. Used for DFS and backtracking.",
    "Queue (FIFO): First In, First Out. Think of a line at a store. Used for BFS.",
    "Priority Queue: Inserting and removing the top element takes O(log N).",
    "Binary Search Tree (BST): Left child < Root < Right child. Inorder traversal gives sorted output.",
    "Balanced Trees: AVL or Red-Black trees guarantee O(log N) operations by preventing the tree from becoming a line.",
    "Graph Representation: Adjacency Matrix is O(V^2) space. Adjacency List is O(V+E).",
    "BFS vs DFS: BFS finds the shortest path in unweighted graphs. DFS is better for exploring all paths or detecting cycles.",
    "Recursion Base Case: Always define the exit condition first to avoid a Stack Overflow.",
    "Memoization: Storing the result of expensive function calls and returning the cached result when the same inputs occur again.",
    "Tabulation: The \"bottom-up\" approach to Dynamic Programming. Iteratively filling a table starting from the smallest subproblem.",
    "Greedy Algorithms: Making the locally optimal choice at each stage. It doesn't always work for every problem.",
    "Sorting Complexity: The best comparison-based sorting algorithms (Merge, Quick, Heap) run in O(N log N).",
    "Stable Sort: A sort that preserves the relative order of elements with equal keys.",
    "QuickSort: Fast in practice, but worst-case O(N^2) if the pivot is chosen poorly.",
    "MergeSort: Always O(N log N), but requires O(N) extra space for the merging process.",
    "Bitwise AND (&): Returns 1 only if both bits are 1. Useful for masking.",
    "Bitwise OR (|): Returns 1 if at least one bit is 1. Useful for setting flags.",
    "Bitwise XOR (^): Returns 1 if bits are different. 1 ^ 1 = 0.",
    "Left Shift (<<): x << 1 is equivalent to multiplying x by 2.",
    "Right Shift (>>): x >> 1 is equivalent to dividing x by 2 (integer division).",
    "ASCII Values: 'a' is 97, 'A' is 65, '0' is 48. char - 'a' maps 0-25.",
    "Modulo Arithmetic: (a + b) % m is safe. (a - b) % m can be negative; use ((a - b) % m + m) % m.",
    "Integer Overflow: In typed languages (C++/Java), exceeding 2^31 - 1 wraps around to negative numbers.",
    "Floating Point: Never compare floats directly with ==. Use a small epsilon: abs(a - b) < 1e-9.",
    "String Immutability: In Python and Java, strings cannot be changed. Concatenation creates a new string (O(N)).",
    "StringBuilder: Use this (or a list of chars) to build strings efficiently in loops to avoid O(N^2) behavior.",
    "Two Pointers: A technique to search pairs in a sorted array or reverse an array in place.",
    "Prefix Sum: An array where P[i] is the sum of A[0]...A[i]. Allows range sum queries in O(1).",
    "Factorials: 10! is 3.6 million. 13! overflows a standard 32-bit integer.",
    "Powers of 2: 2^10 is approx 1000 (1024). 2^20 is approx 1 million.",
    "Primes: A number is prime if it is only divisible by 1 and itself. Check divisors up to sqrt(N).",
    "GCD: The Euclidean algorithm computes the Greatest Common Divisor: gcd(a, b) = gcd(b, a % b).",
    "LCM: Least Common Multiple. lcm(a, b) = (a * b) / gcd(a, b).",
    "Palindrome: A string that reads the same forward and backward.",
    "Anagram: Two strings are anagrams if they contain the same characters with the same frequencies.",
    "Subarray vs Subsequence: A subarray is contiguous. A subsequence maintains order but can skip elements.",
    "Permutations: There are N! ways to arrange N distinct items.",
    "Subsets: A set of N elements has 2^N possible subsets.",
    "Off-by-One Errors: The most common bug in loops. Double-check your < vs <= and length vs length - 1.",
    "Sentinel Values: Using dummy nodes (like a dummy head in Linked Lists) simplifies edge cases.",
    "In-Place Operations: Algorithms that use O(1) extra space (modifying the input directly).",
    "Pass by Value vs Reference: Know if your function receives a copy of the data or a pointer to the original.",
    "Garbage Collection: In languages like Python/Java, memory is freed automatically. In C++, you must delete.",
    "Rubber Duck Debugging: Explaining your code line-by-line to an inanimate object often reveals the bug.",

    // Foundational Neovim & Vim Core
    "h j k l: The holy quartet. Left, Down, Up, Right. Keep your fingers on the home row.",
    "i: Insert mode before the cursor.",
    "a: Insert mode (Append) after the cursor.",
    "Esc: The panic button. Returns you to Normal mode.",
    ":w: Write (save) the file.",
    ":q: Quit.",
    ":q!: Force quit (discard changes).",
    ":wq: Write and quit.",
    "u: Undo the last change.",
    "Ctrl + r: Redo the change you just undid.",
    "w: Jump forward to the start of the next word.",
    "b: Jump backward to the start of the previous word.",
    "e: Jump forward to the end of the current word.",
    "0: Jump to the absolute beginning of the line.",
    "$: Jump to the absolute end of the line.",
    "^: Jump to the first non-whitespace character of the line (often more useful than 0).",
    "x: Delete the character under the cursor.",
    "r: Replace the single character under the cursor (does not enter Insert mode).",
    "s: Substitute the character under the cursor (deletes it and enters Insert mode).",
    "S: Clears the entire line and enters Insert mode (same as cc).",
    "v: Enter Visual mode (character-wise selection).",
    "V: Enter Visual Line mode (select whole lines).",
    "y: Yank (copy) selected text.",
    "p: Paste after the cursor.",
    "P: Paste before the cursor.",
    "yy: Yank the current line.",
    "/pattern: Search forward for \"pattern\". Press n for next, N for previous.",
    "?pattern: Search backward for \"pattern\".",
    ":: Enter Command mode.",
    "Ctrl + d: Scroll down half a page.",
    "Ctrl + u: Scroll up half a page.",
    "Ctrl + f: Scroll forward a full page.",
    "Ctrl + b: Scroll backward a full page.",
    ":123: Jump directly to line 123.",
    "#: Search for the word under the cursor (backward).",
    ">: Indent selection (in Visual mode).",
    "<: Un-indent selection (in Visual mode).",
    ":bn: Buffer Next. Go to the next open file buffer.",
    ":bp: Buffer Previous. Go to the previous open file buffer.",
    ":bd: Buffer Delete. Close the current file buffer.",
    "\": Access registers. \"ayw yanks a word into register 'a'. \"ap pastes from 'a'.",
    "\"+y: Yank to the system clipboard (if configured).",
    ":h <command>: The most important command. Opens the help manual for any key or command.",
];

pub struct TipSystem {
    current_tip_index: usize,
    last_change: std::time::Instant,
    fade_state: FadeState,
}

#[derive(Clone, Copy)]
pub enum FadeState {
    FadingIn(f32),   // 0.0 to 1.0
    Visible,
    FadingOut(f32),  // 1.0 to 0.0
}

impl TipSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            current_tip_index: rng.gen_range(0..TIPS.len()),
            last_change: std::time::Instant::now(),
            fade_state: FadeState::FadingIn(0.0),
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.last_change.elapsed().as_millis() as f32;

        // Timing: 2s fade in, 14s visible, 2s fade out = 18s total
        const FADE_DURATION: f32 = 2000.0;  // 2 seconds
        const VISIBLE_DURATION: f32 = 14000.0;  // 14 seconds
        const TOTAL_CYCLE: f32 = 18000.0;  // 18 seconds

        if elapsed < FADE_DURATION {
            // Fading in - use eased curve for smoother transition
            let t = elapsed / FADE_DURATION;
            let eased = t * t * (3.0 - 2.0 * t);  // smoothstep
            self.fade_state = FadeState::FadingIn(eased);
        } else if elapsed < FADE_DURATION + VISIBLE_DURATION {
            // Fully visible
            self.fade_state = FadeState::Visible;
        } else if elapsed < TOTAL_CYCLE {
            // Fading out - use eased curve for smoother transition
            let t = (elapsed - FADE_DURATION - VISIBLE_DURATION) / FADE_DURATION;
            let eased = t * t * (3.0 - 2.0 * t);  // smoothstep
            self.fade_state = FadeState::FadingOut(1.0 - eased);
        } else {
            // Cycle complete, pick new tip
            let mut rng = rand::thread_rng();
            let mut new_index = rng.gen_range(0..TIPS.len());
            // Avoid showing the same tip twice in a row
            while new_index == self.current_tip_index && TIPS.len() > 1 {
                new_index = rng.gen_range(0..TIPS.len());
            }
            self.current_tip_index = new_index;
            self.last_change = std::time::Instant::now();
            self.fade_state = FadeState::FadingIn(0.0);
        }
    }

    pub fn current_tip(&self) -> &'static str {
        TIPS[self.current_tip_index]
    }

    pub fn opacity(&self) -> f32 {
        match self.fade_state {
            FadeState::FadingIn(progress) => progress,
            FadeState::Visible => 1.0,
            FadeState::FadingOut(remaining) => remaining,
        }
    }
}

impl Default for TipSystem {
    fn default() -> Self {
        Self::new()
    }
}
