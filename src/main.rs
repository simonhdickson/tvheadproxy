use std::env;
use std::error::Error;

use handlebars::Handlebars;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;
use structopt::StructOpt;
use url::Url;
use warp::{self, path, Filter};

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "tvheadproxy", about = "TV Headend Proxy")]
struct Opt {
    #[structopt(short = "h", env = "TVH_PROXY_URL", long = "tvh_proxy_url")]
    tvh_proxy_url: String,

    #[structopt(short = "t", env = "TVH_URL", long = "tvh_url")]
    tvh_url: String,

    #[structopt(short = "u", env = "TVH_USER", long = "tvh_user")]
    tvh_user: String,

    #[structopt(short = "p", env = "TVH_PASS", long = "tvh_pass")]
    tvh_pass: Option<String>,

    #[structopt(
        short = "c",
        env = "TUNER_COUNT",
        long = "tuner_count",
        default_value = "3"
    )]
    tuners: i32,

    #[structopt(
        short = "s",
        env = "STREAM_PROFILE",
        long = "stream_profile",
        default_value = "pass"
    )]
    stream_profile: String,

    #[structopt(
        short = "w",
        env = "TV_WEIGHT",
        long = "tv_weight",
        default_value = "300"
    )]
    tv_weight: i32,

    #[structopt(
        short = "o",
        env = "PORT",
        long = "port",
        default_value = "5004"
    )]
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelResponse {
    entries: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    uuid: String,
    name: String,
    number: i32,
    enabled: bool,
}

const TEMPLATE_XML: &str = "<root xmlns=\"urn:schemas-upnp-org:device-1-0\">
    <specVersion>
        <major>1</major>
        <minor>0</minor>
    </specVersion>
    <URLBase>{{ BaseURL }}</URLBase>
    <device>
        <deviceType>urn:schemas-upnp-org:device:MediaServer:1</deviceType>
        <friendlyName>{{ FriendlyName }}</friendlyName>
        <manufacturer>{{ Manufacturer }}</manufacturer>
        <modelName>{{ ModelNumber }}</modelName>
        <modelNumber>{{ ModelNumber }}</modelNumber>
        <serialNumber></serialNumber>
        <UDN>uuid:{{ DeviceID }}</UDN>
    </device>
</root>";

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "tvheadproxy=info");
    }
    pretty_env_logger::init();
    
    let opt = Opt::from_args();

    debug!("using config: {:?}", opt);

    let discover_data = json!({
        "FriendlyName": "tvheadproxy",
        "Manufacturer" : "Silicondust",
        "ModelNumber": "HDTC-2US",
        "FirmwareName": "hdhomeruntc_atsc",
        "TunerCount": &opt.tuners,
        "FirmwareVersion": "20150826",
        "DeviceID": "12345678",
        "DeviceAuth": "test1234",
        "BaseURL": &opt.tvh_proxy_url.to_owned(),
        "LineupURL": format!("{}/lineup.json", &opt.tvh_proxy_url)
    });

    let discover_json = {
        let discover_json = discover_data.to_string();

        path!("discover.json").map(move || discover_json.to_owned())
    };

    let discover_xml = {
        let mut hb = Handlebars::new();

        hb.register_template_string("template.xml", TEMPLATE_XML)
            .unwrap();

        let discover_xml = hb
            .render("template.xml", &discover_data)
            .unwrap_or_else(|err| err.description().to_owned());

        path!("discover.xml").map(move || discover_xml.to_owned())
    };

    let lineup_status = path!("lineup_status.json").map(|| {
        json!({
            "ScanInProgress": 0,
            "ScanPossible": 1,
            "Source": "Cable",
            "SourceList": ["Cable"]
        })
        .to_string()
    });

    let client = reqwest::Client::new();

    let mut tvh_url = Url::parse(&opt.tvh_url).unwrap();
    tvh_url.set_username(&opt.tvh_user).unwrap();
    tvh_url
        .set_password(opt.tvh_pass.as_ref().map(|x| &**x))
        .unwrap();
    let tvh_url = tvh_url.to_string();
    let proxy_port = opt.port;

    let lineup_json = path!("lineup.json").map(move || {
        let res: ChannelResponse = client.get(&format!(
            "{}/api/channel/grid?start=0&limit=999999",
            &opt.tvh_url
        ))
        .basic_auth(&opt.tvh_user, opt.tvh_pass.clone())
        .send()
        .unwrap()
        .json()
        .unwrap();

        json!(res
            .entries
            .into_iter()
            .filter(|i| i.enabled)
            .map(|i| {
                json!({
                    "GuideNumber": i.number.to_string(),
                    "GuideName": i.name,
                    "URL": format!("{}stream/channel/{}?profile={}&weight={}", tvh_url, i.uuid, &opt.stream_profile, &opt.tv_weight)
                })
            })
            .collect::<Vec<_>>())
        .to_string()
    });

    let lineup_post = path!("lineup.post").map(|| "".to_owned());

    let routes = discover_json.or(discover_xml).or(lineup_status).or(lineup_json).or(lineup_post);

    warp::serve(routes.with(warp::log("tvheadproxy"))).run(([0, 0, 0, 0], proxy_port));
}
