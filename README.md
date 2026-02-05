# run

`run` is a tiny command runner that reads commands from a `run.txt` file and executes them by name.

Think of it as:
- **better than shell aliases** for long commands
- **lighter than `make`** when you just want to *run stuff*
- **one letter shorter than `make`**, so it’s objectively faster to type ;)

This is *not* a build system. It’s a shortcut engine for messy, ugly, pipe-heavy terminal commands with easy syntax.

---

## Why `run`?
Make files are hard to write, and its not even the intended purpose to use it as a shortcut engine for terminal commands.
I also wanted to practice writing in the rust programming language.
