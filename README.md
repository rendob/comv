# compress-video

動画圧縮用コマンド

- 指定したディレクトリ内の全ての動画に対して、開始 0.3 秒を削った上で圧縮する。
- 結果は指定したディレクトリと同階層の`{input_dir}_dest`内にディレクトリ構造を保って出力する。
- 動画以外のファイルは出力ディレクトリにコピーする。(`.DS_Store`は除外)
- 出力ディレクトリ内に既に対象ファイルが存在する場合、そのファイルの圧縮・コピーはスキップされる。

## Prerequisite

- `ffmpeg`

## Installation

1. 自作コマンド用のディレクトリを作成 (ここでは `~/command` とする)
   ```sh
   mkdir ~/command
   ```
1. パスを通すためのコマンドを`.zshrc`などの設定ファイルに追加
   ```sh
   export PATH="$HOME/command:$PATH"
   ```
1. プロジェクトをビルドし、作成したバイナリを`~/command`配下に置く。
   ```sh
   cargo build --release && cp ./target/release/compress-video ~/command
   ```
