use lapce_plugin::{
    LapcePlugin, PLUGIN_RPC,
    psp_types::{
        Request,
        lsp_types::{InitializeParams, request::Initialize},
    },
    register_plugin,
};
use serde_json::Value;

register_plugin!(State);

#[derive(Default)]
struct State {}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}

fn initialize(params: InitializeParams) -> anyhow::Result<()> {
    Ok(())
}
