use crate::datatype::*;
use crate::individual::*;
use crate::parser::*;
use cbor::types::Type;
use cbor::{Config, Decoder};
use std::io::Cursor;

#[derive(PartialEq, Debug)]
pub enum TagId {
    None = 255,
    TextRu = 42,
    TextEn = 43,
    StandardDateTime = 0,
    EpochDateTime = 1,
    PositiveBigint = 2,
    NegativeBigint = 3,
    DecimalFraction = 4,
    CborEncoded = 24,
    URI = 32,
}

pub fn parse_cbor(raw: &mut RawObj) -> Result<String, i8> {
    if raw.data.is_empty() || raw.raw_type != RawType::CBOR {
        return Err(-1);
    }

    let input = Cursor::new(raw.data.to_owned());
    let mut d = Decoder::new(Config::default(), input);

    if let Ok(len) = d.object() {
        raw.len_predicates = len as u32;
        if let Ok(type_info) = d.typeinfo() {
            if let Ok(predicate) = d._text(&type_info) {
                if predicate == "@" {
                    if let Ok(type_info) = d.typeinfo() {
                        if let Ok(uri) = d._text(&type_info) {
                            raw.cur = d.into_reader().position();
                            return Ok(uri);
                        }
                    }
                } else {
                    return Err(-1);
                }
            }
        }
    }

    Err(-1)
}

pub fn parse_cbor_to_predicate(expect_predicate: &str, iraw: &mut Individual) -> bool {
    if iraw.raw.cur >= iraw.raw.data.len() as u64 {
        return false;
    }

    let mut is_found = false;
    let mut cur = Cursor::new(iraw.raw.data.as_slice());
    cur.set_position(iraw.raw.cur);
    let mut d = Decoder::new(Config::default(), cur);

    for _ in iraw.raw.cur_predicates..iraw.raw.len_predicates {
        if let Ok(type_info) = d.typeinfo() {
            if let Ok(predicate) = d._text(&type_info) {
                debug!("predicate {:?}", &predicate);

                if predicate == expect_predicate {
                    is_found = true;
                }
                if !add_value(&predicate, &mut d, &mut iraw.obj, 0) {
                    return false;
                }
            }
        }

        if is_found {
            iraw.raw.cur = d.into_reader().position();
            return true;
        }
    }

    false
}

fn add_value(predicate: &str, d: &mut Decoder<Cursor<&[u8]>>, indv: &mut IndividualObj, order: u32) -> bool {
    if let Ok((type_info, tag)) = d.typeinfo_and_tag() {
        match type_info.0 {
            Type::Bool => {
                if let Ok(b) = d._bool(&type_info) {
                    indv.add_bool(&predicate, b, order);
                }
            }
            Type::Text => {
                if let Ok(t) = d._text(&type_info) {
                    if tag == TagId::URI as u64 {
                        indv.add_uri(&predicate, &t, order);
                    } else {
                        let mut lang = Lang::NONE;

                        if tag == TagId::TextRu as u64 || tag == TagId::TextEn as u64 {
                            if tag == TagId::TextRu as u64 {
                                lang = Lang::RU;
                            } else if tag == TagId::TextEn as u64 {
                                lang = Lang::EN;
                            }
                        }

                        indv.add_string(predicate, &t, lang, order);
                    }
                }
            }
            Type::UInt8 => {
                if let Ok(i) = d._u8(&type_info) {
                    indv.add_integer(&predicate, i64::from(i), order);
                }
            }
            Type::Int8 => {
                if let Ok(i) = d._i8(&type_info) {
                    indv.add_integer(&predicate, i64::from(i), order);
                }
            }
            Type::UInt16 => {
                if let Ok(i) = d._u16(&type_info) {
                    if tag == TagId::EpochDateTime as u64 {
                        indv.add_datetime(&predicate, i64::from(i), order);
                    } else {
                        indv.add_integer(&predicate, i64::from(i), order);
                    }
                }
            }
            Type::UInt32 => {
                if let Ok(i) = d._u32(&type_info) {
                    indv.add_integer(&predicate, i64::from(i), order);
                }
            }
            Type::Array => {
                if let Ok(len) = d._array(&type_info) {
                    for x in 0..len {
                        add_value(predicate, d, indv, x as u32);
                    }
                } else {
                    return false;
                }
            }
            _ => {
                error!("cbor:unknown type {:?}", type_info.0);
                return false;
            }
        }
    }
    true
}
