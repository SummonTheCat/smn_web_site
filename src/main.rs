use std::sync::Arc;

use smn_web_core::{plugins::plugin_static::PluginStatic, systems::{sys_core::run_server, sys_plugin::PluginManager}};


#[tokio::main]
async fn main() {
    // Create and initialize PluginManager.
    let mut manager = PluginManager::new();
    manager.apply_plugin(Box::new(PluginStatic::new(true)));
    manager.init_plugins().await;
    let manager = Arc::new(manager);

    // Run the server on port 33310.
    run_server(33310, manager).await;
}
