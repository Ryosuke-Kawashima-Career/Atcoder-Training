# A - Non-Crossing Railcar Rearrangement

**Time Limit:** 2 sec / **Memory Limit:** 1024 MiB

## Story

An urgent request has arrived from Takahashi Railway in the AtCoder Kingdom. They want your help rearranging the railcars in the freight terminal for the simultaneous departures the next morning.

In the freight terminal, several tracks for departing trains (departure tracks) are lined up, and railcars are placed on them. In addition, the same number of tracks for temporarily storing railcars (siding tracks) are provided at the tail end of the departure tracks. By moving railcars appropriately between the departure tracks and the siding tracks, the goal is to arrange the railcars on each departure track in the prescribed order.

When performing multiple moves simultaneously, if the routes connecting departure tracks and siding tracks intersect, the railcars block each other's paths. Therefore, the moves performed at the same time must be chosen so that their routes do not intersect.

For Takahashi Railway, arrange all railcars in the prescribed order in as few steps as possible.

## Problem Statement

There are $R$ departure tracks arranged in the freight terminal, and $R$ siding tracks are placed on the side opposite to the exits of the departure tracks. The departure tracks and the siding tracks are numbered $0, 1, \dots, R-1$ from left to right. Hereafter, for a departure track, the end on the exit side is called the "front", and the opposite end is called the "rear". For a siding track, the end closer to the departure tracks is called the "front", and the opposite end is called the "rear".

Every departure track is connected by rails to every siding track, and railcars can be moved between departure tracks and siding tracks. Each departure track has a capacity of 15 railcars, and each siding track has a capacity of 20 railcars. Any move that would exceed these capacities is prohibited.

Initially, each departure track contains 10 railcars. Each railcar is assigned a unique ID from $0, \dots, 10R-1$, and the ID of the railcar placed at the $c$-th position from the front of departure track $r$ is $Y_{r,c}$. All siding tracks are empty. By moving railcars between departure tracks and siding tracks, the goal is to arrange, for each departure track $r$, the railcars with IDs $10r, 10r+1, \dots, 10r+9$ in this order from the front.

In each turn, you may perform multiple moves simultaneously from the following two types of moves. Each move handles $k$ consecutive railcars ($k \ge 1$), and their order is preserved.

- **(type = 0)**: Take $k$ consecutive railcars from the rear of departure track $i$ and attach them to the front of siding track $j$.
- **(type = 1)**: Take $k$ consecutive railcars from the front of siding track $j$ and attach them to the rear of departure track $i$.

To ensure safe train operations, the moves performed in the same turn must be chosen so that their routes do not intersect, regardless of direction. That is, the following two conditions must be satisfied:

1. A departure track or siding track must not be used more than once in the same turn.
2. If a move between departure track $i_1$ and siding track $j_1$, and a move between departure track $i_2$ and siding track $j_2$ are performed simultaneously, then if $i_1 < i_2$, it must hold that $j_1 < j_2$.

You may perform at most $4000$ turns of moves.

## Scoring

Let $T$ be the number of turns in the output sequence of moves.

- If all departure tracks exactly match the target configuration, the score is $100R + 4000 - T$.
- Otherwise, the score is the sum of the following points over each railcar placed on the departure tracks in the final state (at most $10R$ railcars):
  - **1 point** if the railcar is placed on the correct departure track.
  - **An additional 9 points** if its position from the front is also correct.

If the number of turns exceeds $4000$, or if the output violates the non-intersection constraint or the capacity constraint of any track, the submission is judged as `WA`.

There are $150$ test cases, and the score of a submission is the total score for each test case. If your submission produces an illegal output or exceeds the time limit for some test cases, the submission itself will be judged as `WA` or `TLE`, and the score of the submission will be zero.

## Input

The input is given from Standard Input in the following format:

```text
R
Y_{0,0} \dots Y_{0,9}
\vdots
Y_{R-1,0} \dots Y_{R-1,9}
```

In every test case, $R = 10$.
$Y_{r,c}$ is the ID of the railcar placed at the $c$-th position from the front of departure track $r$ in the initial state ($0 \le c < 10$).
$Y_{r,c}$ contains each of $0, \dots, 10R-1$ exactly once.

## Output

Print the sequence of moves to Standard Output in the following format:

```text
T
K_0
type_0 i_0 j_0 k_0
\vdots
type_{K_0-1} i_{K_0-1} j_{K_0-1} k_{K_0-1}
\vdots
K_{T-1}
type_0 i_0 j_0 k_0
\vdots
type_{K_{T-1}-1} i_{K_{T-1}-1} j_{K_{T-1}-1} k_{K_{T-1}-1}
```

Here, $T$ is the number of turns in which moves are performed ($T \le 4000$). After printing $T$, for each turn $t = 0, 1, \dots, T-1$, print the following in this order:

First, print the number of moves performed in that turn, $K_t$, on a single line ($1 \le K_t \le R$).
Then, print each of the moves performed in that turn on the following $K_t$ lines in the format `type i j k` ($type \in \{0,1\}$, $0 \le i,j < R$, $k \ge 1$).

- If `type = 0`, take $k$ railcars from the rear of departure track $i$ and attach them to the front of siding track $j$.
- If `type = 1`, take $k$ railcars from the front of siding track $j$ and attach them to the rear of departure track $i$.

## Input Generation

In every test case, $R = 10$.

Generate a uniformly random permutation of the integer sequence $0, 1, \dots, 10R-1$, and assign the elements in order to $Y_{0,0}, Y_{0,1}, \dots, Y_{0,9}, Y_{1,0}, \dots, Y_{R-1,9}$.
