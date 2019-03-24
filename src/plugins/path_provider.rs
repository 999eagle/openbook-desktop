use std::sync::Arc;
use std::{fs, path::PathBuf};

use flutter_engine::{
    channel::{Channel, StandardMethodChannel},
    codec::{standard_codec::Value, MethodCallResult},
    FlutterEngineInner, PlatformMessage, Plugin, PluginRegistry, Window,
};
use log::{debug, info, trace};

const CHANNEL_NAME: &str = "plugins.flutter.io/path_provider";

pub struct PathProviderPlugin {
    channel: StandardMethodChannel,
}

impl PathProviderPlugin {
    pub fn new() -> Self {
        Self {
            channel: StandardMethodChannel::new(CHANNEL_NAME),
        }
    }

    fn get_directory_result(&self, dir: Option<PathBuf>, subdir: bool) -> MethodCallResult<Value> {
        match dir {
            Some(dir) => {
                let dir = if subdir { dir.join("openbook") } else { dir };
                match fs::create_dir_all(&dir) {
                    Ok(_) => {
                        MethodCallResult::Ok(Value::String(dir.to_string_lossy().into_owned()))
                    }
                    Err(_) => MethodCallResult::Err {
                        details: Value::Null,
                        code: String::from(""),
                        message: String::from(""),
                    },
                }
            }
            None => MethodCallResult::Err {
                details: Value::Null,
                code: String::from(""),
                message: String::from(""),
            },
        }
    }

    fn get_temporary_directory(&self) -> MethodCallResult<Value> {
        self.get_directory_result(dirs::cache_dir(), true)
    }

    fn get_application_documents_directory(&self) -> MethodCallResult<Value> {
        self.get_directory_result(dirs::data_dir(), true)
    }

    fn get_storage_directory(&self) -> MethodCallResult<Value> {
        self.get_directory_result(dirs::home_dir(), false)
    }
}

impl Plugin for PathProviderPlugin {
    fn init_channel(&self, registry: &PluginRegistry) -> &str {
        self.channel.init(registry);
        CHANNEL_NAME
    }

    fn handle(
        &mut self,
        msg: &PlatformMessage,
        _engine: Arc<FlutterEngineInner>,
        _window: &mut Window,
    ) {
        let decoded = self.channel.decode_method_call(msg);

        debug!(
            "Got method call {} with args: {}",
            decoded.method,
            super::debug_print_args(&decoded.args)
        );
        match decoded.method.as_str() {
            "getTemporaryDirectory" => {
                let response = self.get_temporary_directory();
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "getApplicationDocumentsDirectory" => {
                let response = self.get_application_documents_directory();
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "getStorageDirectory" => {
                let response = self.get_storage_directory();
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            _ => (),
        }
    }
}
