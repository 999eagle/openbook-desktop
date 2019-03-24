use std::collections::HashMap;
use std::sync::Arc;

use super::DecodeError;

use flutter_engine::{
    channel::{Channel, StandardMethodChannel},
    codec::{standard_codec::Value, MethodCallResult},
    FlutterEngineInner, PlatformMessage, Plugin, PluginRegistry, Window,
};
use log::{debug, info, trace};

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
    storage: HashMap<String, String>,
}

impl FlutterSecureStoragePlugin {
    pub fn new() -> Self {
        Self {
            channel: StandardMethodChannel::new(CHANNEL_NAME),
            storage: HashMap::new(),
        }
    }

    fn read(&self, args: &ReadArgs) -> MethodCallResult<Value> {
        trace!("Read key {}", args.key);

        match self.storage.get(args.key) {
            Some(v) => MethodCallResult::Ok(Value::String(v.clone())),
            None => MethodCallResult::Ok(Value::Null),
        }
    }

    fn write(&mut self, args: &WriteArgs) -> MethodCallResult<Value> {
        trace!("Write key {}. New value: {}", args.key, args.value);

        self.storage
            .insert(String::from(args.key), String::from(args.value));

        MethodCallResult::Ok(Value::Null)
    }

    fn delete(&mut self, args: &DeleteArgs) -> MethodCallResult<Value> {
        trace!("Delete key {}", args.key);
        self.storage.remove(args.key);
        MethodCallResult::Ok(Value::Null)
    }

    fn read_all(&self) -> MethodCallResult<Value> {
        trace!("Read all");
        let mut map = HashMap::<Value, Value>::new();
        for (key, value) in self.storage.iter() {
            map.insert(Value::String(key.clone()), Value::String(value.clone()));
        }
        MethodCallResult::Ok(Value::Map(map))
    }

    fn delete_all(&mut self) -> MethodCallResult<Value> {
        trace!("Delete all");
        self.storage.clear();
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
            "read" => {
                let response = match ReadArgs::from_value(&decoded.args) {
                    Ok(args) => self.read(&args),
                    Err(_) => MethodCallResult::Err {
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
                    Err(_) => MethodCallResult::Err {
                        details: Value::Null,
                        code: String::from(""),
                        message: String::from(""),
                    },
                };
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "delete" => {
                let response = match DeleteArgs::from_value(&decoded.args) {
                    Ok(args) => self.delete(&args),
                    Err(_) => MethodCallResult::Err {
                        details: Value::Null,
                        code: String::from(""),
                        message: String::from(""),
                    },
                };
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "readAll" => {
                let response = self.read_all();
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            "deleteAll" => {
                let response = self.delete_all();
                self.channel
                    .send_method_call_response(msg.response_handle, response);
            }
            _ => (),
        }
    }
}
