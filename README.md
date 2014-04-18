emoji-shell
===========

:shell::shell::shell:

emoji-shell is a shell I wrote in Rust out of my frustration with the usability of existing shells. It uses emoji for maximum ease of use and efficiency.

## Building

Just run

```rust
rustc shell.rs && ./shell
```

I wrote emoji-shell with `rustc 0.11-pre-nightly (e415c25 2014-04-07 21:21:47 -0700)`.

## Instructions

To run commands, just type them in:

> vim

Putting things in quotes is *sooo* exhausting, so emoji-shell uses tomatoes between things instead of spaces. Put tomatoes between command line parameters. You're probably wondering "why tomatoes?" That's a good question.

> vim :tomato: cat.rs :tomato: meow.txt

Naturally, you can print the current directory with 

> :round_pushpin:

And you can change directory with a car. Don't forget to add a tomato between command line parameters and programs.

> :car: :tomato: /Users/robert/Cat Photos

Notice that you don't need to escape the space in `Cat Photos` above...that's the magic of :tomato:!

To `ls`, use :mag:, to `exit`, use :x:, and of course `cat` is :cat:.

Enjoy!
