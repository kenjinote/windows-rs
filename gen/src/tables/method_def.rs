use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef(pub Row);

impl MethodDef {
    pub fn flags(&self, reader: &Reader) -> MethodFlags {
        MethodFlags(reader.u32(self.0, 2))
    }

    pub fn parent(&self, reader: &Reader) -> TypeDef {
        TypeDef(reader.upper_bound(self.0.file, TABLE_TYPEDEF as u16, 6, self.0.row))
    }

    pub fn params(&self, reader: &Reader) -> impl Iterator<Item = Param> {
        reader
            .list(self.0, TABLE_PARAM as u16, 5)
            .map(|row| Param(row))
    }

    pub fn name<'a>(&self, reader: &'a Reader) -> &'a str {
        reader.str(self.0, 3)
    }

    pub fn sig<'a>(&self, reader: &'a Reader) -> Blob<'a> {
        reader.blob(self.0, 4)
    }

    pub fn category(&self, reader: &Reader) -> MethodCategory {
        if self.flags(reader).special() {
            let name = self.name(reader);

            if name.starts_with("get") {
                MethodCategory::Get
            } else if name.starts_with("put") {
                MethodCategory::Set
            } else if name.starts_with("add") {
                MethodCategory::Add
            } else if name.starts_with("remove") {
                MethodCategory::Remove
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                MethodCategory::Normal
            }
        } else {
            MethodCategory::Normal
        }
    }

    pub fn attributes(&self, reader: &Reader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file,
                TABLE_CUSTOMATTRIBUTE,
                0,
                HasAttribute::MethodDef(*self).encode(),
            )
            .map(|row| Attribute(row))
    }
}