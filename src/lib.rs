// NHKラジオのリアルタイム・ストリーミングを保存する
use clap::ValueEnum;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use url::Url;

/// ラジオ放送局のリスト
#[derive(Debug, Clone, ValueEnum)]
pub enum RadioStation {
    NhkR1Sapporo,
    NhkR1Sendai,
    NhkR1Tokyo,
    NhkR1Nagoya,
    NhkR1Osaka,
    NhkR1Hiroshima,
    NhkR1Matsuyama,
    NhkR1Fukuoka,
    NhkFmSapporo,
    NhkFmSendai,
    NhkFmTokyo,
    NhkFmNagoya,
    NhkFmOsaka,
    NhkFmHiroshima,
    NhkFmMatsuyama,
    NhkFmFukuoka,
    NhkR2,
}

/// ラジオ局に対応するURLを取得する関数
/// 以下を参照 http://www.nhk.or.jp/radio/config/config_web.xml
pub fn get_station_url(station: RadioStation) -> &'static str {
    match station {
        RadioStation::NhkR1Sapporo => {
            "https://radio-stream.nhk.jp/hls/live/2023545/nhkradiruikr1/master.m3u8"
        }
        RadioStation::NhkR1Sendai => {
            "https://radio-stream.nhk.jp/hls/live/2023543/nhkradiruhkr1/master.m3u8"
        }
        RadioStation::NhkR1Tokyo => {
            "https://radio-stream.nhk.jp/hls/live/2023229/nhkradiruakr1/master.m3u8"
        }
        RadioStation::NhkR1Nagoya => {
            "https://radio-stream.nhk.jp/hls/live/2023510/nhkradiruckr1/master.m3u8"
        }
        RadioStation::NhkR1Osaka => {
            "https://radio-stream.nhk.jp/hls/live/2023508/nhkradirubkr1/master.m3u8"
        }
        RadioStation::NhkR1Hiroshima => {
            "https://radio-stream.nhk.jp/hls/live/2023512/nhkradirufkr1/master.m3u8"
        }
        RadioStation::NhkR1Matsuyama => {
            "https://radio-stream.nhk.jp/hls/live/2023547/nhkradiruzkr1/master.m3u8"
        }
        RadioStation::NhkR1Fukuoka => {
            "https://radio-stream.nhk.jp/hls/live/2023541/nhkradirulkr1/master.m3u8"
        }
        RadioStation::NhkFmSapporo => {
            "https://radio-stream.nhk.jp/hls/live/2023546/nhkradiruikfm/master.m3u8"
        }
        RadioStation::NhkFmSendai => {
            "https://radio-stream.nhk.jp/hls/live/2023544/nhkradiruhkfm/master.m3u8"
        }
        RadioStation::NhkFmTokyo => {
            "https://radio-stream.nhk.jp/hls/live/2023507/nhkradiruakfm/master.m3u8"
        }
        RadioStation::NhkFmNagoya => {
            "https://radio-stream.nhk.jp/hls/live/2023511/nhkradiruckfm/master.m3u8"
        }
        RadioStation::NhkFmOsaka => {
            "https://radio-stream.nhk.jp/hls/live/2023509/nhkradirubkfm/master.m3u8"
        }
        RadioStation::NhkFmHiroshima => {
            "https://radio-stream.nhk.jp/hls/live/2023513/nhkradirufkfm/master.m3u8"
        }
        RadioStation::NhkFmMatsuyama => {
            "https://radio-stream.nhk.jp/hls/live/2023548/nhkradiruzkfm/master.m3u8"
        }
        RadioStation::NhkFmFukuoka => {
            "https://radio-stream.nhk.jp/hls/live/2023542/nhkradirulkfm/master.m3u8"
        }
        RadioStation::NhkR2 => {
            "https://radio-stream.nhk.jp/hls/live/2023501/nhkradiruakr2/master.m3u8"
        }
    }
}

/// NHKのラジオストリームURL情報を取得してファイルに保存する
pub fn fetch_stream_xml() -> Result<(), Box<dyn std::error::Error>> {
    // NHKのラジオストリームURL情報を取得するためのURL。
    let url_str = "https://www.nhk.or.jp/radio/config/config_web.xml";
    let output_file_name = "config_web.xml";

    // ストリーム情報を格納するファイル名。
    let url = Url::parse(url_str).expect("It were a correct URL of the collection of stream URLs");

    // HTTPクライアントを作成
    let client = Client::new();

    // URLからデータを取得する。
    let mut response = client
        .get(url)
        .send()
        .expect("It had a working network connection");

    // レスポンスを格納するファイルを作成する。
    let mut file = File::create(output_file_name).expect("It could create the output file");

    // レスポンスの内容をファイルに書き込む。
    copy(&mut response.text()?.as_bytes(), &mut file)
        .expect("It could copy the response to the file");

    Ok(())
}
