# Send-via-KDE-connect-nautilus

Send files via KDE connect using Nautilus.

## Usage

Place the compiled binary in `$HOME/.local/share/nautilus/scripts/`

## Installation

Download the precompiled binary in the releases section

OR compile it yourself

```
git clone https://github.com/amblessedezejim/Send-via-KDE-connect-nautilus
cd Send-via-KDE-connect-nautilus
cargo build --release
mv target/release/send_to_kdeconnect_nautilus "$HOME/.local/share/nautilus/scripts/Send via KDE Connect"
```
