use crate::Value;
use std::collections::{BTreeMap, HashMap};

/// A trait for converting types to `Value`.
pub trait ToValueTrait {
    /// Converts a type into a `Value`.
    fn to_value(&self) -> Value;
}
/// A trait for converting `Value` to types.
pub trait FromValueTrait {
    type Item;
    /// Converts a `Value` into a type.
    fn from_value(value: Value) -> Option<Self::Item>;
}

impl FromValueTrait for i8 {
    type Item = i8;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i8()
        } else {
            None
        }
    }
}

impl FromValueTrait for i16 {
    type Item = i16;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i16()
        } else {
            None
        }
    }
}

impl FromValueTrait for i32 {
    type Item = i32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i32()
        } else {
            None
        }
    }
}

impl FromValueTrait for i64 {
    type Item = i64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i64()
        } else {
            None
        }
    }
}

impl FromValueTrait for i128 {
    type Item = i128;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_i128()
        } else {
            None
        }
    }
}

impl FromValueTrait for u8 {
    type Item = u8;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u8()
        } else {
            None
        }
    }
}

impl FromValueTrait for u16 {
    type Item = u16;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u16()
        } else {
            None
        }
    }
}

impl FromValueTrait for u32 {
    type Item = u32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u32()
        } else {
            None
        }
    }
}

impl FromValueTrait for u64 {
    type Item = u64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u64()
        } else {
            None
        }
    }
}

impl FromValueTrait for u128 {
    type Item = u128;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_u128()
        } else {
            None
        }
    }
}

impl FromValueTrait for f32 {
    type Item = f32;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_f32()
        } else {
            None
        }
    }
}

impl FromValueTrait for f64 {
    type Item = f64;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Number(number) = value {
            number.get_f64()
        } else {
            None
        }
    }
}

impl FromValueTrait for String {
    type Item = String;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::String(string_b) = value {
            Some(string_b.to_string())
        } else {
            None
        }
    }
}

impl FromValueTrait for bool {
    type Item = bool;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Boolean(bool) = value {
            Some(bool)
        } else {
            None
        }
    }
}

impl<T> FromValueTrait for Vec<T>
where
    T: FromValueTrait,
{
    type Item = Vec<<T as FromValueTrait>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Array(array) = value {
            Some(
                array
                    .into_iter()
                    .map(|value| T::from_value(value).unwrap())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }
}

impl<T> FromValueTrait for HashMap<String, T>
where
    T: FromValueTrait,
{
    type Item = HashMap<String, <T as FromValueTrait>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Object(array) = value {
            Some(array.iter().fold(HashMap::new(), |mut map, (key, value)| {
                map.insert(key.to_string(), T::from_value(value.clone()).unwrap());
                map
            }))
        } else {
            None
        }
    }
}

impl<T> FromValueTrait for BTreeMap<String, T>
where
    T: FromValueTrait,
{
    type Item = BTreeMap<String, <T as FromValueTrait>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        if let Value::Object(array) = value {
            Some(array.iter().fold(BTreeMap::new(), |mut map, (key, value)| {
                map.insert(key.to_string(), T::from_value(value.clone()).unwrap());
                map
            }))
        } else {
            None
        }
    }
}

impl<T> FromValueTrait for Option<T>
where
    T: FromValueTrait,
{
    type Item = Option<<T as FromValueTrait>::Item>;

    fn from_value(value: Value) -> Option<Self::Item> {
        match value {
            Value::Null => None,
            _ => Some(T::from_value(value)),
        }
    }
}


/// A trait for converting types to JSON strings.
pub trait ToJsonTrait {
    /// Converts a type into a JSON string.
    fn to_json(&self) -> String;
}

/// A trait for converting types to YAML strings.
pub trait ToYamlTrait {
    /// Converts a type into a YAML string.
    fn to_yaml(&self) -> String;
}

/// A trait for converting types to XML strings.
pub trait ToXmlTrait {
    /// Converts a type into an XML string.
    fn to_xml(&self) -> String;
}
