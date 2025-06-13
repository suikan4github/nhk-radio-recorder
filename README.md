# nhk-radio-recorder
NHKらじるらじるのリアルタイム・ストリーミングをファイルに保存します。

## 詳細
nhk-radio-recorderは、NHKらじるらじるのリアルタイム・ストリーミングを録音するためのコマンドラインツールです。録音した音声は、M4A形式で保存されます。

このツールの特徴として、NHKが提供するリアルタイム・ストリーミングのURLをサーバーから取得している事が挙げられます。これにより、NHKがストリーミングのURLを変更した場合でも、ツールを更新することで対応できます。URL情報はローカルのキャッシュに保存し、毎月一度更新します。

録音可能な番組は、NHKらじるらじるによって提供されているリアルタイム・ストリーミングの番組です。具体的には、NHKラジオ第1、NHKラジオ第2、NHK-FMの各チャンネルの番組が対象となります。
聴き逃し番組や、過去の番組の録音には対応していません。

## 動作環境

以下の環境での動作とビルドを確認しています。
- Linux
  - Ubuntu 24.04 LTS (WSL2)
  - Debian 12
- Rustc 1.86.0
- Gstreamer 1.24.2
- FFmpeg 6.1


Unixのディレクトリ構成に依存しているため、Windows環境では動作しないと思われます。MacOSでの動作は未確認です。

## 使い方
使用にあたってはあらかじめユーザーがプログラムのビルドとインストールを必要があります。それぞれの方法については後で説明します。
### ヘルプ
ヘルプ画面を表示するには、nhk-radio-recorderコマンドを引数なしで実行します。
```bash
$ ./nhk-radio-recorder
Usage: ./nhk-radio-recorder <location> <channel> <title> <duration> [mailaddress]
  <location>    : sapporo, sendai, tokyo, nagoya, osaka, hiroshima, matsuyama, fukuoka
  <channel>     : nhk-r1, nhk-fm, nhk-r2
  <title>       : The title of the program
  <duration>    : The duration of the program in minutes
  [mailaddress] : An optional email address to send the recording to
```
### 録音
録音を行うには、地域、チャンネル、タイトル、番組の長さ[分]を指定します。
```bash
nhk-radio-recorder tokyo nhk-r1 "News at 7" 60
```
録音結果は`.m4a`形式で保存されます。トランスコードはしないため、放送される音声の品質そのままに保存されます。

なお、コマンドの末尾にメールアドレスを指定すると、m4aファイルを指定アドレスに送ります。
```bash
nhk-radio-recorder tokyo nhk-r1 "News at 7" 60 foo@example.com
```
メールで送る場合、生成したm4fファイルは自動的に削除されます。

### シェル補完
シェル補完を有効にするには、以下のコマンドを実行します。
```bash
nhk-radio-recorder completion -s bash > /etc/bash_completion.d/nhk-radio-recorder
```
シェルを再起動すると、補完が有効になります。
## ビルド方法
最初に必要なパッケージをインストールします。
```sh
sudo apt update && sudo apt upgrade
sudo apt install git curl build-essential libssl-dev pkg-config ffmpeg
```
Rustのインストールには、以下のコマンドを実行してください。
```sh
curl https://sh.rustup.rs -sSf | sh
```
次に、以下のコマンドを実行して、本プロジェクトのソースコードを取得します。
```sh
git clone https://github.com/suikan4github/nhk-radio-recorder.git
cd nhk-radio-recorder
```
ビルドには以下のコマンドを実行します。
```sh
```bash
cargo build
```
ビルドが成功すると、`target/debug/get-nhk-radio-livestream-url`として実行可能ファイルが生成されます。

## インストール方法
ビルドが成功したら`target/debug/get-nhk-radio-livestream-url`を適当な場所に移動して、コマンドラインから実行できるようにします。
移動先はパスの通ったディレクトリが望ましいです。もし、パスの通ったディレクトリに移動できない場合は、~/.local/binに移動することをお勧めします。

```bash
mkdir -p ~/.local/bin
cargo build --release
mv target/release/get-nhk-radio-livestream-url ~/.local/bin/
```
~/.local/binがパスに通っていなくても構いません。`nhk-radio-recorder`スクリプトの中でパスを通しています。

## ライセンス
このソフトウェアは、[MITライセンス](./LICENSE)の下で提供されています。