use lapce_plugin::{
    LapcePlugin, PLUGIN_RPC, psp_types::lsp_types::InitializeParams, register_plugin,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        use lapce_plugin::psp_types::{Request, lsp_types::request::Initialize};

        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("Lapce Json initialize error: {e}"))
                }
            }
            _ => {}
        }
    }
}

fn initialize(params: InitializeParams) -> anyhow::Result<()> {
    use lapce_plugin::psp_types::lsp_types::{DocumentFilter, MessageType, Uri};
    use std::str::FromStr;

    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|e| e.get("serverPath"))
        .and_then(|e| e.as_str())
        .filter(|e| !e.is_empty());

    let server_uri = server_path.and_then(|e| Uri::from_str(&format!("urn:{e}")).ok());
    let server_args = vec!["--stdio".to_string()];
    let document_selector = vec![DocumentFilter {
        language: Some("json".to_string()),
        scheme: None,
        pattern: None,
    }];

    match server_uri {
        Some(server_uri) => {
            let mut opts = params.initialization_options.unwrap_or_default();
            if let Some(obj) = opts.as_object_mut() {
                use serde_json::Value::Bool;
                obj.insert("provideFormatter".to_string(), Bool(true));
            }

            let result =
                PLUGIN_RPC.start_lsp(server_uri, server_args, document_selector, Some(opts));
            if result.is_err() {
                PLUGIN_RPC.stderr(&format!("Lapce Json Failed to start lsp: {result:?}"));
            }
        }
        None => PLUGIN_RPC.window_show_message(
            MessageType::ERROR,
            format!(r#"
                Please configure the vscode-json-language-server path in the config first.
                If its not installed yet, install by `npm install -g vscode-html-languageserver-bin`.
            "#),
        )?,
    }

    Ok(())
}

register_plugin!(State);
