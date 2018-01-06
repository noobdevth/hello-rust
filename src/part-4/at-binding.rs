enum Packet {
  Header { mode: u32 }
}

fn header_type(header: Packet) {
  match header {
    Packet::Header { mode: 0...2 } => {
      println!("Genesis Header (0-2)");
    },
    Packet::Header { mode: mode @ 3...10 } => {
      println!("High Priority Header (3-10): {}", mode);
    },
    Packet::Header { mode } => {
      println!("Normal Header (11+): {}", mode);
    }
  }
}

fn main() {
  header_type(Packet::Header { mode: 0 });
  header_type(Packet::Header { mode: 3 });
  header_type(Packet::Header { mode: 11 });
}
