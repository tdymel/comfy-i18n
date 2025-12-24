#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct Path(Vec<Identifier>);

impl Path {
    pub fn iter(&self) -> impl Iterator<Item = &Identifier> {
        self.0.iter()
    }

    pub fn append(self, id: Identifier) -> Self {
        let mut new_path = Vec::new();
        new_path.extend(self.0);
        new_path.push(id);
        Path(new_path)
    }

    pub fn parent_path(&self) -> Option<Path> {
        if self.0.len() == 1 {
            None
        } else {
            Some(Path(self.0[..(self.0.len() - 1)].to_vec()))
        }
    }
}

impl From<Vec<Identifier>> for Path {
    fn from(value: Vec<Identifier>) -> Self {
        Path(value)
    }
}

impl From<Identifier> for Path {
    fn from(value: Identifier) -> Self {
        Path(vec![value])
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Identifier {
    Field(String),
    Element(usize),
}
