# twilight-commander

[![Build Status](https://travis-ci.org/golmman/twilight-commander.svg?branch=master)](https://travis-ci.org/golmman/twilight-commander)

A simple console tree file explorer for linux, similiar to NERDTree but independent of vim.
Developed and tested on Ubuntu 18.04 with xterm derivatives.

![Screenshot](https://github.com/golmman/twilight-commander/blob/master/screenshot.png "Screenshot")

## Build and install

### Instructions for Debian 10 or Ubuntu 18.04

| |step|description|
|---|---|---|
|1.|install rust|https://www.rust-lang.org/tools/install|
|2.|clone the repository|`git clone https://github.com/golmman/twilight-commander.git`|
|3.|change to the newly created directory|`cd twilight-commander`|
|4.|build the project|`cargo build --release`|
|5.|run the executable|`./target/release/twilight-commander`|

## Implemented features

### Configuration
The configuration is loaded as follows
1. load values from `$XDG_CONFIG_HOME/twilight-commander.toml`
2. else load values from `$HOME/.config/twilight-commander/twilight-commander.toml`
2. fill missing values with app defaults
3. overwrite values with defines from the command line options

For a config file with the default values, see [twilight-commander.toml](./twilight-commander.toml).
The command line options are derived from the values defined inside the twilight-commander.toml .
E.g.
```
[debug]
enabled = true
```
is set with the option `--debug.enabled=true`.

### Configurable key bindings

The key bindings are configurable. For the set of configurable keys and key combinations consult the [event.rs](./src/model/event.rs).

|default key|default configuration|action|
|---|---|---|
|up arrow|`keybinding.entry_up=up`|move an entry up|
|down arrow|`keybinding.entry_down=down`|move an entry down|
|left arrow|`keybinding.collapse_dir=left`|collapse an entry directory|
|right arrow|`keybinding.expand_dir=left`|expand an entry directory|
|r|`keybinding.reload=r`|collapse all directories and reload root directory|
|return|`keybinding.file_action=return`|perform configured file action|
|q|`keybinding.quit=q`|quit|

### Directory entry management

#### File Action
The command line option / config value `--behavior.file_action` defines the action taken when the return key is pressed
on a file. The action is interpreted by `bash` and any occurence of `%s` will be replaced by the selected filename.  
E.g. when enter is pressed on the file `.bashrc` in a twilight-commander process created with
```
twilight-commander "--behavior.file_action=xterm -e 'cat %s; echo opened file: %s; bash'"
```
then
```
bash -c "xterm -e 'cat /home/user/.bashrc; echo opened file: /home/user/.bashrc; bash'"
```
is executed, i.e.:
* a new xterm window is opened
* where the selected file (`.bashrc`) is printed to stdout
* then `opened file: ~/.bashrc` is printed
* `bash` prevents the window from closing.

`--behavior.file_action` defaults to [true](https://en.wikipedia.org/wiki/True_and_false_(commands)), which does
(almost) nothing.

### Scrolling modes
Specified with the option `--behaviour.scrolling` (default = `center`)

* `center`: move the cursor until it is in the center, then move the text instead
* `editor`: move the cursor until it hits the top/bottom boundaries set by the `debug.paddin_top/bot` limits

### Utf-8 support
In case your terminal does not support utf-8 you can disable it with `--composition.use_utf8=false`.

## Ideas for improvements
* **advanced navigation**
  * jump to parent directory
  * skip x entries by holding a modifier key
  * collapse the current parent directory
* add a proper Makefile
  * https://sagiegurari.github.io/cargo-make/
* better response to terminal resize events: in some terminals response is too slow, text is wrapped
  * intended to work like `less -S <filename>`
  * problem seems not to appear in plain xterm
  * https://www.xfree86.org/4.8.0/ctlseqs.html
  * https://invisible-island.net/ncurses/man/resizeterm.3x.html
  * https://linux.die.net/man/1/resize
  * https://stackoverflow.com/questions/4738803/resize-terminal-and-scrolling-problem-with-ncurses#4739108
* recursive reload
* more colors, configurable
* directory entry stats
* directory entry management
  * copy
  * create directory
  * create file
  * move
  * ~~open with custom command~~
  * remove
  * rename
* subdirectory caching
* improved sorting
* --help screen with info to all command line options
  * [clap](https://crates.io/crates/clap)
  * [gumdrop](https://crates.io/crates/gumdrop)
* bookmark / pin entries (recursivly?) and prevent them from being collapsed
* search
  * case insensitive wildcard
  * mark hits
