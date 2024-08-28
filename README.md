# Aura RS

[Aura Lang](https://github.com/auralangco/aura) features implemented in Rust

## What is Aura RS

Aura RS is a crate that serves as a playground for some ideas from the Aura programming language

## Features

### Either

A type that is similar to `Result` but without the semantics of right and wrong. Has two variants `Left` and `Right`

### Recur (`loop`)

Aura `loop` function implemented as `recur`.

Recur is an immutable looping structure. Given a initial value and a function that returns either `Continue(c)` or `Break(b)`. Where `c` is the input value of the next iteration and `b` is the resulting type of the loop. 