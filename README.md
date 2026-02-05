# run

`run` is a tiny command runner that reads commands from a `run.txt` file and executes them by name.

Think of it as:
- **better than shell aliases** for long commands
- **lighter than `make`** when you just want to *run stuff*
- **one letter shorter than `make`**, so it’s objectively faster to type ;)

This is *not* a build system. It’s a shortcut engine for messy, ugly, pipe-heavy terminal commands with easy syntax.

## Usage

Create a `run.txt` file in the directory you want to work in.

Define commands using this format:
`command: <your very long command && second long command && third long command`

### example run.txt
```
default: fastfetch
assemble: as -o fizzbuzz.o fizzbuzz.s && ld -o fizzbuzz fizzbuzz.o
flex: fastfetch
```

### how run.txt files work
Doing `./run <key>` will run whatever you type after the ':' key in that same line. Each key is only one line long.

Your commands are called "keys". You can make your keys whatever name you want.
If you have a "default" key in your run.txt, then _running_ `./run` with no arguments will execute the "default" key.

---

## Why `run`?
Make files are hard to write, and its not even the intended purpose to use it as a shortcut engine for terminal commands.
I also wanted to practice writing in the rust programming language.
