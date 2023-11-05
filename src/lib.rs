use std::collections::HashMap;

pub fn find_marker_index(str: &str, marker_size: usize) -> usize {

  let mut start_index: usize = 0;
  let char_array: Vec<char> = str.clone().chars().collect();
  let mut character_map: HashMap<char, bool> = HashMap::new();

  for (i, c) in str.chars().enumerate() {
    
    character_map = HashMap::new();

    for n in i..(marker_size + i) {

      if character_map.contains_key(&char_array[n]) {
          break;
      } else {
          character_map.insert(char_array[n], true);
      }
      
      // If count of keys in map is length of MARKER_SIZE, all characters must be unique, and current index is start of sequence
      if character_map.keys().len() == marker_size {
          return i + marker_size;
      }
    }
    

  }

  // Return default start index of 0 if marker not found
  start_index
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_start1() {
    let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    assert_eq!(5, find_marker_index(message, 4));
  }

  #[test]
  fn test_start2() {
    let message = "nppdvjthqldpwncqszvftbrmjlhg";

    assert_eq!(6, find_marker_index(message, 4));
  }

  #[test]
  fn test_start3() {
    let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    assert_eq!(10, find_marker_index(message, 4));
  }

  #[test]
  fn test_start4() {
    let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(11, find_marker_index(message, 4));
  }

  #[test]
  fn test_message1() {
    let message = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    assert_eq!(19, find_marker_index(message, 14));
  }

  #[test]
  fn test_message2() {
    let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    assert_eq!(23, find_marker_index(message, 14));
  }

  #[test]
  fn test_message3() {
    let message = "nppdvjthqldpwncqszvftbrmjlhg";

    assert_eq!(23, find_marker_index(message, 14));
  }

  #[test]
  fn test_message4() {
    let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    assert_eq!(29, find_marker_index(message, 14));
  }

  #[test]
  fn test_message5() {
    let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(26, find_marker_index(message, 14));
  }
}