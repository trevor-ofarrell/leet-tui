var isValidSudoku = function(board) {
    const rows = Array(9).fill(null).map(() => new Set());
    const cols = Array(9).fill(null).map(() => new Set());
    const boxes = Array(9).fill(null).map(() => new Set());
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const val = board[r][c];
            if (val === '.') continue;
            const boxIdx = Math.floor(r / 3) * 3 + Math.floor(c / 3);
            if (rows[r].has(val) || cols[c].has(val) || boxes[boxIdx].has(val)) return false;
            rows[r].add(val);
            cols[c].add(val);
            boxes[boxIdx].add(val);
        }
    }
    return true;
};
