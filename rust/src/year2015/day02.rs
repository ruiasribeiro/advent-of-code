use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::utils::is_full_input;

struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

impl PresentBox {
    fn get_surface_area(&self) -> u32 {
        let l = self.length;
        let w = self.width;
        let h = self.height;

        (2 * l * w) + (2 * w * h) + (2 * h * l)
    }

    fn get_area_of_smallest_side(&self) -> u32 {
        let l = self.length;
        let w = self.width;
        let h = self.height;

        *[l * w, w * h, h * l].iter().min().unwrap()
    }

    fn calculate_necessary_wrapping_paper(&self) -> u32 {
        self.get_surface_area() + self.get_area_of_smallest_side()
    }

    fn get_smallest_side_perimeter(&self) -> u32 {
        let l = self.length;
        let w = self.width;
        let h = self.height;

        *[2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l]
            .iter()
            .min()
            .unwrap()
    }

    fn get_volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn calculate_necessary_ribbon_length(&self) -> u32 {
        self.get_smallest_side_perimeter() + self.get_volume()
    }
}

impl TryFrom<String> for PresentBox {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split('x').collect::<Vec<_>>();

        anyhow::ensure!(parts.len() == 3, "malformed input; expected LxWxH");

        let length = parts[0].parse()?;
        let width = parts[1].parse()?;
        let height = parts[2].parse()?;

        Ok(PresentBox {
            length,
            width,
            height,
        })
    }
}

pub fn solve_part1(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| {
            PresentBox::try_from(line.unwrap())
                .unwrap()
                .calculate_necessary_wrapping_paper()
        })
        .sum::<u32>()
        .to_string()
}

pub fn solve_part2(path: &Path) -> String {
    if !is_full_input(path) {
        return String::from("N/A");
    }

    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| {
            PresentBox::try_from(line.unwrap())
                .unwrap()
                .calculate_necessary_ribbon_length()
        })
        .sum::<u32>()
        .to_string()
}
