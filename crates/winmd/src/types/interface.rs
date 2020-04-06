use crate::codes::*;
use crate::tables::*;
use crate::types::*;
use crate::Reader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub methods: Vec<Method>,
    // pub default: bool,
    // pub exclusive: bool,
    // pub constructors: bool,
    // pub statics: bool,
    // pub overrides: bool,
    pub interfaces: Vec<Interface>,
}

impl Interface {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.methods.iter().flat_map(|m| m.dependencies()))
            .collect()
    }

    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let guid = TypeGuid::from_args(
            def.attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
                .args(reader),
        );
        let methods = def
            .methods(reader)
            .map(|method| Method::from_method_def(reader, method, &name.generics))
            .collect();
        let interfaces = Vec::new();
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    fn from_type_ref(reader: &Reader, type_ref: TypeRef) -> Self {
        Self::from_type_def(reader, type_ref.resolve(reader))
    }

    fn from_type_spec(reader: &Reader, spec: TypeSpec) -> Self {
        let name = TypeName::from_type_spec(reader, spec);
        let guid = TypeGuid::new(); // TODO: Generate generic guid specialization
        let methods = Vec::new();
        let interfaces = Vec::new();
        Self {
            name,
            guid,
            methods,
            interfaces,
        }
    }

    fn from_type_def_or_ref(reader: &Reader, code: TypeDefOrRef) -> Self {
        match code {
            TypeDefOrRef::TypeDef(value) => Self::from_type_def(reader, value),
            TypeDefOrRef::TypeRef(value) => Self::from_type_ref(reader, value),
            TypeDefOrRef::TypeSpec(value) => Self::from_type_spec(reader, value),
        }
    }

    pub fn from_interface_impl(reader: &Reader, key: InterfaceImpl) -> Self {
        // TODO: flip default/exclusive/overridable bits as needed
        Self::from_type_def_or_ref(reader, key.interface(reader))
    }

    pub fn to_stream(&self) -> TokenStream {
        let name = self.name.ident();
        let phantoms = self.name.phantoms();
        let constraints = self.name.constraints();
        let projected_methods = TokenStream::new();

        quote! {
            #[repr(C)]
            #[derive(Default, Clone)]
            pub struct #name where #constraints {
                ptr: winrt::IUnknown,
                #phantoms
            }
            impl<#constraints> #name {
                #projected_methods
            }
        }
    }
}

#[test]
fn can_read_interface_from_reader() {
    let reader = &Reader::from_os();
    let def = reader.resolve(("Windows.Foundation", "IStringable"));
    let t = def.into_type(reader);
    let name = t.name();
    assert!(name.namespace == "Windows.Foundation");
    assert!(name.name == "IStringable");
    assert!(name.generics.is_empty());
    assert!(name.def == def);

    let t = match t {
        Type::Interface(t) => t,
        _ => panic!("Wrong type"),
    };

    assert!(t.methods.len() == 1);
    let method = &t.methods[0];
    assert!(method.name == "ToString");
    assert!(method.params.is_empty());
    let param = method.return_type.as_ref().unwrap();
    assert!(param.kind == TypeKind::String);

    let guid = &t.guid;
    assert!(guid.0[0] == GuidConstant::U32(0x96369F54));
    assert!(guid.0[1] == GuidConstant::U16(0x8EB6));
    assert!(guid.0[2] == GuidConstant::U16(0x48F0));
    assert!(guid.0[3] == GuidConstant::U8(0xAB));
    assert!(guid.0[4] == GuidConstant::U8(0xCE));
    assert!(guid.0[5] == GuidConstant::U8(0xC1));
    assert!(guid.0[6] == GuidConstant::U8(0xB2));
    assert!(guid.0[7] == GuidConstant::U8(0x11));
    assert!(guid.0[8] == GuidConstant::U8(0xE6));
    assert!(guid.0[9] == GuidConstant::U8(0x27));
    assert!(guid.0[10] == GuidConstant::U8(0xC3));
}
