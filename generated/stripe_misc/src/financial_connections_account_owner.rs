/// Describes an owner of an account.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialConnectionsAccountOwner {
    /// The email address of the owner.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAccountOwnerId,
    /// The full name of the owner.
    pub name: String,
    /// The ownership object that this owner belongs to.
    pub ownership: String,
    /// The raw phone number of the owner.
    pub phone: Option<String>,
    /// The raw physical address of the owner.
    pub raw_address: Option<String>,
    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct FinancialConnectionsAccountOwnerBuilder {
    email: Option<Option<String>>,
    id: Option<stripe_misc::FinancialConnectionsAccountOwnerId>,
    name: Option<String>,
    ownership: Option<String>,
    phone: Option<Option<String>>,
    raw_address: Option<Option<String>>,
    refreshed_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FinancialConnectionsAccountOwner {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsAccountOwner>,
        builder: FinancialConnectionsAccountOwnerBuilder,
    }

    impl Visitor for Place<FinancialConnectionsAccountOwner> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FinancialConnectionsAccountOwnerBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FinancialConnectionsAccountOwnerBuilder {
        type Out = FinancialConnectionsAccountOwner;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "ownership" => Ok(Deserialize::begin(&mut self.ownership)),
                "phone" => Ok(Deserialize::begin(&mut self.phone)),
                "raw_address" => Ok(Deserialize::begin(&mut self.raw_address)),
                "refreshed_at" => Ok(Deserialize::begin(&mut self.refreshed_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                email: Deserialize::default(),
                id: Deserialize::default(),
                name: Deserialize::default(),
                ownership: Deserialize::default(),
                phone: Deserialize::default(),
                raw_address: Deserialize::default(),
                refreshed_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let email = self.email.take()?;
            let id = self.id.take()?;
            let name = self.name.take()?;
            let ownership = self.ownership.take()?;
            let phone = self.phone.take()?;
            let raw_address = self.raw_address.take()?;
            let refreshed_at = self.refreshed_at.take()?;

            Some(Self::Out { email, id, name, ownership, phone, raw_address, refreshed_at })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for FinancialConnectionsAccountOwner {
        type Builder = FinancialConnectionsAccountOwnerBuilder;
    }
};
impl stripe_types::Object for FinancialConnectionsAccountOwner {
    type Id = stripe_misc::FinancialConnectionsAccountOwnerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnerId);
