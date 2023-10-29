# Train game, written in Rust

This is my initial attempt at making a train-game solver in Rust. The train game is commonly played on Sydney trains where the objective is to use the four carriage digits (eg 1810) to make 10 (in this case, `1 + 8 + 1 + 0`).

# Usage

`cargo run input=<4 digit number>`

eg

```
~$: cargo run input=1989

Found 3 solutions for 1989!
(1 - (9 * (8 - 9))) = 10
(1 - (9 / (8 - 9))) = 10
(1 * (9 - (8 - 9))) = 10
```

If there are no solutions, the program will output as such:

```
~$: cargo run input=0018

No solutions found!
```

# Implementation

The game is solved by utilising Reverse Polish Notation and iterating through all $4^3=64$ operator permutations between `a b c d`. For instance, one such permutation is $a + (b + (c + d)))$, which is denoted as `a b c d + + +`. A stack is used to compute the value, as well as convert RPN into infix notation.
