# In scope

```
$ rank
Write some code

$ rank rename Check out the package
Goal "Write some code" is now "Check out the package"

$ rank goals
Current Tasks:
  - Write some code
  - Brew some tea

$ rank all
Current Tasks:
  - Write some code
  - Drink some tea
Next up:
  - Write more code
3rd up:
  - Take over the world
4th up:
  - Schedule a vacation

$ rank
Write some code

$ rank done
☑ Write some code
Next: Drink some tea

$ rank
Drink some tea

$ rank push Brew some coffee

$ rank
Brew some coffee

$ rank goals
  - Brew some coffee
  - Drink some tea

$ rank drop
Goal "Brew some coffee" destroyed

$ rank push Brew some tea

$ rank goals
  - Brew some tea
  - Drink some tea

$ rank promote

$ rank goals
  - Brew some tea

$ rank all
Current Tasks:
  - Brew some tea
Next up:
  - Drink some tea
3rd up:
  - Write more code
4th up:
  - Take over the world
5th up:
  - Schedule a vacation
```

# Data structure

## Rank

{
	"name": "Drink some tea",
	"created": "2021-02-24T20:53:17-08:00"
}

{
	"name": "Drink some tea",
	"created": "2021-02-25T04:53:17Z",
	"done": "2021-02-25T04:56:19Z"
}

# Storage

TODO: Should this be (e.g.) yml or toml? (To allow append-write instead of something more fiddly)

/etc/rank
├── goals
|   ├── rank.json
│   ├── uuid-1.json
│   └── uuid-2.json
└── history
    ├── archive
    │   └── 2021-01.tar.xz
    ├── 2021-02-14
    │   ├── uuid-3.json
    │   └── uuid-4.json
    └── 2021-02-25
        └── uuid-5.json

# Out of scope

The goals of `rank` are purposefully focused and minimal. Here are some features that were considered and will not be introduced:

- Undo/redo
- Backup/restore

