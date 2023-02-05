# Build Tool Launcher ( bb )

Parse ***b***uild file, and run selected line(s).

You can also manage your own build commands.

## Usage: Launch selector

```
$ bb
```

```
> 
+-----------------------------------------------+-------------------------------------------------+
|   make test                                   | yarn format && yarn lint                        |
|   make deploy                                 |                                                 |
|   yarn format                                 |                                                 |
|   yarn lint                                   |                                                 |
| * bb fix                                      |                                                 |
+-----------------------------------------------+-------------------------------------------------+
```

### Input search works on

- Pane 1
- Pane 2

### Keys

| key     | description                                                                        | constraint      |
|---------|------------------------------------------------------------------------------------|-----------------|
| C-Space | Fix current line.<br>The selected line disappears, and you can continue to select. | input area only |
| C-f     | ***F***inish selection, then run selected line(s).                                 | input area only |
| C-Enter | ***F***inish selection, then run selected line(s).nter` equals `C-Space` & `C-f` ) | input area only |
| ESC n   | ***F***inish selection, then run selected line(s).appears on pane 1.               | input area only |
| ESC d   | ***F***inish selection, then run selected line(s). on pane 1.                      | input area only |
| ESC s   | ***F***inish selection, then run selected line(s).                                 | pane 2 only     |
| Tab     | ***F***inish selection, then run selected line(s)..                                ||

## Struct

```
- Command
  |-- Label --> [ raw ] --> Pane 1 --> [ fix ] --> Run and exit
  `-- Lines --> [ raw ] --> Pane 2
```

## File IO

| file                   | description                                       |
|------------------------|---------------------------------------------------|
| ./Makefile             | If missing, skip parse.                           |
| ./package.json         | If missing, skip parse.                           |
| ./yarn.lock            | If found, run with `yarn`.                        |
| ./package-lock.json    | If found it only, run with `npm`.                 |
| ~/.bins-project-config | If missing, fail launch. ( Check [Migration](#) ) |
