# Git Status Dispatcher ( bgs )

Dispatch action to ***G***it ***s***tatus.

## Usage: Launch selector

```
$ bgs
```

```
> 
+-------------------------------------------------------------------------------------------------+
| M  src/bin/git.rs                                                                               |
|  D src/bin/config.rs                                                                            |
|  A src/bin/util.rs                                                                              |
|                                                                                                 |
+-------------------------------------------------------------------------------------------------+
```

### Input search works on

- Pane 1 text

### Keys

| key | description     | constraint |
|-----|-----------------|------------|
| C-a | ***A***dd.      |            |
| C-r | ***R***eset.    |            |
| C-c | ***C***heckout. |            |
| C-d | ***D***iff.     |            |
| Esc | exit.           |            |

## Struct

```
- File
  |-- Name   --+--> [ raw ] --> Pane 1 --> [ fix ] --> Run
  `-- Status --+
```

## File IO

| file          | description                                       |
|---------------|---------------------------------------------------|
| ./.git/config | If missing, fail launch.                          |
