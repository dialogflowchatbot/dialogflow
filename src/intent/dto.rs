use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(crate) struct IntentFormData {
    #[serde(rename = "robotId")]
    pub(crate) robot_id: String,
    pub(crate) id: String,
    pub(crate) data: String,
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub(crate) struct IntentsData {
//     pub(crate) phrase_vec_row_id: i64,
//     pub(crate) intents: Vec<Intent>,
// }

// static VEC_ROW_ID_LOCKER: Mutex<()> = Mutex::new(());

// impl IntentsData {
//     pub(crate) fn inc_get_phrase_vec_id(&mut self) -> Result<i64> {
//         let _l = VEC_ROW_ID_LOCKER.lock()?;
//         self.phrase_vec_row_id = self.phrase_vec_row_id + 1;
//         Ok(self.phrase_vec_row_id)
//     }
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub(crate) struct Intent {
//     pub(crate) id: String,
//     pub(crate) name: String,
//     pub(crate) keyword_num: usize,
//     pub(crate) regex_num: usize,
//     pub(crate) phrase_num: usize,
// }

// impl Intent {
//     pub(crate) fn new(intent_name: &str) -> Self {
//         // println!("{}", scru128::new().to_u128());
//         Intent {
//             id: scru128::new_string(),
//             name: String::from(intent_name),
//             keyword_num: 0,
//             regex_num: 0,
//             phrase_num: 0,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct IntentPhraseData {
    pub(crate) id: i64,
    pub(crate) phrase: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct IntentDetail {
    pub(crate) intent_id: String,
    pub(crate) intent_name: String,
    pub(crate) keywords: Vec<String>,
    pub(crate) regexes: Vec<String>,
    phrase_vec_row_id: i64,
    pub(crate) phrases: Vec<IntentPhraseData>,
}

impl IntentDetail {
    pub(crate) fn new(intent_name: &str) -> Self {
        IntentDetail {
            intent_id: scru128::new_string(),
            intent_name: String::from(intent_name),
            keywords: vec![],
            regexes: vec![],
            phrase_vec_row_id: 0,
            phrases: vec![],
        }
    }
}
