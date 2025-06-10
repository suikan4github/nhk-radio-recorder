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
- FFmpeg 6.1

Unixのディレクトリ構成に依存しているため、Windows環境では動作しないと思われます。MacOSでの動作は未確認です。

## 使い方
使用にあたってはあらかじめユーザーがプログラムのビルドを行う必要があります。ビルドの方法については後で説明します。
### ヘルプ
ヘルプ画面を表示するには、コマンドラインから以下のコマンドを実行します。
```bash
nhk-radio-recorder --help
```
### 録音
録音を開始するには、airchekサブコマンドを使用します。aircheckサブコマンドの使用方法は、ヘルプから確認できます。
```bash
nhk-radio-recorder aircheck --help
```
実行すると以下のヘルプ画面が表示されます。
```
./nhk-radio-recorder aircheck --help
エアチェックを行う

Usage: nhk-radio-recorder aircheck [OPTIONS] --location <LOCATION> --channel <CHANNEL> --title <TITLE>

Options:
  -l, --location <LOCATION>  放送局 [possible values: sapporo, sendai, tokyo, nagoya, osaka, hiroshima, matsuyama, fukuoka]
  -c, --channel <CHANNEL>    チャンネル [possible values: nhk-r1, nhk-fm, nhk-r2]
  -t, --title <TITLE>        番組名
  -d, --duration <DURATION>  番組の長さ[分] [default: 60]
  -h, --help                 Print help
```
`-l`オプションに注意してください。ここは自分が住んでいる地域を指定します。地域は以下のいずれかを指定できます。
- sapporo: 札幌
- sendai: 仙台
- tokyo: 東京
- nagoya: 名古屋
- osaka: 大阪
- hiroshima: 広島
- matsuyama: 松山
- fukuoka: 福岡

`-c`オプションは、チャンネルを指定します。以下のいずれかを指定できます。
- nhk-r1: NHKラジオ第1
- nhk-r2: NHKラジオ第2
- nhk-fm: NHK-FM

`-t`オプションは、番組名を指定します。これは録音したファイルの名前に使用されます。

`-d`オプションは、番組の長さを分単位で指定します。デフォルトは60分です。

録音結果は`.m4a`形式で保存されます。トランスコードはしないため、放送される音声の品質そのままに保存されます。

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
ビルドが成功すると、`target/release/nhk-radio-recorder`に実行可能ファイルが生成されます。このファイルを適当な場所に移動して、コマンドラインから実行できるようにしてください。

## ライセンス
このソフトウェアは、[MITライセンス](./LICENSE)の下で提供されています。