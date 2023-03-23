use std::collections::HashMap;

use simple_munin_plugin::MuninNodePlugin;
use ureq;
use jmespath;
use strip_bom::*;

const FIELD: &'static str = "wsratio";

pub struct WsratioPlugin;

impl WsratioPlugin {
    pub fn new() -> WsratioPlugin {
        WsratioPlugin
    }

    pub fn get_wsr() -> f64 {
        let json_url = "https://www.ktr.mlit.go.jp/tonedamu/teikyo/realtime2/json/E015010.json";
        let resp = ureq::get(json_url).call().unwrap();
        let expr = jmespath::compile("waterCapacityList.dataList.*.to_number(validWaterCapacity) | max(@)").unwrap();
        let body_str = resp.into_string().unwrap();
        let body_str :&str = body_str.strip_bom();
        let data = jmespath::Variable::from_json(&body_str).unwrap();
        let max_capacity = expr.search(data).unwrap().as_number().unwrap();

        let expr = jmespath::compile("waterCapacityList.dataList.*.{name: observationName, vol: nowWaterCapacity}").unwrap();
        let data = jmespath::Variable::from_json(&body_str).unwrap();
        let tmp_result: Vec<HashMap<String, String>> = serde_json::from_str(expr.search(data).unwrap().to_string().as_str()).unwrap();
        let mut tmp_map: HashMap<String, f64> = HashMap::new();
        for d in tmp_result {
            tmp_map.insert(d["name"].to_string(), d["vol"].parse::<f64>().unwrap());
        }
    
        let json_url = "https://www.ktr.mlit.go.jp/tonedamu/teikyo/realtime2/json/E007010.json";
        let resp = ureq::get(json_url).call().unwrap();
        let expr = jmespath::compile("damDataList[?observationName!='５ダム' && observationName!='９ダム'].{name: observationName, cap: dataList[0].waterCapacity}").unwrap();
        let body_str = resp.into_string().unwrap();
        let body_str :&str = body_str.strip_bom();
        let data = jmespath::Variable::from_json(&body_str).unwrap();
        let results: Vec<HashMap<String, String>> = serde_json::from_str(expr.search(data).unwrap().to_string().as_str()).unwrap();
        let mut total = 0.0;
        for i in results {
            let mut v = i["cap"].to_string().trim_matches('"').to_string().replace(",", "").parse::<f64>().unwrap_or(-1.0);
            if v < 0.0 {
                v = tmp_map[&i["name"]];
            }
            total = total + v;
        }

        return total / max_capacity * 100.0 ; 
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

