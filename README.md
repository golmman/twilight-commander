# twilight-commander
A simple console tree file explorer for linux, similiar to NERDTree but independent of vim.
Tested in Ubuntu 18.04.

## Implemented features

### directory entry browsing
* up, down: move cursor
* left, right: close, open directories
* return: perform behavior.file_action
* q: quit

### configuration
The configuration is loaded as follows
1. load values from ~/.twilight-commander-rc.toml
2. fill missing values with app defaults
3. overwrite values with defines from the command line options

For a config file with the default values, see [.twilight-commander-rc.toml](./.twilight-commander-rc.toml).
The command line options are derived from the values defined inside the .twilight-commander-rc.toml .
E.g.
```
[debug]
enabled = true
```
is set with the option `--debug.enabled=true`.

### directory entry management
The command line option / config value `--behavior.file_action` defines the action taken when the return key is pressed on a file. It defaults to [true](https://en.wikipedia.org/wiki/True_and_false_(commands)), which does (almost) nothing.

### Scrolling modes
Specified with the option `--behaviour.scrolling` (default = `center`)

* `center`: move the cursor until it is in the center, then move the text instead
* `editor`: move the cursor until it hits the top/bottom boundaries set by the `debug.paddin_top/bot` limits

## Upcoming improvements
* **reload**
* **shorten entries which are too long**
* **configurable key bindings**
* **indicator for empty directories**
* **advanced navigation**
  * jump to parent directory
  * skip x entries by holding a modifier key
  * collapse the current parent directory
* **respond to terminal resize events (SIGWINCH, see POSIX signals)**
* more colors, configurable
* utf8 support
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
* --help screen with info to all cmd line options
* bookmark / pin entries (recursivly?) and prevent them from being collapsed
* search
  * case insensitive wildcard
  * mark hits

