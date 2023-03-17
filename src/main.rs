extern crate simple_munin_plugin;

mod wsr {
    use simple_munin_plugin::MuninNodePlugin;
    use ureq;
    use jmespath::{self, Variable};
    use strip_bom::*;

    const FIELD: &'static str = "wsratio";

    pub struct WsratioPlugin;

    impl WsratioPlugin {
        pub fn new() -> WsratioPlugin {
            WsratioPlugin
        }

        pub fn get_wsr() -> f64 {
            return Self::get_vol() / Self::get_max() * 100.0 

        }

        fn get_max() -> f64 {
            let json_url = "https://www.ktr.mlit.go.jp/tonedamu/teikyo/realtime2/json/E015010.json";
            let resp = ureq::get(json_url).call().unwrap();
            let expr = jmespath::compile("waterCapacityList.dataList.*.to_number(validWaterCapacity) | max(@)").unwrap();
            let body_str = resp.into_string().unwrap();
            let body_str :&str = body_str.strip_bom();
            let data = jmespath::Variable::from_json(&body_str).unwrap();
            let result = expr.search(data).unwrap();
            result.as_number().unwrap()
        }
        
        fn get_vol() -> f64 {
            let json_url = "https://www.ktr.mlit.go.jp/tonedamu/teikyo/realtime2/json/E007010.json";
            let resp = ureq::get(json_url).call().unwrap();
            let expr = jmespath::compile("damDataList[?observationName!='５ダム' && observationName!='９ダム'].dataList[0].waterCapacity").unwrap();
            let body_str = resp.into_string().unwrap();
            let body_str :&str = body_str.strip_bom();
            let data = jmespath::Variable::from_json(&body_str).unwrap();
            let result = expr.search(data).unwrap();
            let variable: &Variable = &*result;
            let mut total = 0.0;
            for i in variable.as_array().unwrap() {
                let v = i.to_string().trim_matches('"').to_string().replace(",","").parse::<f64>().unwrap_or(0.0);
                total = total + v ;
            }
            total
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


