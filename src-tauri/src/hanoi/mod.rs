use std::{env::var, path::{Path, PathBuf}, fs::create_dir};

use anyhow::{Result, Error, Ok};
use opencv::{videoio::VideoWriter, prelude::VideoWriterTrait, core};
use sturcts::Hanoi;
use tauri::{AppHandle, Manager};
use traits::Renderable;

use crate::state::{HanoiState, Payload, Value};

mod sturcts;
mod traits;
mod impls;

pub fn generate_hanoi_animation(rods_number: usize, plates_number: usize, fps: f64, handle: AppHandle) -> Result<()> {
    println!("{fps}");
    let mut rods = vec![vec![]; rods_number];
    rods[0] = (0..plates_number as i32).rev().collect();

    let mut temp_dir = PathBuf::from(var("TEMP").or_else(|_| Ok("./TEMP".to_string())).unwrap());
    if temp_dir.to_str().unwrap() == "./TEMP" && !Path::new(&temp_dir).is_dir() {
        create_dir(&temp_dir)?;
    }

    temp_dir.push("hanoi.mp4");
    let filename = temp_dir.as_path().to_str().unwrap();
    let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v')?;
    let mut writer = VideoWriter::new(
        filename,
        fourcc,
        fps,
        core::Size_ { width: 1920, height: 1080 },
        true
    )?;

    plot(&rods, &mut writer)?;
    let hanoi_state = handle.state::<HanoiState>();
    let max_frame = hanoi_times(rods_number, plates_number);

    *hanoi_state.max_frame.lock().unwrap() = max_frame;
    handle.emit_all("global_state_sync", Payload {
        key: "max_frame".into(),
        value: Value::Number(max_frame)
    })?;

    *hanoi_state.current_frame.lock().unwrap() = 0;
    handle.emit_all("global_state_sync", Payload {
        key: "current_frame".into(),
        value: Value::Number(0)
    })?;

    hanoi(plates_number as i32, (0..rods_number as i32).collect(), &mut rods, &mut writer, handle.clone())?;
    *hanoi_state.video_path.lock().unwrap() = filename.to_string();
    handle.emit_all("global_state_sync", Payload {
        key: "video_path".into(),
        value: Value::String(filename.to_string())
    })?;

    Ok(())
}

fn hanoi_times(rods_number: usize, plates_number: usize) -> usize {
    if plates_number < 1 {
        0
    } else if plates_number == 1 {
        1
    } else {
        2 * hanoi_times(rods_number, plates_number - (rods_number - 2)) + 2 * (rods_number - 2) - 1
    }
}

fn hanoi(n: i32, mut column: Vec<i32>, rods: &mut Vec<Vec<i32>>, writer: &mut VideoWriter, handle: AppHandle) -> Result<()> {
    if column.len() < 3 {
        return Err(Error::msg("column cannot smaller than 3"))
    } else if n < 1 {
        return Ok(());
    }

    let len = column.len();
    if n == 1 {
        // println!("{:?}\n{:?}", column, rods);
        let value = rods[column[0] as usize].pop().unwrap();
        rods[column[len - 1] as usize].push(value);

        // (0..6).for_each(|_| plot(&rods, writer).unwrap());
        let hanoi_state = handle.state::<HanoiState>();
        let mut current_frame = hanoi_state.current_frame.lock().unwrap();
        *current_frame += 1;
        handle.emit_all("global_state_sync", Payload {
            key: "current_frame".into(),
            value: Value::Number(current_frame.to_owned())
        })?;

        plot(&rods, writer)?;
    } else {
        // println!("{}", n - (len as i32 - 2));
        column.as_mut_slice().swap(len - 2, len - 1);
        hanoi(n - (len as i32 - 2), column.clone(), rods, writer, handle.clone())?;
        for i in 1..len - 2 {
            column.as_mut_slice().swap(i, len - 1);
            hanoi(1, column.clone(), rods, writer, handle.clone())?;
        }

        column.as_mut_slice().swap(len - 2, len - 1);
        hanoi(1, column.clone(), rods, writer, handle.clone())?;
        for i in (2..len - 1).rev() {
            column.as_mut_slice().swap(i, 0);
            hanoi(1, column.clone(), rods, writer, handle.clone())?;
        }

        column.swap(0, 1);
        hanoi(n - (len as i32 - 2), column, rods, writer, handle.clone())?;
    }

    Ok(())
}

fn plot(rods: &Vec<Vec<i32>>, writer: &mut VideoWriter) -> Result<()> {
    let mut hanoi = Hanoi::new(1920, 1080);
    hanoi.setup(rods.len() as i32, rods.clone())?;

    let mut mat = hanoi.init_mat()?;
    hanoi.render(&mut mat)?;

    writer.write(&mat)?;
    Ok(())
}