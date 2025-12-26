# advent-of-code-2025

My solutions to Advent Of Code 2025 implemented in Rust. The development environment is VS Code and Windows.

![Rust Workflow](https://github.com/jambolo/advent-of-code-2025/actions/workflows/rust.yml/badge.svg)

## Day 1

Simple modulo arithmetic. Other than getting those annoying boundary conditions right, it was not much of a challenge.

## Day 2

Trivial

## Day 3

Trivial. I hope these start getting harder. There are only 12 days this year.

## Day 4

Still trivial.

## Day 5

I didn't know that sort_by_key can be used on a vector of tuples.

## Day 6

Yawn. I considered loading the data into a matrix and transposing, but it was easier to just process it the naive way.

## Day 7

I think this one was easy because AoC has trained me to expect a solution that doesn't require enumeration.

## Day 8

Disappointingly easy.

## Day 9

Careful what you ask for ...
This one took some thinking and fenceposts were a problem. The fact that all edges were vertical or horizontal allowed for some crucial optimizations. Also, note that the largest rectangle in my input data happened to be an interior rectangle; however, there were two largest rectangles in the example input, one interior and one exterior. As a result, my initial attempt got the correct answer without testing for interior vs. exterior.

## Day 10

Well, that escalated quickly! ...
This one was actually fun because I got to try out a LP solver in Rust. However, I spent a *lot* of time reducing the size of problem before handing it to the solver, but it turned out that *none* of that work was necessary for this puzzle.

## Day 11

After banging my head against the wall for a few hours, I finally came upon the key optimization.

## Day 12

Turned out to be trivial.
