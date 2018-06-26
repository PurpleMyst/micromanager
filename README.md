# µmanager

`µmanager` is a simple little package manager, written in Rust.

It's so micro that with no config, it doesn't know how to download anything.
Thankfully, the default config has at least `git` support.

## Installation

To install, type at your command line:

```shell
$ cargo install --git https://github.com/PurpleMyst/micromanager
...
```

## Usage

After installation, you should have a `µmanager` binary. Running it the first
time will set up all the directories et cetera.

Afterwards, you should have a `config.toml` file somewhere (On Linux, this will
be in `~/.config/µmanager/`), and it should look a lil' bit like this:

```toml
[sources.git]
package_directory = "{key}"

[sources.git.commands]
download = "git clone {location} {package_directory}"
update = "git pull"

################################################################################

[packages]
```

You should fill the file after the packages section to your liking. For a taste
of the format, you can read [`sample-config.toml`].
