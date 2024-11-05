use std::io::Write;

pub fn find_matchs_word(content: &str, pattern: &str, mut writer: impl Write){
  for line in content.lines() {
      if line.contains(pattern) {
          writeln!(writer, "{}", line).unwrap();
      }
  }
}