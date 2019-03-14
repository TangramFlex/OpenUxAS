I use the following settings to customize GDB. These should be written
in `~/.gdbinit`:

```
set auto-load safe-path /
set print pretty on
```

The `auto-load` entry is *required* in order for the local (i.e. in this
directory) `./.gdbinit` to be effective.

The `print pretty` entry is *recommended* to improve the presentation of
nested structures.
