use std::mem;

use miniserde::de::{Map, Visitor};
use miniserde::json::{from_str, to_string, Object, Value};
use miniserde::{make_place, Deserialize};

#[derive(Deserialize)]
pub struct Account {
    pub email: String,
    pub link: Option<String>,
}

#[derive(Deserialize)]
pub struct Bank {
    pub card: i64,
    pub name: Option<i64>,
}

pub enum Source {
    Bank(Bank),
    Account(Account),
}

struct SourceBuilder<'a> {
    out: &'a mut Option<Source>,
    obj_key: Option<String>,
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl<'a> SourceBuilder<'a> {
    fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }
}
make_place!(Place);

impl Visitor for Place<Source> {
    fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
        Ok(Box::new(SourceBuilder {
            out: &mut self.out,
            obj_key: None,
            object: Default::default(),
            key: None,
            value: None,
        }))
    }
}

impl Deserialize for Source {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl<'a> Map for SourceBuilder<'a> {
    fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        if k == "object" {
            return Ok(Deserialize::begin(&mut self.obj_key));
        }
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        self.shift();
        let obj_key = self.obj_key.take().ok_or(miniserde::Error)?;
        let final_object = Value::Object(mem::replace(&mut self.object, Object::new()));
        match obj_key.as_str() {
            "bank" => {
                *self.out = Some(Source::Bank(from_str(&to_string(&final_object))?));
            }
            "account" => {
                *self.out = Some(Source::Account(from_str(&to_string(&final_object))?));
            }
            _ => return Err(miniserde::Error),
        }
        Ok(())
    }
}

fn main() {
    let acct = r#"{"email": "a@a.com"}"#;
    let account: Account = from_str(acct).unwrap();
}
