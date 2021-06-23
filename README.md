[![crates.io version](https://img.shields.io/crates/v/sigi)](https://crates.io/crates/sigi)
[![crates.io downloads](https://img.shields.io/crates/d/sigi?label=crates.io%20downloads)](https://crates.io/crates/sigi)
[![docs.rs docs](https://docs.rs/mio/badge.svg)](https://docs.rs/sigi)

# Sigi

Sigi is an organizing tool.

It's primarily intended for you to use as extra memory. Use it to organize your
tasks, groceries, or the next board games you want to play.

Sigi can also be used as a stack-management tool. It can be used as disk-
persistent stack memory, for example, in a shell script or in Rust code.

```
sigi 1.0.1
An organizational tool.

USAGE:
    sigi [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Omit any leading labels or symbols. Recommended for use in shell scripts
    -s, --silent     Omit any output at all.
    -V, --version    Prints version information
    -v, --verbose    Enable verbose output.

OPTIONS:
    -t, --stack <STACK>    Manage items in a specific stack [aliases: topic, about, namespace]

SUBCOMMANDS:
    complete      Move the current item to "<STACK>_completed" [aliases: done, finish, fulfill]
    create        Create a new item [aliases: push, add, do, start, new]
    delete        Move the current item to "<STACK>_deleted" [aliases: pop, remove, cancel, drop]
    delete-all    Move all items to "<STACK>_deleted" [aliases: purge, pop-all, remove-all, cancel-all, drop-all]
    head          List the first N items [aliases: top, first]
    help          Prints this message or the help of the given subcommand(s)
    is-empty      "true" if stack has no items, "false" otherwise [aliases: empty]
    length        Print the stack's length [aliases: count, size]
    list          List all items [aliases: ls, snoop, show, all]
    move          Move current item to another stack
    move-all      Move all items to another stack
    next          Cycle to the next item; the current item becomes last [aliases: later, cycle, bury]
    peek          Show the current item (This is the default behavior when no command is given) [aliases: show]
    pick          Move items to the top of stack by their number
    rot           Rotate the three most-current items [aliases: rotate]
    swap          Swap the two most-current items
    tail          List the last N items [aliases: bottom, last]
```

# Motivation

_Sigi_ is the [Chamorro](https://en.wikipedia.org/wiki/Chamorro_language) word
for _continue_. I hope it will help you to plan more, forget less, get things
done, and relax. 🌴

There's [a limit](https://wiki.c2.com/?SevenPlusOrMinusTwo) to human memory, and
remembering things uses up [willpower](https://www.penguinrandomhouse.com/books/307740/willpower-by-roy-f-baumeister-and-john-tierney/).
I like working at a command line, and wanted a tool to free me up from trying to
juggle tasks and ideas.

I also just like stacks, and stack-based languages like
[Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) and
[Factor](https://factorcode.org) are a joy to play with.

# Examples

## Sigi as a to-do list

Sigi can understand `do` (create a task) and `done` (complete a task).

```
$ alias todo='sigi --stack todo'

$ todo do Write some code
Creating: Write some code

$ todo do Get a drink
Creating: Get a drink

$ todo do Take a nap
Creating: Take a nap

$ todo list
Now: Take a nap
  1: Get a drink
  2: Write some code

$ sleep 20m

$ todo done
Completed: Take a nap
```

It's best to use sigi behind a few aliases with unique "stacks". You should
save these aliases in your `~/.bashrc` or `~/.zshrc` or whatever your shell has
for configuration. Sigi accepts a `--stack` flag that indicates a unique list.
You can have as many stacks as you can think of names.

Forgot what to do next?

```
$ todo
Now: Get a drink
```

Not going to do it?

```
$ todo delete
Deleted: Get a drink
```

## Sigi as a save-anything list

Extending the alias idea, you can use sigi to store anything you want to
remember later.

```
$ alias watch-later='sigi --stack watch-later'

$ watch-later add One Punch Man
Creating: One Punch Man
```

```
$ alias story-ideas='sigi --stack=story-ideas'

$ story-ideas add Alien race lives backwards through time.
Creating: Alien race lives backwards through time.
```

## Sigi as a local stack-based database

Sigi understands the programmer-familiar `push` (create an item) and `pop`
(remove an item and return it) idioms.

Using the `--quiet` (or `-q`) flag is recommended for shell scripts, as it
leaves out any leading labels or symbols.

TODO: Need an example, maybe a reverse polish notation calculator in bash?

# Installing

## Command-line interface (CLI)

Currently the best way to install sigi is through the Rust language package
manager, cargo:

```
cargo install sigi
```

Instructions on installing cargo can be found here:

- https://doc.rust-lang.org/cargo/getting-started/installation.html

In the future I plan to distribute sigi through more package managers.

# Library

Sigi is available as a Rust library via [crates.io](https://crates.io/crates/sigi).

It is still in active, unstable development, so I suggest not doing anything
ambitious until stable versions (i.e. >= 1.0) become available.

In the future I plan to provide wrappers through other languages. Also, the
implementation language is possibly subject to change.
