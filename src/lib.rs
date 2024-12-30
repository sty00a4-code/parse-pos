use std::{
    fmt::{Debug, Display, Write},
    hash::Hash,
    ops::Range,
    path::Path,
};

/// position span
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub ln: Range<usize>,
    pub col: Range<usize>,
}
/// `T` with a `Position` which is transparent in most cases
pub struct Located<T> {
    pub value: T,
    pub pos: Position,
}
/// `T` with a `Position` and a `Path`
pub struct PathLocated<T> {
    pub value: T,
    pub pos: Position,
    pub path: Box<Path>,
}

impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self {
        Self { ln, col }
    }
    /// extends it's span by another span
    pub fn extend(&mut self, other: &Self) {
        if self.ln.start > other.ln.start {
            self.ln.start = other.ln.start;
        }
        if self.ln.end < other.ln.end {
            self.ln.end = other.ln.end;
        }
        if self.col.start > other.col.start {
            self.col.start = other.col.start;
        }
        if self.col.end < other.col.end {
            self.col.end = other.col.end;
        }
    }
}
impl<T> Located<T> {
    pub fn new(value: T, pos: Position) -> Self {
        Self { value, pos }
    }
    /// creates `Located<T>` with `Position::default()``
    pub fn new_default(value: T) -> Self {
        Self {
            value,
            pos: Position::default(),
        }
    }
    /// maps the inner value to a different value
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Located<U> {
        Located {
            value: f(self.value),
            pos: self.pos,
        }
    }
    pub fn with_path(self, path: Box<Path>) -> PathLocated<T> {
        PathLocated {
            value: self.value,
            pos: self.pos,
            path,
        }
    }
}
impl<T: Default> Located<T> {
    /// only position and `T::default()`
    pub fn default_pos(pos: Position) -> Self {
        Self {
            value: T::default(),
            pos,
        }
    }
}
impl<T: Default> Default for Located<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            pos: Position::default(),
        }
    }
}
impl<T: Debug> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Clone> Clone for Located<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            pos: self.pos.clone(),
        }
    }
}
impl<T: PartialEq> PartialEq for Located<T> {
    /// only the inner values get compared
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: Eq> Eq for Located<T> {}
impl<T: Hash> Hash for Located<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.pos.hash(state);
    }
}
impl<T> PathLocated<T> {
    pub fn new(value: T, pos: Position, path: Box<Path>) -> Self {
        Self { value, pos, path }
    }
    /// maps the inner value to a different value
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> PathLocated<U> {
        PathLocated {
            value: f(self.value),
            pos: self.pos,
            path: self.path,
        }
    }
}
impl<T: Debug> Debug for PathLocated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Display> Display for PathLocated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: Clone> Clone for PathLocated<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            pos: self.pos.clone(),
            path: self.path.clone(),
        }
    }
}
impl<T: PartialEq> PartialEq for PathLocated<T> {
    /// only the inner values get compared
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: Eq> Eq for PathLocated<T> {}
impl<T: Hash> Hash for PathLocated<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.pos.hash(state);
        self.path.hash(state);
    }
}

impl Position {
    pub fn display(&self, f: &mut String, content: &str) -> std::fmt::Result {
        let lines = content.lines().collect::<Vec<&str>>();
        let Some(lines) = lines.get(self.ln.start..=self.ln.end) else {
            writeln!(f, "... code snippet unavailable ...")?;
            return Ok(());
        };
        if lines.is_empty() {
            writeln!(f, "... code snippet unavailable ...")?;
            return Ok(());
        }
        let tab = 4;
        if lines.len() == 1 {
            let line = lines[0];
            let ln = self.ln.start;
            writeln!(f, "{:>tab$}| {line}", ln + 1)?;
            writeln!(
                f,
                "{:>tab$}  {}",
                "",
                line.char_indices()
                    .map(|(col, _)| if self.col.start <= col && self.col.end > col {
                        '~'
                    } else {
                        ' '
                    })
                    .collect::<String>(),
            )?;
        } else {
            let last_ln = lines.len() - 1;
            for (ln, line) in lines.iter().copied().enumerate() {
                writeln!(f, "{:>tab$}| {line}", ln + 1)?;
                if ln == 0 {
                    writeln!(
                        f,
                        "{:>tab$}  {}",
                        "",
                        line.char_indices()
                            .map(|(col, _)| if self.col.start <= col { '~' } else { ' ' })
                            .collect::<String>(),
                    )?;
                } else if ln == last_ln {
                    writeln!(
                        f,
                        "{:>tab$}  {}",
                        "",
                        line.char_indices()
                            .map(|(col, _)| if self.col.end > col { '~' } else { ' ' })
                            .collect::<String>(),
                    )?;
                } else {
                    writeln!(f, "{:>tab$}  {}", "", "~".repeat(line.len()),)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let text = "hello man\n  i like pizza";
        let mut display = String::new();
        Position::new(0..1, 1..5)
            .display(&mut display, text)
            .unwrap();
        println!("{display}");
        panic!();
    }
}
