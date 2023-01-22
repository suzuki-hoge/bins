# Build command Launcher ( bb )
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

### Ctrl + Space ( input area only )
Fix current line.

The selected line disappears, and you can continue to select.

### Ctrl + f ( input area only )
***F***inish selection, then run selected line(s).

### Enter ( input area only )
Fix current line, then run selected line(s).

( `Enter` equals `Ctrl + Space` & `Ctrl + f` )

### Ctrl + n ( input area only )
Create ***n***ew empty your own build command.

It appears on pane 1.

### Ctrl + d ( input area only )
***D***elete your own build command.

It disappears on pane 1.

### Ctrl + s ( pane 2 only )
***S***ave your own build command.

### Tab
Jump cursor alternately between input area and pane 2.

## Struct
```
- Command
  |-- Label --> [ raw ] --> Pane 1 --> [ fix ] --> Run and exit
  `-- Lines --> [ raw ] --> Pane 2
```

## File IO
Read current directory's build file.

### ./Makefile
If missing, skip parse.

### ./package.json
If missing, skip parse.

### ./yarn.lock
If found, run with `yarn`.

### ./package-lock.json
If found it only, run with `npm`.

### ~/.bins-project-mapper
If missing, fail launch.

Check [Migration](#).
