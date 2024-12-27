use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
    path::Path,
};

type Disk = Vec<Option<usize>>;

fn parse_file(path: &Path) -> Disk {
    let file = File::open(path).unwrap();

    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut reading_file = false;
    let mut id_counter = 0_usize;

    line.chars()
        .flat_map(|c| {
            reading_file = !reading_file;

            let size = c.to_digit(10).unwrap().try_into().unwrap();

            if reading_file {
                let id = id_counter;
                id_counter += 1;

                repeat(Some(id)).take(size)
            } else {
                repeat(None).take(size)
            }
        })
        .collect::<Vec<_>>()
}

pub fn solve_part1(path: &Path) -> String {
    let mut disk = parse_file(path);

    let mut index = 0;
    let mut last_index = disk.len() - 1;

    while index < last_index {
        if disk[index].is_none() {
            disk.swap(index, last_index);
        }

        while disk[last_index].is_none() {
            last_index -= 1;
        }

        index += 1;
    }

    (0..=last_index)
        .map(|i| i * disk[i].unwrap())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
enum Block {
    Empty { size: usize },
    File { size: usize, id: usize },
}

type DiskV2 = Vec<Block>;

fn parse_file_v2(path: &Path) -> DiskV2 {
    let file = File::open(path).unwrap();

    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut reading_file = false;
    let mut id_counter = 0_usize;

    line.chars()
        .map(|c| {
            reading_file = !reading_file;

            let size = c.to_digit(10).unwrap().try_into().unwrap();

            if reading_file {
                let id = id_counter;
                id_counter += 1;

                Block::File { size, id }
            } else {
                Block::Empty { size }
            }
        })
        .collect::<Vec<_>>()
}

fn compact_empty_blocks(disk: &mut Vec<Block>) {
    let mut curr = 0;
    let mut next = 1;

    while next < disk.len() {
        if let (Block::Empty { size: curr_size }, Block::Empty { size: next_size }) =
            (&disk[curr], &disk[next])
        {
            disk[curr] = Block::Empty {
                size: curr_size + next_size,
            };

            disk.remove(next);
        } else {
            curr += 1;
            next += 1;
        }
    }
}

pub fn solve_part2(path: &Path) -> String {
    let mut disk = parse_file_v2(path);

    let mut last_index = disk.len() - 1;

    while last_index > 0 {
        if let Block::File {
            size: file_size, ..
        } = disk[last_index]
        {
            let mut index = 1;

            while index < last_index {
                if let Block::Empty { size: empty_size } = disk[index] {
                    if empty_size >= file_size {
                        disk.swap(index, last_index);

                        if empty_size > file_size {
                            disk[last_index] = Block::Empty { size: file_size };

                            disk.insert(
                                index + 1,
                                Block::Empty {
                                    size: empty_size - file_size,
                                },
                            );
                        }

                        compact_empty_blocks(&mut disk);

                        break;
                    }
                }

                index += 1;
            }
        }

        last_index -= 1;
    }

    let mut simple_disk: Disk = vec![];

    for block in disk {
        match block {
            Block::Empty { size } => simple_disk.extend(repeat(None).take(size)),
            Block::File { size, id } => simple_disk.extend(repeat(Some(id)).take(size)),
        };
    }

    (0..simple_disk.len())
        .map(|i| i * simple_disk[i].unwrap_or(0))
        .sum::<usize>()
        .to_string()
}
