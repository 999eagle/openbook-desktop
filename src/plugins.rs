pub use self::flutter_secure_storage::FlutterSecureStoragePlugin;

use flutter_engine::codec::standard_codec::Value;

macro_rules! plugin_args {
    {$name:ident, $($field:ident: $ty:ty, $map_name:expr, $($map_pattern:pat => $map_value:expr),*;)*} => {
        // create struct
        struct $name<'a> {
            // each field is a borrow with lifetime 'a
            $(pub $field: &'a $ty),*
        }

        impl<'a> $name<'a> {
            pub fn from_value(value: &'a Value) -> Result<Self, DecodeError> {
                // check that we have a Value::Map, otherwise error out
                let map = match value {
                    Value::Map(map) => map,
                    _ => return Err(DecodeError::WrongType),
                };
                // declare variables for each field as Option<T>
                $(let mut $field: Option<&$ty> = None;)*
                // iterate through the map
                for (key, value) in map.iter() {
                    // get key from Value::String, otherwise error out
                    let key = match key {
                        Value::String(string) => string,
                        _ => return Err(DecodeError::WrongType),
                    };
                    // match key to field name
                    match key.as_str() {
                        $(
                            $map_name => {
                                // get value from Value, otherwise error out
                                $field = Some(match value {
                                    $(
                                        $map_pattern => $map_value,
                                    )*
                                    _ => return Err(DecodeError::WrongType),
                                });
                            },
                        )*
                        _ => return Err(DecodeError::UnknownMapKey),
                    }
                }
                // check that no field was left as None
                if $($field.is_none())||* {
                    return Err(DecodeError::MissingMapKey);
                }
                // create struct and return
                Ok(Self{
                    $($field: $field.unwrap(),)*
                })
            }
        }
    };
}

mod flutter_secure_storage;

enum DecodeError {
    WrongType,
    UnknownMapKey,
    MissingMapKey,
}

fn debug_print_args(value: &Value) -> String {
    match value {
        Value::String(string) => format!("String: {}", string),
        Value::Boolean(bool) => format!("Boolean: {}", bool),
        Value::F64(num) => format!("F64: {}", num),
        Value::I32(num) => format!("I32: {}", num),
        Value::I64(num) => format!("I64: {}", num),
        Value::LargeInt => String::from("LargeInt"),
        Value::Null => String::from("Null"),
        Value::F64List(list) => format!("F64List"),
        Value::I32List(list) => format!("I32List"),
        Value::I64List(list) => format!("I64List"),
        Value::U8List(list) => format!("U8List"),
        Value::List(list) => format!("List"),
        Value::Map(map) => {
            let mut string = String::from("Map: {\n");
            for (key, value) in map.iter() {
                string += format!(
                    "\t{}:\n\t{}\n",
                    &debug_print_args(key),
                    &debug_print_args(value)
                )
                .as_str();
            }
            string + "}"
        }
    }
}
