use anyhow::Result;
use tauri::{AppHandle, Manager};

use crate::state::{PermutationState, Payload, Value};

pub fn permutation_string(mut content: String, k: usize, n: usize, count: &mut usize, handle: AppHandle) -> Result<()> {
    if k >= n {
        return Ok(())
    }
    
    if k == n - 1 {
        *count += 1;
        let permutation_state = handle.state::<PermutationState>();
        permutation_state.permutation_sequence.lock().unwrap().push(format!("{content} [{count}]"));

        handle.emit_all("global_state_sync", Payload {
            key: "sequence".into(),
            value: Value::PermContainer(permutation_state.permutation_sequence.lock().unwrap().clone())
        })?;
    } else {
        for i in k..n {
            content = swap(content, i, k);
            permutation_string(content.clone(), k + 1, n, count, handle.clone())?;
            content = swap(content, i, k);
        }
    }

    Ok(())
}

fn swap(content: String, a: usize, b: usize) -> String {
    let mut chars: Vec<_> = content.chars().collect();
    chars.as_mut_slice().swap(a, b);
    chars.into_iter().collect()
}