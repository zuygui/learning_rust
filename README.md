# Fibonnaci suite

## Logic of the exercice

This is a fun puzzle game where the objective is to move an entire stack of disks from the source position to another position. Three simple rules are followed:

- Only one disk can be moved at a time.
- Each move consists of taking the upper disk from one of the stacks and placing it on top of another stack. In other words, a disk can only be moved if it is the uppermost disk on a stack.
- No larger disk may be placed on top of a smaller disk.
  Now, let’s try to imagine a scenario. Suppose we have a stack of three disks. Our job is to move this stack from source A to destination C with the help of the intermediate point B.

## TODO

Write an program for solve the Hanoi Tower problem in Rust.

## Utils resources

[Wikipedia FR](https://fr.wikipedia.org/wiki/Tours_de_Hano%C3%AF)

## Output of the program

Using 4 disk

```
Move disk from pole 1 to pole 3
Move disk from pole 1 to pole 2
Move disk from pole 3 to pole 2
Move disk from pole 1 to pole 3
Move disk from pole 2 to pole 1
Move disk from pole 2 to pole 3
Move disk from pole 1 to pole 3
Move disk from pole 1 to pole 2
Move disk from pole 3 to pole 2
Move disk from pole 3 to pole 1
Move disk from pole 2 to pole 1
Move disk from pole 3 to pole 2
Move disk from pole 1 to pole 3
Move disk from pole 1 to pole 2
Move disk from pole 3 to pole 2
```
