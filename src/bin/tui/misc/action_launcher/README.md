# Action Launcher ( ba )
Run preset ***a***ction.

## Usage: Launch selector
```
$ ba
```

```
> 
+------------------------------+------------------------------------------------------------------+
|   ssh front test             | ssh -i ~/.ssh/foo-prod.pem ec2-user@xxx.xxx.xxx.xxx              |
| * ssh front prod             |                                                                  |
|   copy aurora url            |                                                                  |
|   repl: rust                 |                                                                  |
|   repl: haskell              |                                                                  |
|   repl: typescript           |                                                                  |
|   repl: scala                |                                                                  |
|   repl: php81                |                                                                  |
|   repl: php56                |                                                                  |
+------------------------------+------------------------------------------------------------------+
```

### Input search works on
- Pane 1 text

### Enter ( input area only )
Fix current line, then run selected line.

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
  |-- Label --> [ raw ] --> Pane 1 --> [ fix ] --+
  `-- Value                                      +--> Run and exit
```

## File IO
### ~/.bins-preset-actions.yaml
Read preset actions. If the file missing, it will be created automatically.
