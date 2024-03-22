use std::mem;

use miniserde::de::Visitor;
use miniserde::json::{Object, Value};
use miniserde::Deserialize;

#[derive(Default)]
pub struct ObjectBuilderInner {
    obj_key: Option<String>,
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
        if k == "object" {
            return Ok(Deserialize::begin(&mut self.obj_key));
        }
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(String, Value)> {
        self.shift();
        let obj_key = self.obj_key.take()?;
        let final_object = Value::Object(mem::replace(&mut self.object, Object::new()));
        Some((obj_key, final_object))
    }
}

#[derive(Default)]
pub struct MaybeDeletedBuilderInner {
    deleted: Option<bool>,
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl MaybeDeletedBuilderInner {
    pub fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }

    pub fn key_inner(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        if k == "deleted" {
            return Ok(Deserialize::begin(&mut self.deleted));
        }
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(bool, Value)> {
        self.shift();
        let deleted = self.deleted.take()?;
        let final_object = Value::Object(mem::replace(&mut self.object, Object::new()));
        Some((deleted, final_object))
    }
}
