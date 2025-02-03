use std::collections::HashMap;
use std::vec::Vec;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;

use super::detector;
use super::dto::{Intent, IntentDetail, IntentFormData, IntentPhraseData};
use crate::db;
use crate::db_executor;
use crate::result::{Error, Result};
use crate::web::server::to_res;

pub(crate) const INTENT_LIST_KEY: &str = "intents";
// pub(crate) const TABLE: redb::TableDefinition<&str, &[u8]> =
//     redb::TableDefinition::new(INTENT_LIST_KEY);
pub(crate) const TABLE_SUFFIX: &str = "intents";

pub(crate) fn init(robot_id: &str, is_en: bool) -> Result<()> {
    let mut intents: Vec<Intent> = Vec::with_capacity(2);

    // Positive
    let keywords = if is_en {
        vec![
            "sure",
            "ok",
            "okay",
            "no problem",
            "affim",
            "certainly",
            "of course",
            "definitely",
            "correct",
            "pleasant",
            "yes",
        ]
    } else {
        vec![
            "嗯", "恩", "可以", "是", "是的", "好", "好的", "对", "对的", "ok", "OK", "Ok", "知道",
            "明白", "行", "愿意", "方便", "正确",
        ]
    };
    let regexes: Vec<&str> = vec![];
    let mut intent = Intent::new(if is_en { "Positive" } else { "肯定" });
    let mut intent_detail = IntentDetail::new(1, intent.id.clone(), intent.name.clone());
    let mut v = keywords.into_iter().map(|s| String::from(s)).collect();
    intent_detail.keywords.append(&mut v);
    let mut v = regexes.into_iter().map(|s| String::from(s)).collect();
    intent_detail.regexes.append(&mut v);
    // let intent_detail = IntentDetail {
    //     intent_idx: 0,
    //     intent_id: intent.id.clone(),
    //     intent_name: intent.name.clone(),
    //     keywords: keywords.into_iter().map(|s| String::from(s)).collect(),
    //     regexes: regexes.into_iter().map(|s| String::from(s)).collect(),
    //     phrases: vec![],
    // };
    intent.keyword_num = intent_detail.keywords.len();
    intent.regex_num = intent_detail.regexes.len();

    // let table_name = format!("{}intents", robot_id);
    // let table: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new(&table_name);
    // let mut table = write_txn.open_table(TABLE)?;
    // db::write(table, intent.id.as_str(), &intent_detail)?;
    db_executor!(
        db::write,
        robot_id,
        TABLE_SUFFIX,
        intent.id.as_str(),
        &intent_detail
    )?;

    intents.push(intent);

    // Negative
    let keywords = if is_en {
        vec![
            "no",
            "not",
            "Reject",
            "can't",
            "can not",
            "cannot",
            "deny",
            "forbid",
            "forbidden",
            "stop",
            "gross",
            "impossible",
            "never",
            "rarely",
            "hardly",
            "none",
            "nothing",
            "incorrect",
            "awful",
            "unpleasant",
            "sick",
            "disappointed",
        ]
    } else {
        vec![
            "no",
            "不",
            "不是",
            "不要",
            "不用",
            "没用",
            "不好",
            "没",
            "没有",
            "不清楚",
            "不知道",
            "不明白",
            "不可以",
            "不行",
            "不愿意",
            "不考虑",
            "不方便",
            "不正确",
        ]
    };
    let regexes: Vec<&str> = vec![];
    let mut intent = Intent::new(if is_en { "Negative" } else { "否定" });
    let mut intent_detail = IntentDetail::new(1, intent.id.clone(), intent.name.clone());
    let mut v = keywords.into_iter().map(|s| String::from(s)).collect();
    intent_detail.keywords.append(&mut v);
    let mut v = regexes.into_iter().map(|s| String::from(s)).collect();
    intent_detail.regexes.append(&mut v);
    // let intent_detail = IntentDetail {
    //     intent_idx: 1,
    //     intent_id: intent.id.clone(),
    //     intent_name: intent.name.clone(),
    //     keywords: keywords.into_iter().map(|s| String::from(s)).collect(),
    //     regexes: regexes.into_iter().map(|s| String::from(s)).collect(),
    //     phrases: vec![],
    // };
    intent.keyword_num = intent_detail.keywords.len();
    intent.regex_num = intent_detail.regexes.len();

    // db::write(table, intent.id.as_str(), &intent_detail)?;
    db_executor!(
        db::write,
        robot_id,
        TABLE_SUFFIX,
        intent.id.as_str(),
        &intent_detail
    )?;

    intents.push(intent);

    // db::write(table, INTENT_LIST_KEY, &intents)
    db_executor!(db::write, robot_id, TABLE_SUFFIX, INTENT_LIST_KEY, &intents)
}

pub(crate) async fn list(Query(q): Query<HashMap<String, String>>) -> impl IntoResponse {
    // let r: Result<Option<Vec<Intent>>> = db::query(TABLE, INTENT_LIST_KEY);
    if let Some(robot_id) = q.get("robotId") {
        let r: Result<Option<Vec<Intent>>> =
            db_executor!(db::query, robot_id, TABLE_SUFFIX, INTENT_LIST_KEY);
        to_res(r)
    } else {
        to_res(Err(Error::ErrorWithMessage(String::from(
            "Parameter: robotId is missing.",
        ))))
    }
}

pub(crate) async fn add(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let r = add_intent(&params.robot_id, params.data.as_str());
    to_res(r)
}

fn add_intent(robot_id: &str, intent_name: &str) -> Result<()> {
    // let d: Option<Vec<Intent>> = db::query(TABLE, INTENT_LIST_KEY)?;
    let d: Option<Vec<Intent>> = db_executor!(db::query, robot_id, TABLE_SUFFIX, INTENT_LIST_KEY)?;
    let mut intents = d.unwrap();
    let intent_idx = intents.len();
    let intent = Intent::new(intent_name);
    intents.push(intent.clone());
    // db::write(TABLE, INTENT_LIST_KEY, &intents)?;
    db_executor!(db::write, robot_id, TABLE_SUFFIX, INTENT_LIST_KEY, &intents)?;

    let intent_detail = IntentDetail::new(intent_idx, intent.id.clone(), intent.name.clone());
    // db::write(TABLE, intent.id.as_str(), &intent_detail)?;
    // Ok(())
    db_executor!(
        db::write,
        robot_id,
        TABLE_SUFFIX,
        intent.id.as_str(),
        &intent_detail
    )
}

pub(crate) async fn remove(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let r = super::phrase::remove_by_intent_id(&params.robot_id, params.id.as_str())
        .await
        .and_then(|_| {
            db_executor!(
                db::remove,
                &params.robot_id,
                TABLE_SUFFIX,
                params.id.as_str()
            )
            .and_then(|_| {
                params
                    .data
                    .parse::<usize>()
                    .map_err(|e| {
                        log::error!("{:?}", &e);
                        Error::ErrorWithMessage(String::from("Invalid idx parameter."))
                    })
                    .and_then(|idx| {
                        let mut intents: Vec<Intent> = db_executor!(
                            db::query,
                            &params.robot_id,
                            TABLE_SUFFIX,
                            INTENT_LIST_KEY
                        )
                        .unwrap()
                        .unwrap();
                        intents.remove(idx);
                        // db::write(TABLE, INTENT_LIST_KEY, &intents)
                        db_executor!(
                            db::write,
                            &params.robot_id,
                            TABLE_SUFFIX,
                            INTENT_LIST_KEY,
                            &intents
                        )
                    })
            })
            // let r = db::remove(TABLE, params.id.as_str());
            // if let Ok(idx) = params.data.parse() {
            //     let mut intents: Vec<Intent> = db::query(TABLE, INTENT_LIST_KEY).unwrap().unwrap();
            //     intents.remove(idx);
            //     if let Err(e) = db::write(TABLE, INTENT_LIST_KEY, &intents) {
            //         log::error!("Update intents list failed: {:?}", &e);
            //     }
            // }
        });
    to_res(r)
}

pub(in crate::intent) fn get_detail_by_id(
    robot_id: &str,
    intent_id: &str,
) -> Result<Option<IntentDetail>> {
    db_executor!(db::query, robot_id, TABLE_SUFFIX, intent_id)
}

pub(crate) async fn detail(Query(params): Query<IntentFormData>) -> impl IntoResponse {
    // let mut od: Option<IntentDetail> = None;
    // let r = db::process_data(dbg!(params.id.as_str()), |d: &mut IntentDetail| {
    //     od = Some(d);
    //     Ok(())
    // }).map(|_| od);
    // to_res(r)
    // let r: Result<Option<IntentDetail>> = db::query(TABLE, params.id.as_str());
    let r = get_detail_by_id(&params.robot_id, params.id.as_str());
    to_res(r)
}

// fn change_num<I: serde::Serialize, F: FnMut(&mut Vec<Intent>), V>(
fn change_num<I: serde::Serialize, F: FnMut(&mut Vec<Intent>)>(
    robot_id: &str,
    key: &str,
    d: &mut I,
    mut f: F,
) -> Result<()> {
    // let mut intents: Vec<Intent> = db::query(TABLE, INTENT_LIST_KEY).unwrap().unwrap();
    let mut intents: Vec<Intent> = db_executor!(db::query, robot_id, TABLE_SUFFIX, INTENT_LIST_KEY)
        .unwrap()
        .unwrap();
    f(&mut intents);
    db::save_txn(vec![
        // (TABLE, key, Box::new(d)),
        // (TABLE, INTENT_LIST_KEY, Box::new(&intents)),
        (robot_id, TABLE_SUFFIX, key, Box::new(d)),
        (robot_id, TABLE_SUFFIX, INTENT_LIST_KEY, Box::new(&intents)),
    ])
}

pub(crate) async fn add_keyword(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let key = params.id.as_str();
    // let r: Result<Option<IntentDetail>> = db::query(TABLE, key);
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    let r = r.and_then(|op| {
        if let Some(mut d) = op {
            d.keywords.push(String::from(params.data.as_str()));
            let idx = d.intent_idx;
            change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
                i[idx].keyword_num = i[idx].keyword_num + 1
            })
        } else {
            Ok(())
        }
    });
    to_res(r)
}

pub(crate) async fn remove_keyword(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let r = params
        .data
        .parse::<usize>()
        .map_err(|e| {
            log::error!("{:?}", e);
            Error::ErrorWithMessage(String::from("Invalid parameter"))
        })
        .and_then(|idx| {
            let key = params.id.as_str();
            let result: Result<Option<IntentDetail>> =
                db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
            result.and_then(|mut op| {
                if op.is_some() {
                    let mut d = op.as_mut().unwrap();
                    d.keywords.remove(idx);
                    let idx = d.intent_idx;
                    change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
                        i[idx].keyword_num = i[idx].keyword_num - 1
                    })
                } else {
                    Ok(())
                }
            })
        });
    to_res(r)
}

pub(crate) async fn add_regex(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let key = params.id.as_str();
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    let r = r.and_then(|op| {
        if let Some(mut d) = op {
            let _ = regex::Regex::new(params.data.as_str())?;
            d.regexes.push(String::from(params.data.as_str()));
            let idx = d.intent_idx;
            change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
                i[idx].regex_num = i[idx].regex_num + 1
            })
        } else {
            Ok(())
        }
    });
    to_res(r)
}

pub(crate) async fn remove_regex(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let r = params
        .data
        .parse::<usize>()
        .map_err(|e| {
            log::error!("{:?}", e);
            Error::ErrorWithMessage(String::from("Invalid parameter"))
        })
        .and_then(|idx| {
            let key = params.id.as_str();
            let result: Result<Option<IntentDetail>> =
                db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
            result.and_then(|mut op| {
                if op.is_some() {
                    let mut d = op.as_mut().unwrap();
                    d.regexes.remove(idx);
                    let idx = d.intent_idx;
                    change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
                        i[idx].regex_num = i[idx].regex_num - 1
                    })
                } else {
                    Ok(())
                }
            })
        });
    to_res(r)
}

#[axum::debug_handler]
pub(crate) async fn add_phrase(
    Query(query): Query<IntentFormData>,
    Json(params): Json<IntentFormData>,
) -> impl IntoResponse {
    let intent_id = params.id.as_str();
    let phrase = &params.data;
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, intent_id);
    if r.is_err() {
        return to_res(r.map(|_| ()));
    }
    let r = r.unwrap();
    if r.is_none() {
        return to_res(Err(Error::ErrorWithMessage(String::from(
            "Can NOT find intention detail",
        ))));
    }
    let mut d = r.unwrap();
    let r = super::phrase::add(&params.robot_id, None, intent_id, &d.intent_name, phrase)
        .await
        .map_err(|e| {
            log::error!("{:#?}", &e);
            Error::ErrorWithMessage(String::from("Invalid idx parameter."))
        })
        .and_then(|vec_row_id| {
            query
                .data
                .parse::<usize>()
                .map_err(|e| {
                    log::error!("{:#?}", &e);
                    Error::ErrorWithMessage(String::from("Invalid idx parameter."))
                })
                .and_then(|idx| {
                    d.phrases.push(IntentPhraseData {
                        id: vec_row_id,
                        phrase: String::from(params.data.as_str()),
                    });
                    change_num(
                        &params.robot_id,
                        intent_id,
                        &mut d,
                        |i: &mut Vec<Intent>| i[idx].phrase_num = i[idx].phrase_num + 1,
                    )
                })
        });
    to_res(r)
}

/*
pub(crate) async fn add_phrase(
    Query(query): Query<IntentFormData>,
    Json(params): Json<IntentFormData>,
) -> impl IntoResponse {
    let key = params.id.as_str();
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    if r.is_err() {
        return to_res(r.map(|_| ()));
    }
    let r = r.unwrap();
    if r.is_none() {
        return to_res(Err(Error::ErrorWithMessage(String::from(
            "Can NOT find intention detail",
        ))));
    }
    let mut d = r.unwrap();
    let r =
        detector::save_intent_embedding(&params.robot_id, key, &d.intent_name, &params.data).await;
    if r.is_err() {
        return to_res(r.map(|_| ()));
    }
    // let r:Result<i64> = Ok(0i64);
    d.phrases.push(IntentPhraseData {
        id: r.unwrap(),
        phrase: String::from(params.data.as_str()),
    });
    let r = query
        .data
        .parse::<usize>()
        .map_err(|e| {
            log::error!("{:#?}", &e);
            Error::ErrorWithMessage(String::from("Invalid idx parameter."))
        })
        .and_then(|idx| {
            change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
                i[idx].phrase_num = i[idx].phrase_num + 1
            })
        });
    to_res(r)
}
*/

pub(crate) async fn remove_phrase(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    let r = params.data.parse::<usize>();
    let phrase_idx = match r {
        Ok(n) => n,
        Err(e) => {
            log::error!("{:?}", e);
            return to_res(Err(Error::ErrorWithMessage(String::from(
                "Invalid parameter",
            ))));
        }
    };
    let key = params.id.as_str();
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    if let Ok(result) = r {
        if let Some(mut d) = result {
            let phrase = d.phrases.remove(phrase_idx);
            let r = super::phrase::remove(&params.robot_id, phrase.id).await;
            if let Err(e) = r {
                return to_res(Err(e));
            }
            let intent_idx = d.intent_idx;
            return to_res(change_num(
                &params.robot_id,
                key,
                &mut d,
                |i: &mut Vec<Intent>| i[intent_idx].phrase_num = i[intent_idx].phrase_num - 1,
            ));
        }
    }
    to_res(Ok(()))
    // let r = params
    //     .data
    //     .parse::<usize>()
    //     .map_err(|e| {
    //         log::error!("{:?}", e);
    //         Error::ErrorWithMessage(String::from("Invalid parameter"))
    //     })
    //     .and_then(|idx| {
    //         let key = params.id.as_str();
    //         let result: Result<Option<IntentDetail>> =
    //             db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    //         result.and_then(|mut op| {
    //             if op.is_some() {
    //                 let mut d = op.as_mut().unwrap();
    //                 let phrase = d.phrases.remove(idx);
    //                 crate::db::embedding::remove(&params.robot_id, key, phrase.id).await.and_then(|_| {
    //                     let idx = d.intent_idx;
    //                     change_num(&params.robot_id, key, &mut d, |i: &mut Vec<Intent>| {
    //                         i[idx].phrase_num = i[idx].phrase_num - 1
    //                     })
    //                 })
    //             } else {
    //                 Ok(())
    //             }
    //         })
    //     });
    // to_res(r)
}

pub(crate) async fn detect(Json(params): Json<IntentFormData>) -> impl IntoResponse {
    to_res(detector::detect(&params.robot_id, &params.data).await)
}

pub(crate) async fn regenerate_embeddings(
    Query(params): Query<IntentFormData>,
) -> impl IntoResponse {
    let key = params.id.as_str();
    let r: Result<Option<IntentDetail>> =
        db_executor!(db::query, &params.robot_id, TABLE_SUFFIX, key);
    if r.is_err() {
        return to_res(r.map(|_| ()));
    }
    let r = r.unwrap();
    if r.is_none() {
        return to_res(Err(Error::ErrorWithMessage(String::from(
            "Can NOT find intention detail",
        ))));
    }
    let d = r.unwrap();
    // let array: Vec<&str> = d.phrases.iter().map(|v| v.phrase.as_ref()).collect();
    let r =
        super::phrase::batch_add(&params.robot_id, &params.data, &d.intent_name, &d.phrases).await;
    to_res(r)
}
