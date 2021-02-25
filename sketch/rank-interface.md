$ rank
Write some code

$ rank rename 'Check out the package'
Goal "Write some code" is now "Check out the package"

$ rank current
Current Tasks:
  - Write some code
  - Brew some tea

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
Next+1:
  - Take over the world
Next+2:
  - Take over the world



---

The goals of `rank` are purposefully focused and minimal. Here are some features that were considered and will not be introduced:

- Undo/redo
- Backup/restore

