use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::Value;
/*
#[derive(Serialize)]
struct Justmike2000 {
    a: String,
    b: String,
    c: u64,
}
*/
pub fn get_field_by_name<T, R>(data: T, field: &str) -> Option<R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let mut map = match serde_value::to_value(data) {
        Ok(Value::Map(map)) => Some(map),
        _ => None,
    };

    let key = Value::String(field.to_owned());

    let value = match map.as_mut() {
        Some(val) => {
            match val.remove(&key) {
                Some(value) => Some(value),
                None => None,
            }
        },
        None => None
    };

    match value {
        Some(res) => {
            match R::deserialize(res) {
                Ok(r) => Some(r),
                Err(_) => None,
            }
        },
        None => None
    }
    
}
