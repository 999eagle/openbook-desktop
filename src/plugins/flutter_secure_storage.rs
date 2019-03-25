use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

use self::crypto::Crypto;
use super::DecodeError;

use flutter_engine::{
    channel::{Channel, StandardMethodChannel},
    codec::{standard_codec::Value, MethodCallResult},
    FlutterEngineInner, PlatformMessage, Plugin, PluginRegistry, Window,
};
use log::{debug, info, trace};

mod crypto;

const CHANNEL_NAME: &str = "plugins.it_nomads.com/flutter_secure_storage";
const STORAGE_FILE_NAME: &str = "secure_storage.json";

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
    crypto: Crypto,
}

impl FlutterSecureStoragePlugin {
    pub fn new() -> Self {
        let mut storage = match dirs::data_dir() {
            Some(dir) => match File::open(dir.join("openbook").join(STORAGE_FILE_NAME)) {
                Ok(file) => match serde_json::from_reader(file) {
                    Ok(result) => result,
                    _ => HashMap::new(),
                },
                _ => HashMap::new(),
            },
            None => HashMap::new(),
        };

        let crypto = Crypto::from_storage(&mut storage).expect("Failed to create crypto");

        Self::save(&storage);

        Self {
            channel: StandardMethodChannel::new(CHANNEL_NAME),
            storage,
            crypto,
        }
    }

    fn save(storage: &HashMap<String, String>) {
        match serde_json::to_string(storage) {
            Ok(data) => match dirs::data_dir() {
                Some(dir) => {
                    std::fs::write(dir.join("openbook").join(STORAGE_FILE_NAME), data).ok()
                }
                None => None,
            },
            _ => None,
        };
    }

    fn read(&self, args: &ReadArgs) -> MethodCallResult<Value> {
        trace!("Read key {}", args.key);

        match self.storage.get(args.key) {
            Some(v) => match self.crypto.decrypt(v) {
                Ok(data) => MethodCallResult::Ok(Value::String(data)),
                _ => MethodCallResult::Err {
                    details: Value::Null,
                    code: String::from(""),
                    message: String::from(""),
                },
            },
            None => MethodCallResult::Ok(Value::Null),
        }
    }

    fn write(&mut self, args: &WriteArgs) -> MethodCallResult<Value> {
        trace!("Write key {}. New value: {}", args.key, args.value);

        match self.crypto.encrypt(args.value) {
            Ok(data) => {
                self.storage.insert(String::from(args.key), data);

                Self::save(&self.storage);

                MethodCallResult::Ok(Value::Null)
            }
            _ => MethodCallResult::Err {
                details: Value::Null,
                code: String::from(""),
                message: String::from(""),
            },
        }
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
            if let Ok(data) = self.crypto.decrypt(value) {
                map.insert(Value::String(key.clone()), Value::String(data));
            }
        }
        MethodCallResult::Ok(Value::Map(map))
    }

    fn delete_all(&mut self) -> MethodCallResult<Value> {
        trace!("Delete all");
        self.storage.clear();
        MethodCallResult::Ok(Value::Null)
    }

    //    fn decode_value(&self, key: &str) -> Option<String> {
    //        if let Some(value) = self.storage.get(key) {
    //            if let Ok(bytes) = base64::decode(value) {}
    //        }
    //        None
    //    }
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
