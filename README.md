[![crates.io version](https://img.shields.io/crates/v/sigi)](https://crates.io/crates/sigi)
[![crates.io downloads](https://img.shields.io/crates/d/sigi?label=crates.io%20downloads)](https://crates.io/crates/sigi)
[![docs.rs docs](https://docs.rs/mio/badge.svg)](https://docs.rs/sigi)

# `sigi`

`sigi` is an organizing tool for terminal lovers who hate organizing

Use `sigi` as extra memory. Use it to toss your tasks, groceries, or the next
board games you want to play onto a stack. Shell aliases are encouraged to
organize your various stacks.

```
sigi 3.2.2
An organizing tool for terminal lovers who hate organizing

USAGE:
    sigi [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -f, --format <FORMAT>    Use a programmatic format. Options include [csv, json, json-compact,
                             tsv]. Not compatible with quiet/silent/verbose
    -h, --help               Print help information
    -q, --quiet              Omit any leading labels or symbols. Recommended for use in shell
                             scripts
    -s, --silent             Omit any output at all
    -t, --stack <STACK>      Manage items in a specific stack [aliases: topic, about, namespace]
    -v, --verbose            Print more information, like when an item was created [aliases: noisy]
    -V, --version            Print version information

SUBCOMMANDS:
    complete       Move the current item to "<STACK>_history" and mark as completed [aliases:
                       done, finish, fulfill]
    count          Print the total number of items in the stack [aliases: size, length]
    delete         Move the current item to "<STACK>_history" and mark as deleted [aliases: pop,
                       remove, cancel, drop]
    delete-all     Move all items to "<STACK>_history" and mark as deleted [aliases: purge, pop-
                       all, remove-all, cancel-all, drop-all]
    head           List the first N items (default is 10) [aliases: top, first]
    help           Print this message or the help of the given subcommand(s)
    interactive    Run in an interactive mode [aliases: i]
    is-empty       Print "true" if stack has zero items, or print "false" (and exit with a
                       nonzero exit code) if the stack does have items [aliases: empty]
    list           List all items [aliases: snoop, show, all]
    list-stacks    List all stacks [aliases: stacks]
    move           Move current item to another stack
    move-all       Move all items to another stack
    next           Cycle to the next item; the current item becomes last [aliases: later, cycle,
                       bury]
    peek           Show the first item. This is the default behavior when no command is given
                       [aliases: show]
    pick           Move items to the top of stack by their number
    push           Create a new item [aliases: create, add, do, start, new]
    rot            Rotate the three most-current items [aliases: rotate]
    swap           Swap the two most-current items
    tail           List the last N items (default is 10) [aliases: bottom, last]

INTERACTIVE MODE:

Use subcommands in interactive mode directly. No OPTIONS (flags) are understood in interactive mode.

The following additional commands are available:
    ?               Show the short version of "help"
    stack           Change to the specified stack
    quit/q/exit     Quit interactive mode
```

# The big idea

_Sigi_ is the [Chamorro](https://en.wikipedia.org/wiki/Chamorro_language) word
for _continue_. I hope it will help you to get on with your life, by helping
you prioritize better, forget less, get some stuff done, and relax. 🌴

There's [a limit](https://wiki.c2.com/?SevenPlusOrMinusTwo) to human memory, and
remembering things uses up [willpower](https://www.penguinrandomhouse.com/books/307740/willpower-by-roy-f-baumeister-and-john-tierney/).
I like working at a command line, and wanted a tool to free me up from trying to
juggle tasks and ideas.

On a more personal level, while I love beauty and craftsmanship and
accomplishing things... I absolutely despise spending time organizing. A five
factor personality evaluation rated me at 7% in the orderliness aspect of
[conscientousness](https://en.wikipedia.org/wiki/Conscientiousness), which
(among other things) means I am both less disturbed by chaos and more disturbed
by order than 92% of humankind.

`sigi` intends to be far _more_ flexible and messy, and far _less_ rigid and
tidy, when compared to the plethora of personal and professional organizational
tools that exist. It will let you write stuff down and look at that stuff when
you want to. It's less like a Google/Outlook/Apple Calendar application and
more like a pen-and-paper notebook.

I also just find that stacks, and stack-based languages like
[Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) and
[Factor](https://factorcode.org) are a joy to play with. Also, they're a good
fit for organization. Usually older things that aren't "done" or "deletable"
are things that can wait more than whatever things are actively being juggled.

# Examples

## `sigi` as a to-do list

`sigi` can understand `do` (create a task) and `done` (complete a task).

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

It's best to use `sigi` behind a few aliases with unique "stacks". You should
save these aliases in your `~/.bashrc` or `~/.zshrc` or whatever your shell has
for configuration. `sigi` accepts a `--stack` option, and you can have as many
stacks as you can think of names.

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

## `sigi` as a save-anything list

Extending the alias idea, you can use `sigi` to store anything you want to
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

## `sigi` as a local stack-based database

`sigi` understands the programmer-familiar `push` and `pop` idioms. It can be
used for simple, persistent, small-scale stack use-cases.

`sigi` is not intending to be highly performant. While no limits are enforced,
it would not handle high, concurrent throughput well. It also isn't suitable
for enormous amounts of data. For something beefier with stack semantics,
check out Redis.

Using the `--quiet` (or `-q`) flag is recommended for shell scripts, as it
leaves out any leading labels or symbols.

# Installing

[![Packaging status](https://repology.org/badge/vertical-allrepos/sigi.svg)](https://repology.org/project/sigi/versions)

If your packaging system doesn't have it yet, the best way to install `sigi` is
through the Rust language package manager, `cargo`:

```
cargo install sigi
```

Instructions on installing `cargo` can be found here:

- https://doc.rust-lang.org/cargo/getting-started/installation.html

Please package it up for your Linux/BSD/etc distribution.

# Contributing and support

Please [open an issue](https://github.com/hiljusti/sigi/issues) if you see
bugs or have ideas!

I'm looking for people to use [the `sigi` wiki](https://github.com/hiljusti/sigi/wiki)
to share their tips, tricks, and examples.

Thanks for checking it out!
