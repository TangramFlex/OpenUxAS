TUI interfaces make GDB more effective by presenting context in additional
panes. Several such interfaces exist, including `tgdb` (or `cgdb`, depending
upon distro), `insight`, and GDB's own `set tui` mode.

GDB and its TUIs depend upon `readline`. While `readline` supports both Emacs
and vi bindings, GDB simply will not function when `readline` uses vi bindings.
If you are affected by this, the simplest solution is to invoke GDB (or your
choice of TUI) without a `readline` configuration:

$ env INPUTRC=/dev/null <gdb-command ...>
