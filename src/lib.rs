use quick_xml::events::Event;
use quick_xml::reader::Reader;

use clap::ValueEnum;

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
#[derive(Debug, Clone, ValueEnum)]
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
pub fn fetch_stream_xml() -> String {
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

pub fn get_station_url(location: RadioLocation, channel: RadioChannel) -> String {
    let mut context_stack = Vec::new();
    let xml_string = fetch_stream_xml();
    let mut reader = Reader::from_str(&xml_string);
    let mut buf = Vec::new();

    let mut context = "".to_string();
    let mut r1_url = "".to_string();
    let mut r2_url = "".to_string();
    let mut fm_url = "".to_string();
    let mut area = "".to_string();
    let result: String;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                context_stack.push(context.clone());
                context = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }

            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap().to_string();
                if context == "areajp" {
                    println!("地域: {}", text);
                } else if context == "area" {
                    area = text;
                }
            }
            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(&e).to_ascii_lowercase().to_string();
                if context == "r1hls" {
                    r1_url = text;
                } else if context == "r2hls" {
                    r2_url = text;
                } else if context == "fmhls" {
                    fm_url = text;
                }
            }
            Ok(Event::End(_)) => {
                if context == "data" {
                    if area == location.as_str().to_string() {
                        result = match channel.as_str().to_string() {
                            r if r == RadioChannel::NhkR1.as_str() => r1_url.clone(),
                            r if r == RadioChannel::NhkR2.as_str() => r2_url.clone(),
                            r if r == RadioChannel::NhkFm.as_str() => fm_url.clone(),
                            _ => "".to_string(),
                        };
                        break;
                    }
                    r1_url.clear();
                    r2_url.clear();
                    fm_url.clear();
                    area.clear();
                }
                context = context_stack.pop().unwrap_or_default();
            }
            Ok(Event::Eof) => {
                panic!("Error: reached end of file before finding the expected end tag.")
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    result
}
