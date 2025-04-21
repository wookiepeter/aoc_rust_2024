fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct BlockInfo {
    length: usize,
    /// None if block is empty
    id: Option<usize>,
}

fn process(input: &str) -> String {
    let chars = input.trim().chars();

    let (block_infos, compressed_length) = parse_blocks(input, chars);
    let compressed_disk = create_compressed_disk(block_infos, compressed_length);

    let result = compressed_disk
        .iter()
        .enumerate()
        .fold(0u128, |acc, (i, file_id)| acc + (i * file_id) as u128);

    result.to_string()
}

fn create_compressed_disk(block_infos: Vec<BlockInfo>, compressed_length: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(compressed_length);
    let mut reverse_data = block_infos.iter().rev().flat_map(|block| block.into_iter());

    for block in &block_infos {
        match block.id {
            Some(id) => {
                for _ in 0..block.length {
                    result.push(id);
                }
            }
            None => {
                for _ in 0..block.length {
                    // We know the compressed length -> reverse data shouldn't run out
                    result.push(*reverse_data.next().unwrap())
                }
            }
        }
        if result.len() > compressed_length {
            break;
        }
    }

    // shorten result to remove potential duplicate elements from the last processed block
    result.truncate(compressed_length);

    result
}

fn parse_blocks(input: &str, chars: std::str::Chars<'_>) -> (Vec<BlockInfo>, usize) {
    let mut block_infos: Vec<BlockInfo> = Vec::with_capacity(input.len());
    let mut compressed_length = 0;

    for (index, c) in chars.enumerate() {
        let length = c.to_digit(10).unwrap() as usize;
        let id = match index % 2 {
            0 => {
                compressed_length += length;
                Some(index / 2)
            }
            _ => None,
        };

        block_infos.push(BlockInfo { length, id })
    }
    (block_infos, compressed_length)
}

pub struct BlockInfoIter<'a> {
    info: &'a BlockInfo,
    index: usize,
}

impl<'a> Iterator for BlockInfoIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.info.length {
            return None;
        }
        self.index += 1;
        self.info.id.as_ref()
    }
}

impl<'a> IntoIterator for &'a BlockInfo {
    type Item = &'a usize;
    type IntoIter = BlockInfoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockInfoIter {
            info: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process("2333133121414131402");
        assert_eq!(result, "1928".to_string())
    }
}
