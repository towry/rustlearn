
pub mod path {
  use std::path::Path;

  pub fn basename(filepath: String) -> String {
      let path = Path::new(&filepath);
      return String::from(path.file_name().unwrap().to_str().unwrap());
  }

  pub fn name_without_ext(filepath: String) -> String {
      let path = Path::new(&filepath);
      return String::from(path.file_stem().unwrap().to_str().unwrap());
  }
}

#[cfg(test)]
mod tests {
  use super::path::*;

  #[test]
  fn test_basename() {
      let path = String::from("test/it/hello.txt");
      let base = basename(path);
      assert_eq!(String::from("hello.txt"), base);
  }

  #[test]
  fn test_name_without_ext() {
      let path = String::from("test/it/hello.txt");
      let base = name_without_ext(path);
      assert_eq!(String::from("hello"), base);
  }
}
