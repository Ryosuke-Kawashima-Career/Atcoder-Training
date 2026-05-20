# Atcoder Heuristics Contest 065

## Overview Story

AtCoder Inc. has recently introduced a new warehouse to store its original merchandise sold online. The warehouse contains a large number of boxes holding items such as T-shirts, keychains, and acrylic stands, and each box is assigned a management number.

Since the boxes were brought in hastily, they were placed in locations completely unrelated to their numbers. For future inventory management, the boxes need to be taken out through the exit in increasing order of management number.

CEO Takahashi decided to install a circular conveyor belt in the warehouse and make the boxes circulate so that all boxes can be taken out with **as few operations as possible**.

## Problem Statement

There is an N×N warehouse. Let (0,0) be the coordinates of the top-left cell, and let (i,j) be the coordinates of the cell located i cells downward and j cells to the right from there. The cell (0,N/2) contains the exit of the warehouse.

There is exactly one box with each number from 0 to N^2 - 1 in the warehouse. In the initial state, the number of the box placed in cell (i,j) is a_{i,j}.

First, install at most N^2 loop-shaped devices in the warehouse. Hereafter, each loop-shaped device is called a conveyor belt.

The m-th conveyor belt is represented by a sequence of cells (i_{m,0},j_{m,0}),(i_{m,1},j_{m,1}),…,(i_{m,l_m-1},j_{m,l_m-1}), where l_m is the length of the conveyor belt.

Each conveyor belt must satisfy the following conditions.
l_m >= 2
For each x=0,…,l_m-1, 0≤i_{m,x},j_{m,x}<N.
The cells (i_{m,0},j_{m,0}),…,(i_{m,l_m-1},j_{m,l_m-1}) contained in the conveyor belt are all distinct.
For each x=0,…,l_m-2, the cells (i_{m,x},j_{m,x}) and (i_{m,x+1},j_{m,x+1}) are adjacent vertically or horizontally.
The cells (i_{m,l_m-1},j_{m,l_m-1}) and (i_{m,0},j_{m,0}) are also adjacent vertically or horizontally.
Also, over all conveyor belts, each cell must be contained in at most two conveyor belts.

where l_m is the length of the conveyor belt.

After installing the conveyor belts, you may perform at most 10^5 turns of operations. In each turn, specify a conveyor belt number m and a direction d∈{−1,1}, and all boxes and empty cells on that conveyor belt simultaneously move circularly by one cell. That is, a box or an empty cell at cell (i_{m,x},j_{m,x}) before the operation moves after the operation to cell (i_{m,(x+d)mod l_m},j_{m,(x+d)mod l_m}).

After the operation, if there is a box in the exit cell (0,N/2) and its number is the smallest among the boxes remaining in the warehouse, that box is removed through the exit.

If a_{0,N/2}=0 in the initial state, box 0 is removed before the first operation is performed.

Move out all boxes in increasing order of management number using as few operations as possible.

The score is the total number of operations performed.

N is always a multiple of 4.

After the operation, if there is a box in the exit cell (0,N/2) and its number is the smallest among the boxes remaining in the warehouse, that box is removed through the exit.

If a_{0,N/2} = 0 in the initial state, box 0 is removed before the first operation is performed.

Move out all boxes in increasing order of management number using as few operations as possible.

## Scoring

Let T be the length of the output operation sequence, and let B be the number of boxes successfully moved out. Then, you will obtain the following score.
If B = N^2, 10^6 + round(10^6 log_2(10^5 / T))
If B < N^2, round(10^6 * B/N^2)
There are 150 test cases, and the score of a submission is the total score for each test case.

## Input Format

```text
N
a_{0,0} ... a_{0, N-1}
:                :
a_{N-1,0} ... a_{N-1,N-1}
```

In all test cases, N is fixed to 20.
a_{i,j} represents the number of the box placed in cell (i,j) in the initial state.
a_{i,j} is an integer satisfying 0≤a_{i,j}≤N^2-1.
The array a contains each integer from 0 to N^2-1 exactly once.

## Output Format

First, let $M$ be the number of conveyor belts to install, and output it to Standard Output in the following format.

```text
M
l_0 (i_{0,0}, j_{0,0}) ... (i_{0,l_0-1}, j_{0,l_0-1})
...
l_{m-1}, (i_{m-1,0}, j_{m-1,0}) ... (i_{m-1,l_{m-1}-1}, j_{m-1,l_{m-1}-1})
t_1,d_1 ... t_T,d_T
```

Then, let T be the number of operations, and output the operation sequence in the following format.

```text
T
m_{0} d_{0}
...
m_{T-1} d_{T-1}
```

In the $t$-th operation, the $m_t$-th conveyor belt is circularly shifted by one cell in direction $d_t$.

The output must satisfy the following conditions.

$0 \le M \le N^2$
The length $l_m$ of each conveyor belt satisfies $l_m \ge 2$.
Each coordinate $(i_{m,x}, j_{m,x})$ satisfies $0 \le i_{m,x}, j_{m,x} < N$.
The cells contained in each conveyor belt are all distinct.
For each conveyor belt, adjacent cells in the sequence, as well as the last cell and the first cell, are adjacent vertically or horizontally.
Each cell must be contained in at most two conveyor belts.
$0 \le T \le 10^5$
For each operation, $0 \le m_t < M$.
For each operation, $d_t \in \{-1, 1\}$.

## Constraints

**Time Limit**: 2 sec

**Memory Limit**: 1024 MiB

$2 \le N \le 20$
