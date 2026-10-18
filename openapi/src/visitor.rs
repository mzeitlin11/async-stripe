use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::RustType;

pub trait Visit<'a> {
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
