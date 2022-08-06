use crate::graph::hash_table::HashTable;
use std::fs::File;
use std::io;
use std::io::BufRead;

// Input format described here: https://pacechallenge.org/2022/tracks/
pub struct Metis {
  filename: String,
  edges: Vec<(u32, u32)>,
  edge_count: usize,
  vertex_count: usize,
}

impl Metis {
  pub fn new(file: &str) -> Self {
    Self {
      filename: file.to_string(),
      edges: vec![],
      edge_count: 0,
      vertex_count: 0,
    }
  }

  // Based on: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  // Returns an Iterator to the Reader of the lines of the file.
  fn lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&self.filename)?;
    Ok(io::BufReader::new(file).lines())
  }

  pub fn parse(&mut self) {
    if let Ok(lines) = self.lines() {
      let mut idx = 0;
      for line in lines.flatten() {
        if idx != 0 {
          idx = self.parse_content_line(line, idx);
        } else {
          self.parse_header_line(line);
          idx += 1;
        }
      }
    }
  }

  fn parse_content_line(&mut self, line: String, idx: u32) -> u32 {
    if line.starts_with('%') {
      return idx;
    }

    for edge in line.split_whitespace() {
      let target = edge.parse::<u32>().unwrap();
      self.edges.push((idx, target));
    }

    idx + 1
  }

  fn parse_header_line(&mut self, header: String) {
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() >= 2 {
      self.vertex_count = parts[0].parse::<usize>().unwrap();
      self.edge_count = parts[1].parse::<usize>().unwrap();
    }
  }

  pub fn edges(&self) -> &[(u32, u32)] {
    self.edges.as_slice()
  }

  pub fn vertices(&self) -> usize {
    self.vertex_count
  }
}

pub fn graph_from_file(filename: &str) -> HashTable {
  let mut parser = Metis::new(filename);
  parser.parse();
  HashTable::from_edges(parser.vertices(), parser.edges())
}

#[cfg(test)]
mod tests {
  use crate::tools::metis::Metis;

  #[test]
  fn can_parse_e_001() {
    can_parse_metis_file("test/resources/exact/e_001", 512, 651);
  }

  #[test]
  fn can_parse_e_001_with_comments() {
    can_parse_metis_file("test/resources/exact/e_001_with_comments", 512, 651);
  }

  fn can_parse_metis_file(path: &str, expected_vertex_count: usize, expected_edge_count: usize) {
    let mut e_001 = Metis::new(path);
    e_001.parse();

    assert_eq!(e_001.vertex_count, expected_vertex_count);
    assert_eq!(e_001.edge_count, expected_edge_count);
    assert_eq!(e_001.edges.len(), e_001.edge_count);

    println!("{:?}", e_001.edges);
  }
}
