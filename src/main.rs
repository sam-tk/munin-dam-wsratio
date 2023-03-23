
mod wsr;

use simple_munin_plugin::MuninNodePlugin;

fn main() {
    let plugin = wsr::WsratioPlugin::new();
    std::process::exit(plugin.start());
}


