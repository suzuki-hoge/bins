# Build Command Launcher ( bb )

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

| key     | description                                                                           | constraint      |
|---------|---------------------------------------------------------------------------------------|-----------------|
| C-Space | Fix current line.<br>The selected line disappears, and you can continue to select.    | input area only |
| C-f     | ***F***inish selection, then run selected line(s).                                    | input area only |
| C-Enter | Fix current line, then run selected line(s).<br> ( `Enter` equals `C-Space` & `C-f` ) | input area only |
| C-n     | Create ***n***ew empty your own build command.<br>It appears on pane 1.               | input area only |
| C-d     | ***D***elete your own build command.<br>It disappears on pane 1.                      | input area only |
| C-s     | ***S***ave your own build command.                                                    | pane 2 only     |
| Tab     | Jump cursor alternately between input area and pane 2.                                ||

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
| ~/.bins-project-mapper | If missing, fail launch. ( Check [Migration](#) ) |
