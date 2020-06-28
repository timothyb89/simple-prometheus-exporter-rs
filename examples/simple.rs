use simple_prometheus_exporter::{Exporter, export};

fn main() {
  let exporter = Exporter::new();
  let mut s = exporter.session();
  export!(s, "example", 1.23, foo = "bar");

  println!("{}", s);
}
