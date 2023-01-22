# Filter ( bf )
***F***ilter lines, and output selected line(s).

## Usage: Launch selector
Launch with piped stdin.
```
$ ps aux | bf
```

Launch with command substitution.
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

### Ctrl + Space
Fix current line.

The selected line disappears, and you can continue to select.

### Ctrl + f
***F***inish selection, then output selected line(s).

### Enter
Fix current line, then output selected line(s).

( `Enter` equals `Ctrl + Space` & `Ctrl + f` )

## Struct
```
- String --> [ raw ] --> Pane 1 --> [ fix ] --> Print and exit
```

## File IO
No file io.
