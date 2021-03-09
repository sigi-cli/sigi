# In scope

```
$ sigi
Write some code

$ sigi rename Check out the package
Goal "Write some code" is now "Check out the package"

$ sigi goals
Current Tasks:
  - Write some code
  - Brew some tea

$ sigi all
Current Tasks:
  - Write some code
  - Drink some tea
Next up:
  - Write more code
3rd up:
  - Take over the world
  - Schedule a vacation

$ sigi
Write some code

$ sigi done
☑ Write some code
Next: Drink some tea

$ sigi
Drink some tea

$ sigi push Brew some coffee

$ sigi
Brew some coffee

$ sigi goals
  - Brew some coffee
  - Drink some tea

$ sigi drop
Goal "Brew some coffee" destroyed

$ sigi push Brew some tea

$ sigi goals
  - Brew some tea
  - Drink some tea

$ sigi promote

$ sigi goals
  - Brew some tea

$ sigi all
Current Tasks:
  - Brew some tea
Next up:
  - Drink some tea
3rd up:
  - Write more code
4th up:
  - Take over the world
  - Schedule a vacation
```

## Data structure

### Sigi

{
	"name": "Drink some tea",
	"created": "2021-02-24T20:53:17-08:00"
}

{
	"name": "Drink some tea",
	"created": "2021-02-25T04:53:17Z",
	"done": "2021-02-25T04:56:19Z"
}

## Storage

TODO: Should this be (e.g.) yml or toml? (To allow append-write instead of something more fiddly)

/etc/sigi
├── goals
|   ├── sigi.json
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

The goals of `sigi` are purposefully focused and minimal. Here are some features that have been considered and have no plan to be introduced:

- Undo/redo
- Backup/restore
