extern crate simple_munin_plugin;

mod wsr {
    use simple_munin_plugin::MuninNodePlugin;
    use ureq;
    use jq_rs;
    use serde_json;

    const FIELD: &'static str = "wsratio";

    pub struct WsratioPlugin;

    impl WsratioPlugin {
        pub fn new() -> WsratioPlugin {
            WsratioPlugin
        }

        pub fn get_wsr() -> f32 {
            let dam_json_url = "https://www.ktr.mlit.go.jp/tonedamu/teikyo/realtime2/json/E007010.json";
        
            let resp = ureq::get(dam_json_url).call().unwrap();
            let body = resp.into_string().unwrap();
            let res = jq_rs::run(".damDataList[-1].dataList[0].waterRate", &body).unwrap();
            let wsr: f32 = serde_json::from_str(&res).unwrap();
            return wsr

        }
    }

    impl MuninNodePlugin for WsratioPlugin {
        fn config(&self) {
            println!(r#"graph_title Tonegawa DAM Water Storage Ratio
graph_args -l 0
graph_vlabel %
graph_category environment
{0}.label wsratio"#, FIELD);
        }

        fn run(&self) {
            //dbg!(wsr);
            println!("{}.value {}", FIELD, Self::get_wsr());
        }
        
    }
}

use simple_munin_plugin::MuninNodePlugin;

fn main() {
    let plugin = wsr::WsratioPlugin::new();
    std::process::exit(plugin.start());
}


