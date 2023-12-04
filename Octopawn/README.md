# AIFinalProject
Pedro Gonzalez

Professor Bart Massey

CS 441 : AI

## Octopawn

Octopawn is the 4×4 variant of [Hexapawn] (https://en.wikipedia.org/wiki/Hexapawn) Links to an external site.

A game created by Martin Gardner, to be easily solved with pencil-and-paper.

Please see the Wikipedia link for the rules of Hexapawn / Octopawn.

## Description

A game state in Octopawn is the positions of the pawns on the board together with the side on move. 

White pawns are represented by P and black pawns by p. For example

..p.

p..P

..p.

PP..

is a legal board state. 

If it is White's turn to move, White wins by advancing its rightmost pawn. 

(If it is Black's turn to move, Black wins by advancing its pawn in column 3.) 

For brevity and utility in indexing, we will represent this board with White to move as "..p.p..P..p.PP..W", 

that is, the scan of the board from top-to-bottom left-to-right and then the side on move.

Your solution to Octapawn should consist of a JSON file 4pawn.json containing a "JSON object" (dictionary), 

with each key being a board state as described and the corresponding value being either 1 (side on move wins) or -1 (side on move loses). 

Note that Octapawn has no draws. For example, your solution file might start

## Project Notes
This project was in line with a lot of the HW assignments from this class, and also some of Bart's Rust class. 

I found it straight forward which was a relief considering how much work the other portion of this final assignment was. 

Negamax continues to be a nice solution to zero sum games.

In order to run this project you must have Rust and Crate installed.

You can install them [here](https://www.rust-lang.org/)

Then the command to start the program is cargo run.