# Filter ( bf )

***F***ilter lines, and output selected line(s).

## Usage: Launch selector

```
$ vi `find . -name '.rs' | bf`
```

```
> src fil
+-------------------------------------------------------------------------------------------------+
| ./src/bin/filter/ui.rs                                                                          |
| ./src/bin/filter/runner.rs                                                                      |
| ./src/bin/filter/main.rs                                                                        |
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
- String --> [ raw ] --> Pane 1 --> [ fix ] --> Print and exit
```

## File IO

No file io.
