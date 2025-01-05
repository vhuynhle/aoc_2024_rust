fn main() {
    let mut encoded_disk = String::new();
    std::io::stdin()
        .read_line(&mut encoded_disk)
        .expect("Failed to read input");
    let encoded_disk = encoded_disk.trim();

    let (mut disk, mut free_blocks) = decode_disk(encoded_disk);

    let compacted_disk = compact(disk.clone());
    println!("Part 1: {}", checksum(&compacted_disk));

    defrag(&mut disk, &mut free_blocks);
    println!("Part 2: {}", checksum(&disk));
}

#[derive(Debug)]
struct Block {
    offset: usize,
    length: usize,
}

const FREE_BLOCK_MARKER: usize = usize::MAX;

fn compact(mut disk: Vec<usize>) -> Vec<usize> {
    // Move file blocks from the end of the disk to free blocks at the beginning
    let mut i = find_free_cell(&disk, 0);
    let mut j = find_file_cell(&disk, disk.len() - 1);
    loop {
        if i.is_none() || j.is_none() || i.unwrap() >= j.unwrap() {
            break;
        }

        let i_val = i.unwrap();
        let j_val = j.unwrap();
        disk.swap(i_val, j_val);

        i = find_free_cell(&disk, i_val + 1);
        j = find_file_cell(&disk, j_val - 1);
    }

    disk
}

fn decode_disk(encoded_disk: &str) -> (Vec<usize>, Vec<Block>) {
    let encoded_disk = encoded_disk.as_bytes();
    let mut disk = vec![];
    let mut free_blocks = vec![];

    for i in 0..encoded_disk.len() / 2 {
        let file_block_length = digit_to_num(encoded_disk[i << 1]);
        disk.append(&mut vec![i; file_block_length]);

        let free_block_length = digit_to_num(encoded_disk[(i << 1) | 1]);
        if free_block_length > 0 {
            free_blocks.push(Block {
                offset: disk.len(),
                length: free_block_length,
            });
            disk.append(&mut vec![FREE_BLOCK_MARKER; free_block_length]);
        }
    }

    if encoded_disk.len() % 2 == 1 {
        let file_block_length = digit_to_num(encoded_disk[encoded_disk.len() - 1]);
        disk.append(&mut vec![encoded_disk.len() / 2; file_block_length]);
    }

    (disk, free_blocks)
}

fn digit_to_num(digit: u8) -> usize {
    (digit - b'0') as usize
}

fn find_free_cell(disk: &[usize], start_pos: usize) -> Option<usize> {
    (start_pos..disk.len()).find(|&i| disk[i] == FREE_BLOCK_MARKER)
}

fn find_file_cell(disk: &[usize], end_pos: usize) -> Option<usize> {
    (0..=end_pos).rev().find(|&i| disk[i] != FREE_BLOCK_MARKER)
}

fn checksum(disk: &[usize]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(index, value)| {
            if *value == FREE_BLOCK_MARKER {
                0
            } else {
                index * value
            }
        })
        .sum()
}

fn defrag(disk: &mut [usize], free_blocks: &mut Vec<Block>) {
    let mut file_end_pos = disk.len() - 1;
    while let Some(file_block) = find_file_block(disk, file_end_pos) {
        if let Some(free_block_index) =
            find_free_block(free_blocks, file_block.offset, file_block.length)
        {
            let free_block = &mut free_blocks[free_block_index];

            // Update the disk
            let file_id = disk[file_block.offset];
            disk[free_block.offset..(free_block.offset + file_block.length)].fill(file_id);
            disk[file_block.offset..(file_block.offset + file_block.length)]
                .fill(FREE_BLOCK_MARKER);

            // Update the free list
            free_block.offset += file_block.length;
            free_block.length -= file_block.length;
            if free_block.length == 0 {
                free_blocks.remove(free_block_index);
            }

            // Optional: Add a free block left by the moved file
            // Append Block {file_block.offset, file_block.length}
            // Sort by offset
            // Merge neighboring blocks
        }

        // Move to the next file
        file_end_pos = file_block.offset.wrapping_sub(1);
    }
}

fn find_file_block(disk: &[usize], mut end_pos: usize) -> Option<Block> {
    while end_pos < disk.len() && disk[end_pos] == FREE_BLOCK_MARKER {
        end_pos = end_pos.wrapping_sub(1);
    }
    if end_pos >= disk.len() {
        return None; // Wrapped around
    }

    let mut start_pos = end_pos;
    while start_pos < disk.len() && disk[start_pos] == disk[end_pos] {
        start_pos = start_pos.wrapping_sub(1);
    }
    start_pos = start_pos.wrapping_add(1);

    Some(Block {
        offset: start_pos,
        length: end_pos - start_pos + 1,
    })
}

fn find_free_block(free_blocks: &[Block], offset: usize, size: usize) -> Option<usize> {
    for (index, block) in free_blocks.iter().enumerate() {
        if block.offset >= offset {
            return None;
        }
        if block.length >= size {
            return Some(index);
        }
    }
    None
}
