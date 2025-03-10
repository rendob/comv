# comv

動画圧縮用コマンド

```
% comv -h
compress videos in a directory.

Usage: comv [OPTIONS] <INPUT_DIR>

Arguments:
  <INPUT_DIR>  path of the directory that contains uncompressed videos

Options:
  -r, --recursive  whether to compress videos in descendant directories
  -h, --help       Print help
```

- 指定したディレクトリ内の動画を、開始 0.3 秒を削った上で圧縮する。
  - `--recursive` (`-r`) オプションあり → 子孫ディレクトリのファイルも対象になる。
  - `--recursive` オプションなし → 入力ディレクトリ直下のファイルのみが対象。
- 出力ディレクトリ
  - `--recursive` オプションあり → 入力ディレクトリと同階層の `{input_dir}_dest`
  - `--recursive` オプションなし → 入力ディレクトリ直下の `_dest`
- 動画以外のファイルは出力ディレクトリにコピーする。(`.DS_Store` は除外)
- 出力ディレクトリ内に既に同名ファイルが存在する場合、そのファイルの圧縮・コピーはスキップされる。

## Prerequisite

- `ffmpeg`

## Installation

- Install `comv` to `~/.cargo/bin/`

```sh
git clone https://github.com/rendob/comv
cd comv
cargo install --path .
```

or

```sh
cargo install --git https://github.com/rendob/comv
```
