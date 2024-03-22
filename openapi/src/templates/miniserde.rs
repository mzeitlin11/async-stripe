use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::rust_object::{EnumOfObjects, StructField};
use crate::templates::enums::write_feature_gate;
use crate::templates::ObjectWriter;
use crate::types::RustIdent;

pub fn gen_enum_of_objects_miniserde(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    let builder_name = RustIdent::joined(ident, "Builder");
    let inner_builder_type = match objects {
        EnumOfObjects::MaybeDeleted(_) => "MaybeDeletedBuilderInner",
        EnumOfObjects::ObjectUnion(_) => "ObjectBuilderInner",
    };
    let inner = build_inner(ident, &builder_name, objects);
    formatdoc! {
        r#"
        #[cfg(feature = "min-ser")]
        #[derive(Default)]
        pub struct {builder_name} {{
            inner: stripe_types::miniserde_helpers::{inner_builder_type},
        }}
    
        #[cfg(feature = "min-ser")]
        const _: () = {{
            use miniserde::de::{{Map, Visitor}};
            use miniserde::json::{{from_str, to_string}};
            use miniserde::{{make_place, Deserialize, Result}};
            use stripe_types::MapBuilder;
            use super::*;
            
            make_place!(Place);
            
            {inner}
        }};
        "#
    }
}

fn take_out_inner(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    let mut finish_inner = String::new();
    match objects {
        EnumOfObjects::MaybeDeleted(items) => {
            let deleted_name = &items.deleted.ident;
            let base_name = &items.base.ident;
            formatdoc! {r#"
            let (deleted, object) = self.inner.finish_inner()?;
            let obj_str = to_string(&object);
            Some(if deleted {{
                {ident}::{deleted_name}(from_str(&obj_str).ok()?)
            }} else {{
                {ident}::{base_name}(from_str(&obj_str).ok()?)
            }})
            "#}
        }
        EnumOfObjects::ObjectUnion(objects) => {
            for (obj_discr, obj) in objects {
                let name = &obj.ident;
                write_feature_gate(&mut finish_inner, obj);
                let _ = writeln!(
                    finish_inner,
                    r#""{obj_discr}" => {ident}::{name}(from_str(&obj_str).ok()?),"#
                );
            }
            formatdoc! {r#"
            let (obj_key, object) = self.inner.finish_inner()?;
            let obj_str = to_string(&object);
            Some(match obj_key.as_str() {{
                {finish_inner}
                _ => return None,
            }})
            "#

            }
        }
    }
}

fn build_inner(ident: &RustIdent, builder_name: &RustIdent, objects: &EnumOfObjects) -> String {
    let take_out_func_inner = take_out_inner(ident, objects);
    formatdoc! {r#"
    struct Builder<'a> {{
        out: &'a mut Option<{ident}>,
        builder: {builder_name},
    }}
    
    
    impl Deserialize for {ident} {{
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {{
           Place::new(out)
        }}
    }}
    
    impl Visitor for Place<{ident}> {{
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {{
            Ok(Box::new(Builder {{
                out: &mut self.out,
                builder: Default::default(),
            }}))
        }}
    }}
    
    impl<'a> Map for Builder<'a> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}
        
        fn finish(&mut self) -> Result<()> {{
            *self.out = self.builder.take_out();
            Ok(())
        }}
    }}
    
    impl stripe_types::MapBuilder for {builder_name} {{
        type Out = {ident};
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {{
            self.inner.key_inner(k)
        }}
        
        fn deser_default() -> Self {{
            Self::default()
        }}

        fn take_out(&mut self) -> Option<Self::Out> {{
            {take_out_func_inner}
        }}
    }}
    
    impl stripe_types::ObjectDeser for {ident} {{
        type Builder = {builder_name};
    }}
    "#
    }
}

impl<'a> ObjectWriter<'a> {
    pub fn gen_miniserde_struct_deserialize(&self, out: &mut String, fields: &[StructField]) {
        let ident = self.ident;
        let builder_name = RustIdent::joined(ident, "Builder");
        let mut builder_inner = String::new();
        for field in fields {
            let f_name = &field.field_name;
            let printable = self.components.construct_printable_type(&field.rust_type);
            let _ = writeln!(builder_inner, "{f_name}: Option<{printable}>,");
        }

        let inner = miniserde_struct_inner(ident, &builder_name, fields);
        let _ = writedoc! {
            out,
            r#"
        #[cfg(feature = "min-ser")]
        pub struct {builder_name} {{
            {builder_inner}
        }}
        
        #[cfg(feature = "min-ser")]
        const _: () = {{
            use miniserde::de::{{Map, Visitor}};
            use miniserde::{{make_place, Deserialize, Result}};
            use stripe_types::{{MapBuilder, ObjectDeser}};
        
            make_place!(Place);
        
            {inner}
        }};
        "#
        };
    }
}

fn miniserde_struct_inner(
    ident: &RustIdent,
    builder_name: &RustIdent,
    fields: &[StructField],
) -> String {
    let mut finish_inner = String::new();
    let mut key_inner = String::new();
    let mut builder_new_inner = String::new();
    for field in fields {
        let f_name = &field.field_name;

        let _ = writeln!(
            key_inner,
            r#""{}" => Ok(Deserialize::begin(&mut self.{f_name})),"#,
            field.wire_name()
        );
        let _ = writeln!(finish_inner, "let {f_name} = self.{f_name}.take()?;");

        // NB: using miniserde::Deserialize::default() instead of `None` is very important - this copies
        // the miniserde derives in defaulting `Option<Option<T>>` to `Ok(Some)` so that missing
        // values are allowed for option types
        let _ = writeln!(builder_new_inner, "{f_name}: Deserialize::default(),");
    }
    let finish_init = fields.iter().map(|f| f.field_name.as_str()).collect::<Vec<_>>().join(",");

    formatdoc! {r#"
    impl Deserialize for {ident} {{
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {{
           Place::new(out)
        }}
    }}
    
    struct Builder<'a> {{
        out: &'a mut Option<{ident}>,
        builder: {builder_name},
    }}

    impl Visitor for Place<{ident}> {{
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {{
            Ok(Box::new(Builder {{
                out: &mut self.out,
                builder: {builder_name}::deser_default(),
            }}))
        }}
    }}
    
    impl MapBuilder for {builder_name} {{
        type Out = {ident};
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {{
            #[allow(clippy::match_single_binding)]
            match k {{
                {key_inner}
                _ => Ok(<dyn Visitor>::ignore()),
            }}
        }}
        
        fn deser_default() -> Self {{
            Self {{
                {builder_new_inner}
            }}
        }}

        fn take_out(&mut self) -> Option<Self::Out> {{
            {finish_inner}
            Some(Self::Out {{ {finish_init} }})
        }}
    }}
    
    impl<'a> Map for Builder<'a> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}
        
        fn finish(&mut self) -> Result<()> {{
            *self.out = self.builder.take_out();
            Ok(())
        }}
    }}
    
    impl ObjectDeser for {ident} {{
        type Builder = {builder_name};
    }}
    "#
    }
}
