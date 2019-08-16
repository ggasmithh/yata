# Why?
As I wrote untitled_rl, a roguelike game, as a C++ refresher, and yarl (another roguelike purged from internet as a matter of public safety/sanity) as a means to learn C, I'm using this project as a means to learn Rust.

# What?
My past game-like projects have taught me to start at a scope of roughly half of what I think I can handle. I want to make this a text-based adventure, so I'll make a finite state machine instead and go from there. My moonshot end-goal is to have yata be merely a text-based adventure engine, reading states, transitions, etc, from an external file.

# Thoughts
 * I'm beginging to question whether a FSM is the best pattern to use for a text adventure. It makes sense for locations to be implemented as States, but
the presentation of content--to me--lends itself toward object-oriented patterns.

# Post-minimum viable stuff
## Engine
* JSON Parser to read Content (Location/Item descriptions) from an external, mutable file.

## Finite State Machine
* Additional extra-gameplay states (Start, End, Main Menu, etc)
* Player inventory

## Content
* Item descriptions

# ~~Minimum viable game requirements~~
## ~~Finite State Machine~~
* ~~Conditional Branching~~

## ~~Content~~
* ~~State descriptions~~
* ~~State transition descriptions~~

