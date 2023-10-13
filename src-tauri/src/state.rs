use std::sync::Mutex;

use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};

macro_rules! deref_tuple_delcare {
    ($ty:path, $name:ident, $value:expr, $block:block) => {
        if let $ty($name) = $value $block
    };
}

#[derive(Debug)]
pub struct PermutationState {
    pub n: Mutex<usize>,
    pub k: Mutex<usize>,
    pub content: Mutex<String>,
    pub permutation_sequence: Mutex<Vec<String>>
}

#[derive(Debug)]
pub struct HanoiState {
    pub rods_number: Mutex<usize>,
    pub plates_number: Mutex<usize>,
    pub fps: Mutex<f64>,
    pub video_path: Mutex<String>,
    pub max_frame: Mutex<usize>,
    pub current_frame: Mutex<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub key: String,
    pub value: Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(usize),
    Double(f64),
    String(String),
    PermContainer(Vec<String>)
}

impl Default for PermutationState {
    fn default() -> Self {
        Self {
            n: Mutex::new(4),
            k: Mutex::new(0),
            content: Mutex::new("ABCD".into()),
            permutation_sequence: Mutex::new(vec![])
        }
    }
}

impl Default for HanoiState {
    fn default() -> Self {
        Self {
            rods_number: Mutex::new(3),
            plates_number: Mutex::new(4),
            fps: Mutex::new(2.0),
            video_path: Mutex::new("".into()),
            max_frame: Mutex::new(0),
            current_frame: Mutex::new(0)
        }
    }
}

pub fn state_setup(handle: AppHandle) {
    let clone = handle.clone();
    let _id = handle.listen_global("global_state_change", move |e| {
        if let Some(payload) = e.payload() {
            let data = serde_json::from_str::<Payload>(payload);
            match data {
                Ok(payload) => state_sync(payload, clone.clone()),
                _ => println!("deserialize failed")
            }
        }
    });
}

fn state_sync(payload: Payload, handle: AppHandle) {
    let permutation_state = handle.state::<PermutationState>();
    let hanoi_state = handle.state::<HanoiState>();
    let clone = payload.clone();
    match &*payload.key.to_lowercase() {
        "n" => deref_tuple_delcare!(Value::Number, value, payload.value, {
            *permutation_state.n.lock().unwrap() = value;
        }),
        "k" => deref_tuple_delcare!(Value::Number, value, payload.value, {
            *permutation_state.k.lock().unwrap() = value;
        }),
        "content" => deref_tuple_delcare!(Value::String, value, payload.value, {
            *permutation_state.content.lock().unwrap() = value;
        }),
        "rods" => deref_tuple_delcare!(Value::Number, value, payload.value, {
            *hanoi_state.rods_number.lock().unwrap() = value;
        }),
        "plates" => deref_tuple_delcare!(Value::Number, value, payload.value, {
            *hanoi_state.plates_number.lock().unwrap() = value;
        }),
        "fps" => deref_tuple_delcare!(Value::Number, value, payload.value, {
            *hanoi_state.fps.lock().unwrap() = value as f64;
        }),
        _ => return
    }

    handle.emit_all("global_state_sync", clone).unwrap();
}