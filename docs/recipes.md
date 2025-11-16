# Recipes

A few example scripts for inspiration.
Feel free to open PRs to add some more.

If you arrived here from the Readme, make sure you create the necessary directory first

```shell
mkdir $HOME/.config/dark-mode-daemon/scripts
```

> [!NOTE]  
> We also respect the `XDG_CONFIG_HOME` environment variable if you prefer a different layout.

The remainder of this document lists a few example files you can put in your `dark-mode-daemon/scripts/` directory.

> [!IMPORTANT]  
>  Make sure that you adjust the permissions of the scripts after copy pasting them from here!
> ```shell
> chmod +x my-script.sh
> ```
> Otherwise they won't be executed.

## [Alacritty](https://alacritty.org)

While being quite performant, Alacritty is also quite minimal.
It does not support an automatic dark/light theme, but Dark Mode Daemon can help here.

Let's assume you've followed the default instructions from [`alacritty`](https://github.com/alacritty/alacritty-theme) and your themes are stored in `~/.config/alacritty/themes/themes/{theme}.toml`.
Then the following script will automatically sync the theme with your preferred themes:

```shell
#!/usr/bin/env bash

CONFIG="$HOME/.config/alacritty/alacritty.toml"
LIGHT_THEME="alabaster"
DARK_THEME="alabaster_dark"

if [[ "$DMD_COLOR_MODE" = "light" ]]
then
  sed -i.bak "s/themes\/themes\/.*\.toml/themes\/themes\/$LIGHT_THEME.toml/" "$CONFIG";
fi

if [[ "$DMD_COLOR_MODE" = "dark" ]]
then
  sed -i.bak "s/themes\/themes\/.*\.toml/themes\/themes\/$DARK_THEME.toml/" "$CONFIG";
fi
```

Ensure you have the live-reloading feature of alacritty enabled!

## [Fish](https://fishshell.com)

The Fish shell has a _really_ cool feature called [Universal Variables](https://fishshell.com/docs/2.2/tutorial.html#tut_universal).

> A universal variable is a variable whose value is shared **across all instances of fish**, now and in the future – even after a reboot.

You can also run code if such a universal variable changes.
These two features + Dark Mode Daemon allow us to easily theme commandline tools that read the current theme from an environment variable.

First, setup the following Dark Mode Daemon script

```
#!/usr/bin/env fish

set COLOR_MODE $DMD_COLOR_MODE
```

This will sync the `COLOR_MODE` universal variable across all fish shell instances with the OS theme to either `dark` or `light`.
Assuming you are currently in light mode, you can open up a terminal, and run `echo $COLOR_MODE`.
You should see "light" printed to the terminal.
Then switch to dark mode, behind the scenes Dark Mode Daemon adjusts the `COLOR_MODE` variable.
Running `echo $COLOR_MODE` should now print "dark".

Now let's say we want to sync the syntax theme for [`bat`](https://github.com/sharkdp/bat).
`bat` reads the current theme from the `BAT_THEME` environment variable.

Now let's create the following file at `~/.config/fish/conf.d/bat.fish`

```fish
set -Ux BAT_THEME_LIGHT 'Solarized (light)'
set -Ux BAT_THEME_DARK tokyonight_night

if set -qU BAT_THEME
else
    set -Ux BAT_THEME $BAT_THEME_DARK
end

function update_bat_theme --on-variable COLOR_MODE
    if [ $COLOR_MODE = light ]
        set BAT_THEME $BAT_THEME_LIGHT
    else
        set BAT_THEME $BAT_THEME_DARK
    end
end
```

Now every fish shell instance will adjust the `BAT_THEME` environment variable magically when the OS theme changes.

> [!NOTE]  
> Tools like `bat` often have some basic support for adjusting the theme based on the terminal colors.
> However, this is most often an automated process. So while the shade of green might look like the green from your theme, e.g. strings might be colored green, when in the actual theme they are purple.
> If you are unsure, compare the output when having the colors being inferred automatically and when you set them explicitly using `BAT_THEME`.

## [Helix](https://helix-editor.com)

The post-modern text editor has a few issues requesting auto dark/light theme support.
The latest seems to be [#8899](https://github.com/helix-editor/helix/issues/8899) and there seems to be some progress and the intent to implement a solution by the maintainers.
However, at the time of writing, there is no builtin solution, so Dark Mode Daemon can help.

Helix supports reloading the configuration by sending the `US1` signal the Helix process ([docs](Bhttps://docs.helix-editor.com/configuration.html#configuration)).
The following script sets the theme to either `onelight` or `tokyonight`, depending on your OS colorscheme.
Note that this requires `sed` to be available, as well as an existing `theme` configuration being set in your `helix/config.toml`.
It also probably a good idea to put a comment in your `helix/config.toml` indicating that the theme will be automatically adjusted based on a script.

```bash
#!/usr/bin/env bash

CONFIG="$HOME/.config/helix/config.toml"

if [[ "$DMD_COLOR_MODE" = "light" ]]
then
  sed -i.bak 's/^theme = ".*"/theme = "onelight"/' "$CONFIG";
fi

if [[ "$DMD_COLOR_MODE" = "dark" ]]
then
  sed -i.bak 's/^theme = ".*"/theme = "tokyonight"/' "$CONFIG";
fi

rm -f "$CONFIG.bak";

pkill -USR1 hx || true;
```

## `say` / `espeak`

This recipe is not that useful, but fun.
It makes your OS anounce color mode changes using the commonly available Text-To-Speech (TTS) program.
So every time you switch to dark mode, you will hear the works "Switched to dark mode" being uttered by your computer.

```shell
#!/usr/bin/env bash

# MacOS
say "Switched to $DMD_COLOR_MODE mode"

# Linux
espeak "Switched to $DMD_COLOR_MODE mode"
```

