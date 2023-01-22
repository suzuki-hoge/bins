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

### Keys

| key   | description                                                             | constraint      |
|-------|-------------------------------------------------------------------------|-----------------|
| C-n   | Create ***n***ew empty your own build command.<br>It appears on pane 1. | input area only |
| C-d   | **D**elete project.<br>It disappears on pane 1.                         | input area only |
| Enter | Fix current line, then run selected line.                               | input area only |
| C-s   | **S**ave your own build command.                                        | pane 2 only     |
| Tab   | Jump cursor alternately between input area and pane 2.                  ||

## Struct

```
- Command
  |-- Label --> [ raw ] --> Pane 1 --> [ fix ] --+
  `-- Value                                      +--> Run and exit
```

## File IO

| file                        | description                                                                    |
|-----------------------------|--------------------------------------------------------------------------------|
| ~/.bins-preset-actions.yaml | Read preset actions.<br>If the file missing, it will be created automatically. |
