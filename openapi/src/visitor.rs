use crate::rust_object::{EnumVariant, ObjectMetadata, ObjectUsage, RustObject, StructField};
use crate::rust_type::RustType;

pub trait Visit<'a> {
    fn visit_field(&mut self, field: &'a StructField, usage: ObjectUsage)
    where
        Self: Sized,
    {
        self.visit_typ(&field.rust_type, usage);
    }

    fn visit_variant(&mut self, variant: &'a EnumVariant, usage: ObjectUsage)
    where
        Self: Sized,
    {
        if let Some(typ) = &variant.rust_type {
            self.visit_typ(typ, usage);
        }
    }

    fn visit_obj(
        &mut self,
        obj: &'a RustObject,
        _meta: Option<&'a ObjectMetadata>,
        usage: ObjectUsage,
    ) where
        Self: Sized,
    {
        obj.visit(self, usage);
    }

    fn visit_typ(&mut self, typ: &'a RustType, usage: ObjectUsage)
    where
        Self: Sized,
    {
        typ.visit(self, usage);
    }
}

pub trait VisitMut {
    fn visit_obj_mut(
        &mut self,
        obj: &mut RustObject,
        _meta: Option<&ObjectMetadata>,
        usage: ObjectUsage,
    ) where
        Self: Sized,
    {
        obj.visit_mut(self, usage);
    }

    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage)
    where
        Self: Sized,
    {
        typ.visit_mut(self, usage);
    }
}
