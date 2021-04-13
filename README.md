# Sigi

Sigi is an organizing tool and no-frills stack database.

It's primarily intended for you to use as extra memory. Use it to organize your
tasks, groceries, or the next board games you want to play.

Sigi is also a stack-management tool. It can be used as disk-persistent stack
memory, for example, in a shell script or in Rust code. (And more languages in
the future!)

```
sigi 0.2.3
An organizational tool.

USAGE:
    sigi [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Omit any leading labels or symbols. Recommended for use in shell scripts
    -s, --silent     Omit any output at all.
    -V, --version    Prints version information

OPTIONS:
    -t, --stack <STACK>    Manage items in a specific stack [aliases: topic, about, namespace]

SUBCOMMANDS:
    complete      Mark the current item as successfully completed [aliases: done, finish, fulfill]
    create        Create a new item [aliases: push, add, do, start, new]
    delete        Delete the current item [aliases: pop, remove, cancel, drop, abandon, retire]
    delete-all    Delete all items [aliases: purge, pop-all, remove-all, cancel-all, drop-all, abandon-all, retire-all]
    help          Prints this message or the help of the given subcommand(s)
    is-empty      Determine if stack is empty [aliases: empty]
    length        Count all items [aliases: count, size]
    list          List all items [aliases: ls, show, all, list-all]
    move          Move current item to destination
    move-all      Move all items to destination stack
    next          Move the next item to current, and moves current to last [aliases: later, punt, bury]
    peek          Peek at the current item (This is the default behavior when no command is given) [aliases: show]
    pick          Move the specified indices to the top of stack
    rot           Rotate the three most-current items [aliases: rotate]
    swap          Swap the two most current items
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

# Similar projects

If sigi doesn't do quite what you want, check out these similar projects. Sigi
was created before I found these, but some inspiration may be gleaned from them
for improvement. Most in this list predate sigi by several years.

## Similar CLIs

- [devtodo](https://swapoff.org/devtodo.html) - A hierarchical command-line task manager
- [dstask](https://github.com/naggie/dstask) - Single binary terminal-based TODO manager with git-based sync + markdown notes per task
- [grit](https://github.com/climech/grit) - Multitree-based task manager
- [node-todo-cli](https://www.npmjs.com/package/node-todo-cli) - A command line program that manages todo tasks
- [py-todo-cli](https://github.com/Mantaseus/Todo-CLI) - A simple command line Todo program written in Python
- [taskell](https://taskell.app) - Command-line Kanban board/task management
- [taskwarrior](https://taskwarrior.org) - Taskwarrior is Free and Open Source Software that manages your TODO list from the command line
- [tax](https://github.com/netgusto/tax) - CLI task list manager
- [todo cli](https://gitlab.com/bigfiga99/todo-cli) - Todo CLI is a simple program that uses a sqlite3 database to keep track of your tasks
- [todo.txt](http://todotxt.org) - Future-proof task tracking in a file you control

### Similar CLI Definitions

- gophercises #7: [task](https://github.com/gophercises/task) - TODO CLI definition. (Defines a CLI)
- [pushpop](https://github.com/secretGeek/pushpop) - "Mental stack manager" definition. (Defines both a GUI and CLI)
  - See also implementations in [sh](https://paste.sr.ht/~erazemkokot/c6aeb2a7bc25049d08825b3cc7aea63b5cf72a08), [power shell](https://github.com/kberridge/psushpop/blob/master/psushpop.psm1)

## Similar non-CLI apps

- Too many to count. TODO apps are kind of the canonical JavaScript "first big project." They're also ubiquitous in mobile app stores.

## Similar Databases

- [piladb](https://github.com/fern4lvarez/piladb) - Stack-based database. (A working REST API and Database)
