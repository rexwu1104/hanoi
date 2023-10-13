use anyhow::Ok;
use anyhow::Result;
use opencv::prelude::*;
use opencv::imgproc;
use opencv::core;

use super::sturcts::StackPlate;
use super::{sturcts::{Hanoi, Stack, ColorGenerator}, traits::Renderable};

impl Hanoi {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            columns: -1,
            renderables: vec![]
        }
    }

    pub fn setup(&mut self, columns: i32, data: Vec<Vec<i32>>) -> Result<()> {
        self.columns = columns;

        let plates = data.iter().map(|v| v.len() as i32).sum();
        let x_step = self.width / (self.columns + 1);
        let y = self.height / 9 * 7;
        let y_height = self.height / 9 * 5;
        let colors = ColorGenerator::new(plates as usize).collect::<Vec<core::VecN<f64, 4>>>();
        self.renderables = (0..columns)
            .zip(data.iter())
            .map(move |(idx, datum)| {
                Box::new(Stack::new(
                    x_step * (idx as i32 + 1),
                    y,
                    x_step,
                    y_height,
                    datum.clone(),
                    colors.clone(),
                    plates
                )) as Box<dyn Renderable>
            })
            .collect();
        Ok(())
    }

    pub fn init_mat(&self) -> Result<Mat> {
        Ok(Mat::new_rows_cols_with_default(self.height, self.width, core::CV_8UC3, core::VecN([255., 255., 255., 0.]))?)
    }
}

impl Renderable for Hanoi {
    fn render(&self, mat: &mut Mat) -> Result<()> {
        Ok(self.renderables.iter().for_each(|renderable| renderable.render(mat).unwrap()))
    }
}

impl Stack {
    pub fn new(x: i32, y: i32, width: i32, height: i32, data: Vec<i32>, color: Vec<core::VecN<f64, 4>>, max: i32) -> Self {
        let y_step = height / (max + 1);
        let renderables: Vec<Box<dyn Renderable>> = data.into_iter().enumerate().map(move |(i, n)| {
            (
                y - y_step * i as i32,
                width / (max + 1) * (n + 1),
                color[n as usize]
            )
        })
        .map(|(y, width, color)| Box::new(StackPlate::new(x, y, width, y_step, color)) as Box<dyn Renderable>)
        .collect();
        Self {
            start_x: x,
            start_y: y,
            width,
            height,
            renderables
        }
    }
}

impl Renderable for Stack {
    fn render(&self, mat: &mut Mat) -> Result<()> {
        imgproc::rectangle(
            mat,
            core::Rect::from_points(core::Point::new(self.start_x - 10, self.start_y - self.height), core::Point::new(self.start_x + 10, self.start_y)),
            core::VecN([0., 0., 0., 0.]),
            -1,
            imgproc::LINE_8,
            0
        )?;
        Ok(self.renderables.iter().for_each(|renderable| renderable.render(mat).unwrap()))
    }
}

impl StackPlate {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: core::VecN<f64, 4>) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color
        }
    }
}

impl Renderable for StackPlate {
    fn render(&self, mat: &mut Mat) -> Result<()> {
        imgproc::rectangle(
            mat,
            core::Rect::from_points(
                core::Point::new(self.x - self.width / 2, self.y - self.height),
                core::Point::new(self.x + self.width / 2, self.y)
            ),
            self.color,
            -1,
            imgproc::LINE_8,
            0
        )?;
        Ok(())
    }
}

impl ColorGenerator {
    pub fn new(times: usize) -> Self {
        let mut n = 1;
        loop {
            if usize::pow(n, 3) >= times {
                break;
            } else {
                n += 1;
            }
        }

        Self(n, [0, 0, 0], times, 0)
    }
}

impl Iterator for ColorGenerator {
    type Item = core::VecN<f64, 4>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut r_n = self.1[0];
        let mut g_n = self.1[1];
        let mut b_n = self.1[2];

        b_n += 1;
        g_n += b_n / self.0;
        b_n %= self.0;
        r_n += g_n / self.0;
        g_n %= self.0;

        self.1 = [r_n, g_n, b_n];
        if r_n == self.0 - 1 && g_n == self.0 - 1 && b_n == self.0 - 1 {
            None
        } else {
            Some(core::VecN([
                255. / self.0 as f64 * r_n as f64,
                255. / self.0 as f64 * g_n as f64,
                255. / self.0 as f64 * b_n as f64,
                0.
            ]))
        }
    }
}