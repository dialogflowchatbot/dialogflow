// use std::borrow::Cow;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::flow::rt::context::Context;
use crate::flow::rt::dto::Request;

#[derive(Deserialize, Serialize)]
pub(crate) struct SimpleVariable {
    #[serde(rename = "varName")]
    pub(crate) var_name: String,
    #[serde(rename = "varType")]
    pub(crate) var_type: VariableType,
    #[serde(rename = "varVal")]
    pub(crate) var_val: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Variable {
    // #[serde(rename(deserialize = "varName"))]
    #[serde(rename = "varName")]
    pub(crate) var_name: String,
    #[serde(rename = "varType")]
    pub(crate) var_type: VariableType,
    #[serde(rename = "varValueSource")]
    pub(crate) var_val_source: VariableValueSource,
    #[serde(rename = "varConstantValue")]
    pub(crate) var_constant_value: String,
    #[serde(rename = "varAssociateData")]
    pub(crate) var_associate_data: String,
    #[serde(rename = "obtainValueExpressionType")]
    pub(crate) obtain_value_expression_type: VariableObtainValueExpressionType,
    #[serde(rename = "obtainValueExpression")]
    pub(crate) obtain_value_expression: String,
    #[serde(rename = "cacheEnabled")]
    pub(crate) cach_enabled: bool,
}

impl Variable {
    fn get_data_from_scraper(&self, s: &str) -> Option<String> {
        let parts = &self
            .obtain_value_expression
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mut selectors = Vec::with_capacity(parts.len());
        for part in parts {
            if let Ok(selector) = scraper::Selector::parse(part) {
                selectors.push(selector);
            } else {
                return None;
            }
        }
        let doc = scraper::Html::parse_document(s);
        let mut ele_ref_op = doc.select(&selectors.pop().unwrap()).next();
        for sel in selectors.iter() {
            if let Some(ele_ref) = ele_ref_op {
                ele_ref_op = ele_ref.select(sel).next();
            } else {
                return None;
            }
        }
        if let Some(ele_ref) = ele_ref_op {
            if let Some(res) = ele_ref.text().next() {
                return Some(String::from(res));
            }
        }
        None
    }
    fn get_data_from_res<'a, 'b>(
        &'b self,
        ctx: &'a mut Context,
        s: &'b str,
    ) -> Option<&'a VariableValue> {
        let mut str_store: Option<String> = None;
        let r = match self.obtain_value_expression_type {
            VariableObtainValueExpressionType::JsonPointer => {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
                    if let Some(r) = v.pointer(&self.obtain_value_expression) {
                        if r.is_string() {
                            str_store = Some(String::from(r.as_str().unwrap()));
                            str_store.as_ref().unwrap()
                        } else {
                            s
                        }
                    } else {
                        s
                    }
                } else {
                    s
                }
            }
            VariableObtainValueExpressionType::HtmlScrape => {
                str_store = self.get_data_from_scraper(s);
                if str_store.is_some() {
                    str_store.as_ref().unwrap()
                } else {
                    s
                }
            }
            VariableObtainValueExpressionType::JavaScript => s,
            VariableObtainValueExpressionType::None => s,
        };
        // println!("{}", r);
        let v = VariableValue::new(r, &self.var_type);
        if self.cach_enabled {
            ctx.vars.insert(self.var_name.clone(), v);
            return ctx.vars.get(&self.var_name);
        } else {
            ctx.none_persistent_vars.insert(self.var_name.clone(), v);
            return ctx.none_persistent_vars.get(&self.var_name);
        }
    }
    pub(crate) fn get_value<'a, 'b>(
        &'a self,
        req: &'b Request,
        ctx: &'b mut Context,
    ) -> Option<&'b VariableValue> {
        if self.cach_enabled && ctx.vars.contains_key(&self.var_name) {
            // println!("get from cache");
            ctx.vars.get(&self.var_name)
        } else {
            self.get_value2(req, ctx)
        }
        /*
        fn get_or_update(key: u32, map: &mut HashMap<u32, String>) -> Result<&str, Error> {
            use std::collections::hash_map::Entry;

            Ok(match map.entry(key) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(get_value()?),
            })
        }
        */
    }
    fn get_value2<'a, 'b>(
        &'a self,
        req: &'b Request,
        ctx: &'b mut Context,
    ) -> Option<&'b VariableValue> {
        match &self.var_val_source {
            VariableValueSource::Collect | VariableValueSource::Import => {
                // println!("{:?}", ctx.vars.get(&self.var_name));
                ctx.vars.get(&self.var_name)
            }
            VariableValueSource::UserInput => {
                let v = VariableValue::new(&req.user_input, &self.var_type);
                ctx.vars.insert(self.var_name.clone(), v);
                ctx.vars.get(&self.var_name)
            }
            VariableValueSource::Constant => {
                let v = VariableValue::new(&self.var_constant_value, &self.var_type);
                ctx.vars.insert(self.var_name.clone(), v);
                ctx.vars.get(&self.var_name)
            }
            VariableValueSource::ExternalHttp => {
                let cache = {
                    if let Some(c) = ctx.none_persistent_data.get(&self.var_associate_data) {
                        let mut cache = String::with_capacity(512);
                        cache.push_str(c);
                        Some(cache)
                    } else {
                        None
                    }
                };
                if cache.is_some() {
                    return self.get_data_from_res(ctx, cache.as_ref().unwrap());
                }
                if let Ok(op) =
                    crate::external::http::crud::get_detail(&req.robot_id, &self.var_associate_data)
                {
                    if let Some(api) = op {
                        return tokio::task::block_in_place(
                            /*move*/
                            || match tokio::runtime::Handle::current()
                                .block_on(crate::external::http::client::req(api, &ctx.vars, false))
                            {
                                Ok(r) => match r {
                                    crate::external::http::dto::ResponseData::Str(s) => {
                                        // 下面这句，需要在get_data_from_res的上方，否则会报*ctx可变借用了两次，因为返回值，对ctx有引用
                                        ctx.none_persistent_data
                                            .insert(self.var_associate_data.clone(), s.clone());
                                        return self.get_data_from_res(ctx, &s);
                                    }
                                    _ => None,
                                },
                                Err(e) => {
                                    log::error!("{:?}", e);
                                    None
                                }
                            },
                        );
                    }
                }
                None
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum VariableValue {
    Str(String),
    Num(f64),
    Array(Vec<VariableValue>),
}

impl VariableValue {
    pub(crate) fn new(v: &str, t: &VariableType) -> Self {
        match t {
            VariableType::Str => VariableValue::Str(String::from(v)),
            VariableType::Num => VariableValue::Num(v.parse::<f64>().unwrap_or(0f64)),
        }
    }
    // pub(crate) fn val_to_string(&self) -> Cow<'_, str> {
    pub(crate) fn val_to_string(&self) -> String {
        match self {
            VariableValue::Str(s) => String::from(s),
            VariableValue::Num(n) => n.to_string(),
            VariableValue::Array(arr) => {
                let mut s = String::with_capacity(512);
                s.push('[');
                for item in arr.iter() {
                    s.push_str(&item.val_to_string());
                    s.push(',');
                }
                s.pop();
                s.push(']');
                s
            }
        }
        // Cow::Owned(s)
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub(crate) enum VariableType {
    Str,
    Num,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum VariableValueSource {
    Import,
    Collect,
    UserInput,
    Constant,
    ExternalHttp,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum VariableObtainValueExpressionType {
    None,
    JsonPointer,
    HtmlScrape,
    JavaScript,
}
