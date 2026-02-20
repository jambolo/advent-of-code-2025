# advent-of-code-2025

My solutions to Advent Of Code 2025 implemented in Rust. The development environment is VS Code and WSL Ubuntu.

![Rust Workflow](https://github.com/jambolo/advent-of-code-2025/actions/workflows/rust.yml/badge.svg)

## Day 1

Simple modulo arithmetic. Other than getting those annoying boundary conditions right, it was not much of a challenge.

| Part | Answer |
|------|--------|
|    1 |   1150 |
|    2 |   6738 |

## Day 2

Trivial

| Part |    Answer   |
|------|-------------|
|    1 | 17077011375 |
|    2 | 36037497037 |

## Day 3

Trivial. I hope these start getting harder. There are only 12 days this year.

| Part |      Answer     |
|------|-----------------|
|    1 |           16854 |
|    2 | 167526011932478 |

## Day 4

Still trivial.

| Part | Answer |
|------|--------|
|    1 |   1489 |
|    2 |   8890 |

## Day 5

I didn't know that sort_by_key can be used on a vector of tuples.

| Part |      Answer     |
|------|-----------------|
|    1 |             615 |
|    2 | 353716783056994 |

## Day 6

Yawn. I considered loading the data into a matrix and transposing, but it was easier to just process it the naive way.

| Part |     Answer     |
|------|----------------|
|    1 |  5227286044585 |
|    2 | 10227753257799 |

## Day 7

I think this one was easy because AoC has trained me to expect a solution that doesn't require enumeration.

| Part |     Answer     |
|------|----------------|
|    1 |           1630 |
|    2 | 47857642990160 |

## Day 8

Disappointingly easy.

| Part |   Answer  |
|------|-----------|
|    1 |    123420 |
|    2 | 673096646 |

## Day 9

Careful what you ask for ...
This one took some thinking and fence posts were a problem. The fact that all edges were vertical or horizontal allowed for some crucial optimizations. Also, note that the largest rectangle in my input data happened to be an interior rectangle; however, there were two largest rectangles in the example input, one interior and one exterior. As a result, my initial attempt got the correct answer without testing for interior vs. exterior.

| Part |   Answer   |
|------|------------|
|    1 | 4741451444 |
|    2 | 1562459680 |

## Day 10

Well, that escalated quickly! ...
This one was actually fun because I got to try out a LP solver in Rust. However, I spent a *lot* of time reducing the size of problem before handing it to the solver, but it turned out that *none* of that work was necessary for this puzzle.

| Part | Answer |
|------|--------|
|    1 |    532 |
|    2 |  18387 |

## Day 11

After banging my head against the wall for a few hours, I finally came upon the key optimization.

| Part |      Answer     |
|------|-----------------|
|    1 |             523 |
|    2 | 517315308154944 |

## Day 12

Turned out to be trivial.

| Part | Answer |
|------|--------|
|    1 |    583 |

## Summary

### Rust Language

I did Advent of Code 2023 in Rust and had a difficult time with the language. This time it was a lot easier because I was more familiar with the language and I am more familiar with borrow/copy/move in Rust, which turns out to be different from C++. In Rust, move is the default and copy is an extra step, whereas copy is the default in C++ and move takes an extra step. Interestingly, borrow in Rust and references in C++ are similar in both function and syntax, but they operate differently and have different semantics.

I appreciate the borrow checker. Managing lifetimes and references is difficult to do in C++ and a huge source of bugs, but Rust eliminates that whole class of bugs. I wish Rust were more object-oriented, but I understand the tradeoffs.

Overall, I like Rust and I am likely to prefer it over C++ in the future.

### Algorithms and Techniques

This is a list of the algorithms and techniques used to solve this year's puzzles.

|       Algorithm / Technique       | Days  |
|-----------------------------------|-------|
| Modular arithmetic                | 1     |
| Ranges /intervals                 | 2, 5  |
| Cellular automaton                | 4     |
| Union-Find / connected components | 8     |
| Computational geometry            | 9     |
| Combinatorics                     | 10    |
| Integer linear programming (ILP)  | 10    |
| DAG traversal                     | 7, 11 |
| Graph reduction                   | 11    |
