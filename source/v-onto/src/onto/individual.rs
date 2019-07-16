use crate::datatype::{DataType, Lang};
use crate::parser::*;
use crate::resource::{Resource, Value};
use std::collections::HashMap;
use std::fmt;

pub enum IndividualError {
    None,
    ParseError,
}

pub struct IndividualObj {
    pub uri: String,
    pub resources: HashMap<String, Vec<Resource>>,
}

pub struct RawObj {
    pub data: Vec<u8>,
    pub cur: u64,
    pub len_predicates: u32,
    pub cur_predicates: u32,
    pub raw_type: RawType,
}

impl RawObj {
    pub fn new(buff: Vec<u8>) -> RawObj {
        RawObj {
            data: buff,
            raw_type: RawType::UNKNOWN,
            cur: 0,
            len_predicates: 0,
            cur_predicates: 0,
        }
    }

    pub fn new_empty() -> RawObj {
        RawObj {
            data: Vec::new(),
            raw_type: RawType::UNKNOWN,
            cur: 0,
            len_predicates: 0,
            cur_predicates: 0,
        }
    }
}

pub struct Individual {
    pub obj: IndividualObj,
    pub raw: RawObj,
}

impl Individual {
    pub fn new_raw(raw: RawObj) -> Self {
        Individual {
            obj: IndividualObj::new(),
            raw: raw,
        }
    }

    pub fn new() -> Self {
        Individual {
            obj: IndividualObj::new(),
            raw: RawObj::new_empty(),
        }
    }

    pub fn any_exists(&mut self, predicate: &str, values: &[&str]) -> bool {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => {
                    for el in v {
                        if let Value::Str(s, _l) = &el.value {
                            for ve in values {
                                if str::eq (ve, s) {
                                    return true;
                                }
                            }
                        }
                    }
                }
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        false
    }
    /*
        pub fn get_resources(&mut self, predicate: &str) -> Result<Vec<Resource>, IndividualError> {
            for _ in 0..2 {
                match self.obj.resources.get(predicate) {
                    Some(v) => {
                        return Ok(v.to_vec());
                    }
                    None => {
                        if self.raw.cur < self.raw.data.len() as u64 {
                            // next parse
                            if !parse_to_predicate(predicate, self) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            Err(IndividualError::None)
        }
    */
    pub fn get_literals(&mut self, predicate: &str) -> Result<Vec<String>, IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => {
                    return Ok(v
                        .iter()
                        .map(|el| {
                            if let Value::Str(s, _l) = &el.value {
                                s.to_string()
                            } else {
                                "".to_string()
                            }
                        })
                        .collect::<Vec<String>>());
                }
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn get_first_literal(&mut self, predicate: &str) -> Result<String, IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => match &v[0].value {
                    Value::Str(s, _l) => {
                        return Ok(s.to_string());
                    }
                    _ => {
                        return Err(IndividualError::ParseError);
                    }
                },
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn get_first_binobj(&mut self, predicate: &str) -> Result<Vec<u8>, IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => match &v[0].value {
                    Value::Binary(s) => {
                        return Ok(s.clone());
                    }
                    _ => {
                        return Err(IndividualError::ParseError);
                    }
                },
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn get_first_integer(&mut self, predicate: &str) -> Result<i64, IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => {
                    if let Value::Int(i) = &v[0].value {
                        return Ok(*i);
                    }
                }
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn get_first_number(&mut self, predicate: &str) -> Result<(i64, i64), IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => {
                    if let Value::Num(m, e) = &v[0].value {
                        return Ok((*m, *e));
                    }
                }
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn get_first_float(&mut self, predicate: &str) -> Result<f64, IndividualError> {
        for _ in 0..2 {
            match self.obj.resources.get(predicate) {
                Some(v) => {
                    return Ok(v[0].get_float());
                }
                None => {
                    if self.raw.cur < self.raw.data.len() as u64 {
                        // next parse
                        if !parse_to_predicate(predicate, self) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Err(IndividualError::None)
    }

    pub fn parse_all(&mut self) {
        while self.raw.cur < self.raw.data.len() as u64 {
            // next parse
            if !parse_to_predicate("?", self) {
                break;
            }
        }
    }
}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uri={}, \n {:#?}", self.obj.uri, self.obj.resources)
    }
}

impl IndividualObj {
    pub fn new() -> Self {
        IndividualObj {
            uri: "".to_string(),
            resources: HashMap::new(),
        }
    }

    pub fn get_predicates(&self) -> Vec<String> {
        self.resources.iter().map(|(key, _)| key.clone()).collect()
    }

    pub fn add_bool(&mut self, predicate: &str, b: bool, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Boolean,
            order: order as u16,
            value: Value::Bool(b),
        });
    }

    pub fn add_datetime(&mut self, predicate: &str, i: i64, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Datetime,
            order: order as u16,
            value: Value::Int(i),
        });
    }

    pub fn add_binary(&mut self, predicate: &str, v: Vec<u8>, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Binary,
            order: order as u16,
            value: Value::Binary(v),
        });
    }

    pub fn add_integer(&mut self, predicate: &str, i: i64, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Integer,
            order: order as u16,
            value: Value::Int(i),
        });
    }

    pub fn add_decimal_d(&mut self, predicate: &str, mantissa: i64, exponent: i64, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Decimal,
            order: order as u16,
            value: Value::Num(mantissa, exponent),
        });
    }

    pub fn add_uri(&mut self, predicate: &str, s: &str, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::Uri,
            order: order as u16,
            value: Value::Str(s.to_owned(), Lang::NONE),
        });
    }

    pub fn add_string(&mut self, predicate: &str, s: &str, lang: Lang, order: u32) {
        let values = self.resources.entry(predicate.to_owned()).or_default();
        values.push(Resource {
            rtype: DataType::String,
            order: order as u16,
            value: Value::Str(s.to_owned(), lang),
        });
    }
}
