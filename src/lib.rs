use quick_xml::events::Event;
use quick_xml::reader::Reader;

use chrono::Datelike;
use clap::ValueEnum;
use dirs;

/// ラジオ放送局のリスト
#[derive(Debug, Clone, ValueEnum)]
pub enum RadioLocation {
    Sapporo,
    Sendai,
    Tokyo,
    Nagoya,
    Osaka,
    Hiroshima,
    Matsuyama,
    Fukuoka,
}

/// ラジオチャンネルのリスト
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum RadioChannel {
    NhkR1,
    NhkFm,
    NhkR2,
}

impl RadioChannel {
    /// チャンネルの名前を取得する
    pub fn as_str(&self) -> &str {
        match self {
            RadioChannel::NhkR1 => "r1hls",
            RadioChannel::NhkR2 => "r2hls",
            RadioChannel::NhkFm => "fmhls",
        }
    }
}

impl RadioLocation {
    pub fn as_str(&self) -> &str {
        match self {
            RadioLocation::Sapporo => "sapporo",
            RadioLocation::Sendai => "sendai",
            RadioLocation::Tokyo => "tokyo",
            RadioLocation::Nagoya => "nagoya",
            RadioLocation::Osaka => "osaka",
            RadioLocation::Hiroshima => "hiroshima",
            RadioLocation::Matsuyama => "matsuyama",
            RadioLocation::Fukuoka => "fukuoka",
        }
    }
}

/// NHKのラジオストリームURL情報を取得してファイルに保存する
fn get_stream_xml_from_server() -> String {
    // NHKのラジオストリームURL情報を取得するためのURL。
    let url_str = "https://www.nhk.or.jp/radio/config/config_web.xml";

    // URLからデータを取得する。この今回はXML形式のデータとわかっているので
    // レスポンスを文字列として取得する。
    // この値は関数の返り値になる。
    reqwest::blocking::get(url_str)
        .expect("It could get the response from the URL")
        .text()
        .expect("It could be converted to text")
}

/// アプリケーション、キャッシュからconfig.xmlを取得する。
/// アプリけションキャッシュは$XDG_CACHE_HOME/nhk-radio-recorder/config.xmlである。
/// XDG_CACHE_HOMEが設定されていない場合は、デフォルトで~/.cache/nhk-radio-recorder/config.xmlを使用する。
/// config.xmlが存在しない場合は、fetch_stream_xml_from_server()を呼び出して取得する。
/// また、キャッシュにconfig.xmlが存在する場合でも、キャッシュの更新月が現在の月と異なる場合は、
/// fetch_stream_xml_from_server()を呼び出して更新する。
fn get_config_xml_from_cache() -> String {
    const CACHE_SUBDIR: &'static str = ".cache";
    const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    const CACHE_FILE_NAME: &'static str = "config.xml";
    // $XDG_CACHE_HOMEが示すパスを取得する。
    let mut xdg_cache_file = match std::env::var("XDG_CACHE_HOME") {
        Ok(path) => std::path::PathBuf::from(path),
        Err(_) => {
            // XDG_CACHE_HOMEが設定されていない場合は、明示的にキャッシュパス ~/.cache を作る。
            let mut path = dirs::home_dir().expect("Home directory must exist.");
            path.push(CACHE_SUBDIR);
            path
        }
    };
    // config.xmlのパスを設定する。
    xdg_cache_file.push(APP_NAME);
    xdg_cache_file.push(CACHE_FILE_NAME);

    // キャッシュにconfig.xmlが存在するか確認する。
    if !xdg_cache_file.exists() {
        // config.xmlが存在しない場合、そもそもキャッシュディレクトリが存在するか確認する。
        if let Some(parent) = xdg_cache_file.parent() {
            // ディレクトリが存在しない場合は、作成する。
            std::fs::create_dir_all(parent).expect("Cache directory could be created");
        }

        // config.xmlが存在しないので、fetch_stream_xml_from_server()を呼び出して取得する。
        let xml_string = get_stream_xml_from_server();
        // 取得したXMLをファイルに保存する。
        std::fs::write(&xdg_cache_file, xml_string).expect("config.xml could be written to cache");
    }
    // ここで、キャッシュの更新月を確認する。
    // まず、現在の月を取得する。
    let current_month = chrono::Local::now().month();
    // 次に、キャッシュの更新月を取得する。
    let metadata = std::fs::metadata(&xdg_cache_file).expect("config.xml metadata could be read");
    let modified_time = metadata
        .modified()
        .expect("config.xml modified time could be read");
    let modified_month = chrono::DateTime::<chrono::Local>::from(modified_time).month();
    // 現在の月とキャッシュの更新月が異なる場合は、fetch_stream_xml_from_server()を呼び出して更新する。
    if current_month != modified_month {
        // fetch_stream_xml_from_server()を呼び出して、最新のXMLを取得する。
        let xml_string = get_stream_xml_from_server();
        // 取得したXMLをファイルに保存する。
        std::fs::write(&xdg_cache_file, xml_string).expect("con");
    }

    // config.xmlを読み込む。
    let xml_string =
        std::fs::read_to_string(&xdg_cache_file).expect("config.xml could be read from cache");
    // 取得したXMLを返す。
    xml_string
}

/// 指定された地域とチャンネルに基づいて、NHKラジオのストリームURLを取得する。
pub fn get_station_url(location: RadioLocation, channel: RadioChannel) -> String {
    // NHKのラジオストリームURL情報を取得して、readerを作成する。
    let xml_string = get_config_xml_from_cache();
    let mut reader = Reader::from_str(&xml_string);

    // XMLを木構造解析するためのスタックとバッファ。
    // contextは、木の中の位置を表す。
    let mut context_stack = Vec::new();
    let mut buf = Vec::new();
    let mut context = "".to_string();

    // URL、エリアなどを保持する変数
    let mut r1_url = "".to_string();
    let mut r2_url = "".to_string();
    let mut fm_url = "".to_string();
    let mut area = "".to_string();

    // 取得したURLを保持する変数。
    let result_url: Option<String>;

    loop {
        // XMLのイベントを読み取る。
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // 開始タグの名前を取得して、現在のコンテキストを更新する。
                context_stack.push(context);
                context = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }

            Ok(Event::Text(e)) => {
                // XMLのテキストノードを取得して、現在のコンテキストに応じて変数に保存する。
                // unescape()は、XMLエスケープ文字を元の文字に戻す。
                let text = e.unescape().unwrap().to_string();
                if context == "area" {
                    area = text;
                }
            }
            Ok(Event::CData(e)) => {
                // CDATAセクションのテキストを取得して、現在のコンテキストに応じて変数に保存する。
                // CDATAは、XMLの特殊文字を含むテキストをそのまま扱うために使用される。
                let text = String::from_utf8_lossy(&e).to_ascii_lowercase().to_string();
                // r1hls, r2hls, fmhlsのいずれかのコンテキストに応じてストリーミングURLを区別して保存する。
                if context == "r1hls" {
                    r1_url = text;
                } else if context == "r2hls" {
                    r2_url = text;
                } else if context == "fmhls" {
                    fm_url = text;
                }
            }
            Ok(Event::End(_)) => {
                // dataタグの終了時に、地域が一致するか確認する。
                if context == "data" {
                    if area == location.as_str().to_string() {
                        // チャンネルに基づくURLを保存する。
                        result_url = match channel.as_str().to_string() {
                            r if r == RadioChannel::NhkR1.as_str() => Some(r1_url),
                            r if r == RadioChannel::NhkR2.as_str() => Some(r2_url),
                            r if r == RadioChannel::NhkFm.as_str() => Some(fm_url),
                            _ => None,
                        };
                        // ループ正常終了
                        break;
                    }
                    // XMLの構造が正しいならば以下のクリーニングは無用。
                    // ただし、何らかの理由で不正なXMLが返された場合に備えて、
                    // 変数をクリアする。
                    r1_url.clear();
                    r2_url.clear();
                    fm_url.clear();
                    area.clear();
                }
                // 木をバックトラックするので、現在のコンテキストを捨ててスタックから
                // 以前のコンテキストを取得する。
                context = context_stack.pop().unwrap_or_default();
            }
            // EOFは、XMLの終端を示す。異常終了。
            Ok(Event::Eof) => {
                panic!("Error: reached end of file before finding the expected end tag.")
            }
            // それ以外のエラーは、XMLの解析に失敗したことを示す。異常終了。
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    // エラーチェックして、結果のURLが取得できなかった場合はパニックを起こす。
    if result_url.is_none() {
        // これは指定されたチャンネルが存在しない場合のエラー。
        panic!("Error: Specified channel error. Could be program logic error.");
    } else if result_url.as_ref().unwrap().is_empty() {
        // これは指定された地域とチャンネルに対してURLが空の場合のエラー。
        panic!("Error: The URL for the specified location is empty");
    } else {
        // 取得したURLを返す。
        result_url.unwrap()
    }
}
