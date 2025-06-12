// NHKラジオのリアルタイム・ストリーミングURLを標準出力に出力する。
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete;
use get_nhk_radio_url::{RadioChannel, RadioLocation};

/// コマンドライン引数の定義
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// エアチェックを行う
    Aircheck {
        /// 放送局
        #[arg(value_enum, long, short)]
        location: RadioLocation,

        /// チャンネル
        #[arg(value_enum, long, short)]
        channel: RadioChannel,
    },
    /// コマンドライン補完スクリプトを生成
    Completion {
        /// 補完の種類
        #[arg(value_enum, long, short)]
        shell: clap_complete::Shell,
    },
}

/// 引数を解析するための構造体
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// コマンドライン引数
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    // コマンドライン引数を解析
    let cli = Cli::parse();

    // コマンドライン引数の解析
    match cli.command {
        // エアチェックを行う
        Commands::Aircheck { location, channel } =>
        // 地域とチャンネルから適切なストリームURLを取得する。
        {
            // ストリームURLを出力する
            println!("{}", get_nhk_radio_url::get_station_url(location, channel));
        }

        // コマンドライン補完スクリプトを生成する
        Commands::Completion { shell } => {
            // コマンドライン引数の体系をclapが解析するための構造体を生成する。
            let mut cmd = Cli::command();
            // 補完用のshellスクリプトを生成する。
            clap_complete::generate(
                shell,
                &mut cmd,
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
    }
}
