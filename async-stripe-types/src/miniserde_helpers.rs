#![allow(missing_debug_implementations)]
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

use miniserde::Deserialize;
use miniserde::de::{Map, Visitor};
use miniserde::json::{Number, Object, Value};

use crate::Currency;

#[derive(Default)]
pub struct ObjectBuilderInner {
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl ObjectBuilderInner {
    pub fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }

    pub fn key_inner(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(String, Object)> {
        self.shift();
        let object_key = self.object.get("object").and_then(|o| match o {
            Value::String(object_key) => Some(object_key.clone()),
            _ => None,
        })?;
        let final_object = mem::replace(&mut self.object, Object::new());
        Some((object_key, final_object))
    }
}

#[derive(Default)]
pub struct MaybeDeletedBuilderInner {
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl MaybeDeletedBuilderInner {
    pub fn from_object(object: Object) -> Self {
        Self { object, ..Self::default() }
    }

    pub fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }

    pub fn key_inner(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(bool, Object)> {
        self.shift();
        let deleted = self
            .object
            .get("deleted")
            .map(|o| match o {
                Value::Bool(bool) => *bool,
                _ => false,
            })
            .unwrap_or(false);
        let final_object = mem::replace(&mut self.object, Object::new());
        Some((deleted, final_object))
    }
}

pub trait FromValueOpt {
    fn from_value(v: Value) -> Option<Self>
    where
        Self: Sized;
}

/// Build a JSON object before converting it through `FromValueOpt`.
/// Used by opaque object newtypes without instantiating any concrete model.
pub fn object_value_map<T: FromValueOpt>(out: &mut Option<T>) -> Box<dyn Map + '_> {
    Box::new(ObjectValueMap { out, inner: ObjectBuilderInner::default() })
}

struct ObjectValueMap<'a, T> {
    out: &'a mut Option<T>,
    inner: ObjectBuilderInner,
}

impl<T: FromValueOpt> Map for ObjectValueMap<'_, T> {
    fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.inner.key_inner(key)
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        self.inner.shift();
        let object = mem::take(&mut self.inner.object);
        *self.out = Some(T::from_value(Value::Object(object)).ok_or(miniserde::Error)?);
        Ok(())
    }
}

impl FromValueOpt for bool {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl FromValueOpt for i64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => i64::try_from(u).ok(),
                Number::I64(i) => Some(i),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for u64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for f64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u as f64),
                Number::I64(i) => Some(i as f64),
                Number::F64(f) => Some(f),
            },
            _ => None,
        }
    }
}
impl FromValueOpt for u32 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u.try_into().ok()?),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}
impl FromValueOpt for u8 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u.try_into().ok()?),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for String {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::String(str) => Some(str),
            _ => None,
        }
    }
}

impl<T: FromValueOpt> FromValueOpt for Option<T> {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Null => Some(None),
            val => T::from_value(val).map(Some),
        }
    }
}

impl<T: FromValueOpt> FromValueOpt for Vec<T> {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Array(items) => items.into_iter().map(|i| T::from_value(i)).collect(),
            _ => None,
        }
    }
}

impl<V: FromValueOpt> FromValueOpt for HashMap<String, V> {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        obj.into_iter()
            .map(|(k, v)| {
                let v = V::from_value(v)?;
                Some((k, v))
            })
            .collect::<Option<HashMap<_, _>>>()
    }
}

impl<V: FromValueOpt> FromValueOpt for HashMap<Currency, V> {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        obj.into_iter()
            .map(|(k, v)| {
                let k = Currency::from_str(&k).ok()?;
                let v = V::from_value(v)?;
                Some((k, v))
            })
            .collect::<Option<HashMap<_, _>>>()
    }
}

pub fn extract_object_discr(value: Value) -> Option<(String, Object)> {
    let Value::Object(obj) = value else { return None };
    let obj_name = obj.get("object")?;
    let obj_str = match obj_name {
        Value::String(str) => str.to_string(),
        _ => return None,
    };
    Some((obj_str, obj))
}

impl FromValueOpt for Value {
    fn from_value(v: Value) -> Option<Self> {
        Some(v)
    }
}

impl<T: FromValueOpt> FromValueOpt for Box<T> {
    fn from_value(v: Value) -> Option<Self> {
        Some(Box::new(T::from_value(v)?))
    }
}

#[macro_export]
macro_rules! impl_from_val_with_from_str {
    ($struct_name:ident) => {
        impl $crate::miniserde_helpers::FromValueOpt for $struct_name {
            fn from_value(v: miniserde::json::Value) -> Option<Self> {
                match v {
                    miniserde::json::Value::String(str) => std::str::FromStr::from_str(&str).ok(),
                    _ => None,
                }
            }
        }
    };
}

impl_from_val_with_from_str!(Currency);
