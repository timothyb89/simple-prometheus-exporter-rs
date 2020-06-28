use std::fmt;

/// A Prometheus exporter.
///
/// This is a configuration wrapper to allow for optional global labels.
pub struct Exporter {
  pub global_labels: Vec<(String, String)>
}

impl Exporter {
  pub fn new() -> Exporter {
    Exporter {
      global_labels: vec![]
    }
  }

  /// Adds a label to the list of global labels
  pub fn add_global_label<S1, S2>(&mut self, key: S1, value: S2)
  where
    S1: Into<String>,
    S2: Into<String>
  {
    self.global_labels.push((key.into(), value.into()));
  }

  /// Creates a new exporter session
  pub fn session(&self) -> ExporterSession {
    ExporterSession {
      exporter: self,
      buffer: String::new()
    }
  }
}

pub struct ExporterSession<'a> {
  exporter: &'a Exporter,
  buffer: String,
}

impl<'a> fmt::Display for ExporterSession<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.buffer)
  }
}

impl<'a> ExporterSession<'a> {
  /// Formats a single Prometheus metric and appends it to the exporter session
  ///
  /// Note that this function will allow conflicting metric names to be written,
  /// so the caller is responsible for ensuring that each set of (name, labels)
  /// are unique.
  pub fn export<F: Into<f64>>(
    &mut self,
    name: &str,
    value: F,
    labels: &[(&str, &str)]
  ) {
    let mut effective_labels = Vec::new();
    effective_labels.extend(self.exporter.global_labels.iter().cloned());
    effective_labels.extend(labels.iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
    );

    self.buffer.push_str(name);

    if !effective_labels.is_empty() {
      self.buffer.push('{');
      for (i, (k, v)) in effective_labels.iter().enumerate() {
        if i > 0 {
          self.buffer.push(',');
        }

        self.buffer.push_str(k);
        self.buffer.push_str("=\"");
        self.buffer.push_str(v);
        self.buffer.push('"');
      }
      self.buffer.push('}');
    }

    self.buffer.push(' ');
    self.buffer.push_str(&value.into().to_string());
    self.buffer.push('\n');
  }
}

#[macro_export]
macro_rules! export {
  ($session:ident, $name:expr, $value:expr, $($label_key:ident = $label_value:expr),*) => {
    let mut labels: Vec<(&str, std::borrow::Cow<str>)> = Vec::new();
    $(labels.push((stringify!($label_key), std::borrow::Cow::from($label_value)));)*

    let decowed: Vec<(&str, &str)> = labels.iter()
      .map(|(k, v)| (*k, v.as_ref()))
      .collect();

    $session.export($name, $value, &decowed);
  };

  ($session:ident, $name:expr, $value:expr) => {
    $session.export($name, $value, &[]);
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_export() {
    let exporter = Exporter::new();
    let mut s = exporter.session();
    s.export("foo", 1, &[]);
    s.export("bar", 2, &[("baz", "qux")]);

    let string_label = String::from("world");
    s.export("baz", 3, &[("hello", &string_label)]);

    assert_eq!(
      s.to_string(),
      concat!(
        "foo 1\n",
        "bar{baz=\"qux\"} 2\n",
        "baz{hello=\"world\"} 3\n"
      )
    )
  }

  #[test]
  fn test_macro() {
    let exporter = Exporter::new();
    let mut s = exporter.session();
    export!(s, "foo", 1);
    export!(s, "bar", 2, baz = "qux");

    let string_label = String::from("world");
    export!(s, "baz", 3, hello = string_label, a = "b");

    assert_eq!(
      s.to_string(),
      concat!(
        "foo 1\n",
        "bar{baz=\"qux\"} 2\n",
        "baz{hello=\"world\",a=\"b\"} 3\n"
      )
    )
  }

  #[test]
  fn test_global_labels() {
    let mut exporter = Exporter::new();
    exporter.add_global_label("hello", String::from("world"));

    let mut s = exporter.session();
    s.export("foo", 1, &[]);
    s.export("bar", 2, &[("baz", "qux")]);
    export!(s, "baz", 3, lorem = "ipsum");

    assert_eq!(
      s.to_string(),
      concat!(
        "foo{hello=\"world\"} 1\n",
        "bar{hello=\"world\",baz=\"qux\"} 2\n",
        "baz{hello=\"world\",lorem=\"ipsum\"} 3\n"
      )
    )
  }
}
