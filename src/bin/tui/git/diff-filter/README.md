# Diff Filter ( bgdf )

Show ***G***it ***d***i***f***f .

## Usage: Launch selector

```
```

```
$ gdff
```

```
+-------------------------------------------------------------------------------------------------+
|  M Cargo.lock                                                                                   |
|  M Cargo.toml                                                                                   |
|  A src/bin/foo.rs                                                                               |
|                                                                                                 |
|                                                                                                 |
+-------------------------------------------------------------------------------------------------+
```

### Input search works on

- Pane 1 text

### Keys
| key     | description                                                                           | constraint |
|---------|---------------------------------------------------------------------------------------|------------|
| C-Space | Fix current line.<br>The selected line disappears, and you can continue to select.    |            |
| C-f     | ***F***inish selection, then run selected line(s).                                    |            |
| C-Enter | Fix current line, then run selected line(s).<br> ( `Enter` equals `C-Space` & `C-f` ) |            |

## Struct

```
- String --> [ raw ] --> Pane 1 --> [ fix ] --> Show diff
```

## File IO

No file io.
