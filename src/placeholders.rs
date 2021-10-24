use std::fmt::Debug;
use std::ops::Add;

pub type PlaceholderFn<T, A> = Box<dyn Fn(&T, A) -> String>;

pub trait PlaceholderExpander where Self: 'static {
  type Args: Debug;

  fn get_placeholder(&self, name: &str) -> Option<PlaceholderFn<Self, Self::Args>>;
  fn parse_args(&self, name: &str, args: &[&str]) -> Self::Args;

  fn placeholder_prefix(&self) -> &str { "${" }
  fn placeholder_suffix(&self) -> &str { "}" }

  fn expand_placeholders(&self, str: &str) -> String {
    let mut out = String::with_capacity(str.len() * 2);
    let prefix = self.placeholder_prefix();
    let suffix = self.placeholder_suffix();
    let mut idx = 0;

    while idx < str.len() {
      let remaining = &str[idx..];

      match remaining.find(prefix) {
        Some(start) => {
          let value = &remaining[start + prefix.len()..];

          match value.find(suffix) {
            Some(len) => {
              let placeholder_raw = &value[..len];
              let placeholder_args = placeholder_raw.split("|").collect::<Vec<_>>();
              let placeholder_len = len + suffix.len() + 1;
              let name = placeholder_args.get(0).unwrap_or(&"");
              let args = placeholder_args.get(1..).unwrap_or_default();
              let args = self.parse_args(name, args);

              log::debug!("Placeholder \"{placeholder_raw}\" with arguments {args:?} at index {idx}, +{start} (index {}) from last placeholder", idx + start);

              out.push_str(&str[idx..idx + start]);

              let value = match self.get_placeholder(name) {
                Some(f) => f(self, args),
                None => {
                  log::warn!("Placeholder \"{name}\" does not exit");
                  String::with_capacity(placeholder_len * 2)
                    .add(prefix)
                    .add(name)
                    .add(suffix)
                }
              };

              out.push_str(value.as_str());

              idx += start + placeholder_len;
            }
            None => {
              let last_suffix = remaining.find(suffix).unwrap_or(0);
              let remaining = &remaining[last_suffix..];
              let next_prefix = remaining.find(prefix).unwrap_or(remaining.len());

              log::warn!("Missing suffix for placeholder at index '{}'", idx + start);

              out.push_str(&remaining[last_suffix..next_prefix]);

              idx += start;
            }
          }
        }
        None => {
          out.push_str(remaining);
          idx = str.len()
        }
      }

      idx += 1;
    }

    out
  }
}