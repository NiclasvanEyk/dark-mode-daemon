# Dark Mode Daemon

Run scripts when the system color scheme changes between light and dark.

## Installation

TODO

## Usage

Place executable scripts in one of the following locations:

  - `$XDG_CONFIG_HOME/dark-mode-daemon/scripts`
  - `$HOME/.config/dark-mode-daemon/scripts`

You can use the following as an example

```bash
#!/usr/bin/env bash

say "Changed to $DMD_COLOR_MODE mode"
```

Dont forget to make it executable (`chmod +x myscript.sh`)!
Then, if not automatically run at startup through Homebrew, run

```shell
dark-mode-daemon daemon
```

to begin listening for color mode changes.

If you used the example, you should hear the phrase "Changed to dark mode".

