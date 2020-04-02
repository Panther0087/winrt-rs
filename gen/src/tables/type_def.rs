use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct TypeDef(pub Row);

impl TypeDef {
    pub fn flags(&self, reader: &Reader) -> TypeFlags {
        TypeFlags(reader.u32(self.0, 0))
    }

    pub fn name<'a>(&self, reader: &'a Reader) -> (&'a str, &'a str) {
        (reader.str(self.0, 2), reader.str(self.0, 1))
    }

    pub fn extends(&self, reader: &Reader) -> TypeDefOrRef {
        reader.decode(self.0, 3)
    }

    pub fn fields(&self, reader: &Reader) -> impl Iterator<Item = Field> {
        reader
            .list(self.0, TABLE_FIELD as u16, 4)
            .map(|row| Field(row))
    }

    pub fn methods(&self, reader: &Reader) -> impl Iterator<Item = MethodDef> {
        reader
            .list(self.0, TABLE_METHODDEF as u16, 5)
            .map(|row| MethodDef(row))
    }

    pub fn generics(&self, reader: &Reader) -> impl Iterator<Item = GenericParam> {
        reader
            .equal_range(
                self.0.file,
                TABLE_GENERICPARAM,
                2,
                TypeOrMethodDef::TypeDef(*self).encode(),
            )
            .map(|row| GenericParam(row))
    }

    pub fn interfaces(&self, reader: &Reader) -> impl Iterator<Item = InterfaceImpl> {
        reader
            .equal_range(self.0.file, TABLE_INTERFACEIMPL, 0, self.0.row + 1)
            .map(|row| InterfaceImpl(row))
    }

    pub fn attributes(&self, reader: &Reader) -> impl Iterator<Item = Attribute> {
        reader
            .equal_range(
                self.0.file,
                TABLE_CUSTOMATTRIBUTE,
                0,
                HasAttribute::TypeDef(*self).encode(),
            )
            .map(|row| Attribute(row))
    }

    pub fn has_attribute(&self, reader: &Reader, name: (&str, &str)) -> bool {
        self.attributes(reader)
            .any(|attribute| attribute.name(reader) == name)
    }

    pub fn attribute(&self, reader: &Reader, name: (&str, &str)) -> Attribute {
        self.attributes(reader)
            .find(|attribute| attribute.name(reader) == name)
            .unwrap()
    }

    pub fn ignore(&self, reader: &Reader) -> bool {
        let flags = self.flags(reader);

        if !flags.windows_runtime() {
            true
        } else if flags.interface() {
            false
        } else {
            match self.extends(reader).name(reader) {
                ("System", "ValueType") => self.has_attribute(
                    reader,
                    ("Windows.Foundation.Metadata", "ApiContractAttribute"),
                ),
                ("System", "Attribute") => true,
                _ => false,
            }
        }
    }

    pub fn category(&self, reader: &Reader) -> TypeCategory {
        debug_assert!(self.flags(reader).windows_runtime());

        if self.flags(reader).interface() {
            TypeCategory::Interface
        } else {
            match self.extends(reader).name(reader) {
                ("System", "Enum") => TypeCategory::Enum,
                ("System", "MulticastDelegate") => TypeCategory::Delegate,
                ("System", "ValueType") => TypeCategory::Struct,
                _ => TypeCategory::Class,
            }
        }
    }

    pub fn info(&self, reader: &Reader) -> Type {
        Type::from_type_def(reader, *self)
    }
}
