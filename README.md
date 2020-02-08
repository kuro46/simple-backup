# simple-backup

Simple backup tool using rsync.

One day, I using rsync for backup my computer. I wrote simple script for that work.
A little later, I suddenly thought "I have to develop Rust version of this!"

## Usage

### Setting up!

1. Install by follow 'Installation' section
1. Run `simple-backup` (Configuration file is created by do this)
1. Open `$XDG_CONFIG_HOME/simple-backup/config.toml`
1. Fill `target_dir` and `dest_dir`

### Backup!

1. Run `simple-backup`!  
This executes `rsync -avh --delete --backup --backup-dir="<dest_dir>/backup-$(date +%Y%m%d-%H%M%S)" <target_dir> "<dest_dir>/backup-latest"` internally!

## Installation

```
git clone https://github.com/kuro46/simple-backup.git
cd simple-backup
cargo install --path .
```
(If you don't have rsync, install it.)  
After do above, you can run `simple-backup` command.
