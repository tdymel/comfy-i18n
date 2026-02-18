#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct Path(Vec<Identifier>);

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Identifier> {
        self.0.iter()
    }

    pub fn append(mut self, id: Identifier) -> Self {
        self.0.push(id);
        self
    }

    pub fn prepend(mut self, id: Identifier) -> Self {
        self.0.insert(0, id);
        self
    }

    pub fn remove(mut self, index: usize) -> Option<Self> {
        if self.0.len() > index {
            self.0.remove(index);
            Some(self)
        } else {
            None
        }
    }

    pub fn parent_path(&self) -> Option<Path> {
        if self.0.len() == 1 {
            None
        } else {
            Some(Path(self.0[..(self.0.len() - 1)].to_vec()))
        }
    }

    pub fn root(&self) -> Identifier {
        self.0.first().unwrap().clone()
    }

    pub fn map<B, F>(&self, mapper_fn: F) -> Vec<B>
    where
        F: FnMut(&Identifier) -> B,
    {
        self.0.iter().map(mapper_fn).collect()
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

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Field(field) => f.write_str(field),
            Identifier::Element(idx) => f.write_str(&idx.to_string()),
        }
    }
}
