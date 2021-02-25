# In scope

```
$ rank
Write some code

$ rank rename 'Check out the package'
Goal "Write some code" is now "Check out the package"

$ rank current
Current Tasks:
  - Write some code
  - Brew some tea

# Meh...
$ rank +1
Current Tasks:
  - Write some code
  - Brew some tea
Next up:
  - Drink some tea
  - Write more code

$ rank all
Current Tasks:
  - Write some code
  - Brew some tea
Next up:
  - Drink some tea
  - Write more code
3rd up:
  - Take over the world
4th up:
  - Take over the world
```

# Out of scope

The goals of `rank` are purposefully focused and minimal. Here are some features that were considered and will not be introduced:

- Undo/redo
- Backup/restore

