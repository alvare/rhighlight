# rhighlight
Highlight patterns on standard input.

Usage:
```
cat some_log.log | rhighlight 'some.regex$'
```
Will print stdin "as is" but with the pattern in ANSI-terminal red.
