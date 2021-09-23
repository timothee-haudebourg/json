use super::{Map, Number, Value};
use generic_json::{ValueMut, ValueRef};

pub struct Json;

impl generic_json::Json for Json {
    type MetaData = ();

    type Value = Value;

    type Number = Number;

    type String = String;

    type Array = Vec<Value>;

    type Key = String;

    type Object = Map<String, Value>;
}

impl From<generic_json::Value<Json>> for Value {
    fn from(v: generic_json::Value<Json>) -> Self {
        match v {
            generic_json::Value::Null => Self::Null,
            generic_json::Value::Boolean(b) => Self::Bool(b),
            generic_json::Value::Number(n) => Self::Number(n),
            generic_json::Value::String(s) => Self::String(s),
            generic_json::Value::Array(a) => Self::Array(a),
            generic_json::Value::Object(o) => Self::Object(o),
        }
    }
}

impl<'a> From<&'a Value> for ValueRef<'a, Json> {
    fn from(v: &'a Value) -> Self {
        match v {
            Value::Null => ValueRef::Null,
            Value::Bool(b) => ValueRef::Boolean(*b),
            Value::Number(n) => ValueRef::Number(n),
            Value::String(s) => ValueRef::String(s.as_ref()),
            Value::Array(a) => ValueRef::Array(a),
            Value::Object(o) => ValueRef::Object(o),
        }
    }
}

impl<'a> From<&'a mut Value> for ValueMut<'a, Json> {
    fn from(v: &'a mut Value) -> Self {
        match v {
            Value::Null => ValueMut::Null,
            Value::Bool(b) => ValueMut::Boolean(b),
            Value::Number(n) => ValueMut::Number(n),
            Value::String(s) => ValueMut::String(s),
            Value::Array(a) => ValueMut::Array(a),
            Value::Object(o) => ValueMut::Object(o),
        }
    }
}

impl generic_json::MetaValue<Json> for Value {
    fn new(value: generic_json::Value<Json>, _metadata: ()) -> Self {
        value.into()
    }

    fn value(&self) -> ValueRef<'_, Json> {
        self.into()
    }

    fn value_mut(&mut self) -> ValueMut<'_, Json> {
        self.into()
    }

    fn metadata(&self) -> &() {
        &()
    }

    fn as_pair(&self) -> (ValueRef<'_, Json>, &()) {
        (self.into(), &())
    }

    fn as_pair_mut(&mut self) -> (ValueMut<'_, Json>, &()) {
        (self.into(), &())
    }
}
