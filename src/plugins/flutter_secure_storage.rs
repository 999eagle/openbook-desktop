use std::sync::Arc;

use super::DecodeError;

use flutter_engine::{
    channel::{Channel, StandardMethodChannel},
    codec::{standard_codec::Value, MethodCallResult},
    FlutterEngineInner, PlatformMessage, Plugin, PluginRegistry, Window,
};
use log::{debug, info};

const CHANNEL_NAME: &str = "plugins.it_nomads.com/flutter_secure_storage";

plugin_args! {
    ReadArgs,
    key: str, "key", Value::String(v) => v.as_str();
}
plugin_args! {
    WriteArgs,
    key: str, "key", Value::String(v) => v.as_str();
    value: str, "value", Value::String(v) => v.as_str();
}
plugin_args! {
    DeleteArgs,
    key: str, "key", Value::String(v) => v.as_str();
}

pub struct FlutterSecureStoragePlugin {
    channel: StandardMethodChannel,
}

impl FlutterSecureStoragePlugin {
    pub fn new() -> Self {
        Self {
            channel: StandardMethodChannel::new(CHANNEL_NAME),
        }
    }

    fn read(&self, args: &ReadArgs) -> MethodCallResult<Value> {
        info!("Trying to read key {}", args.key);

        MethodCallResult::Ok(Value::Null)
    }

    fn write(&self, args: &WriteArgs) -> MethodCallResult<Value> {
        info!(
            "Trying to write key {}. New value: {}",
            args.key, args.value
        );

        MethodCallResult::Ok(Value::Null)
    }
}

impl Plugin for FlutterSecureStoragePlugin {
    fn init_channel(&self, registry: &PluginRegistry) -> &str {
        self.channel.init(registry);
        CHANNEL_NAME
    }

    fn handle(
        &mut self,
        msg: &PlatformMessage,
        engine: Arc<FlutterEngineInner>,
        _window: &mut Window,
    ) {
        let decoded = self.channel.decode_method_call(msg);

        debug!(
            "Got method call {} with args: {}",
            decoded.method,
            super::debug_print_args(&decoded.args)
        );
        match decoded.method.as_str() {
            "read" => {
                let response = match ReadArgs::from_value(&decoded.args) {
                    Ok(args) => self.read(&args),
                    Err(error) => MethodCallResult::Err {
                        details: Value::Null,
                        code: String::from(""),
                        message: String::from(""),
                    },
                };
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "write" => {
                let response = match WriteArgs::from_value(&decoded.args) {
                    Ok(args) => self.write(&args),
                    Err(error) => MethodCallResult::Err {
                        details: Value::Null,
                        code: String::from(""),
                        message: String::from(""),
                    },
                };
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "delete" => {}
            "readAll" => {}
            "writeAll" => {}
            _ => (),
        }
    }
}
