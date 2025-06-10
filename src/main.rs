// NHKラジオのリアルタイム・ストリーミングを保存する
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete;
use nhk_radio_recorder::{RadioChannel, RadioLocation};
use std::process::Command;

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

        /// 番組名
        #[arg(long, short)]
        title: String,

        /// 番組の長さ[分]
        #[arg(long, short, default_value = "60")]
        duration: u32,
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
        Commands::Aircheck {
            location,
            channel,
            title: program,
            duration,
        } => {
            // 地域とチャンネルから適切なストリームURLを取得する。
            let station_url = nhk_radio_recorder::get_station_url(location, channel);

            // ffmpegコマンドにこのストリームURLを渡して、指定された番組名と長さでエアチェックを行う。
            let mut child = Command::new("ffmpeg")
                .arg("-i")
                .arg(station_url)
                .arg("-c")
                .arg("copy") // トランスエンコードせずにTSの中のデータをコピーする
                .arg("-t")
                .arg(format!("{}:00", duration)) // durationは分単位なので、秒に変換する必要がある
                .arg(format!("{}.m4a", program)) // 出力ファイル名を指定する
                .arg("-loglevel")
                .arg("error") // エラーメッセージのみを表示する
                .stderr(std::process::Stdio::from(
                    // 標準エラー出力をファイルにリダイレクトする
                    std::fs::File::create(format!("{}.stderr.log", program)).unwrap(),
                ))
                .stdout(std::process::Stdio::from(
                    // 標準出力をファイルにリダイレクトする
                    std::fs::File::create(format!("{}.stdout.log", program)).unwrap(),
                ))
                .spawn()
                .expect("Command::new() should start successfully");
            // サブプロセスの終了を待つ
            let _ = child.wait().expect("child.wait() should succeed");
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
