#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PersonRelationship {
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    pub executive: Option<bool>,
    /// Whether the person is the legal guardian of the account's representative.
    pub legal_guardian: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    pub title: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PersonRelationshipBuilder {
    director: Option<Option<bool>>,
    executive: Option<Option<bool>>,
    legal_guardian: Option<Option<bool>>,
    owner: Option<Option<bool>>,
    percent_ownership: Option<Option<f64>>,
    representative: Option<Option<bool>>,
    title: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PersonRelationship {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonRelationship>,
        builder: PersonRelationshipBuilder,
    }

    impl Visitor for Place<PersonRelationship> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PersonRelationshipBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PersonRelationshipBuilder {
        type Out = PersonRelationship;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "director" => Ok(Deserialize::begin(&mut self.director)),
                "executive" => Ok(Deserialize::begin(&mut self.executive)),
                "legal_guardian" => Ok(Deserialize::begin(&mut self.legal_guardian)),
                "owner" => Ok(Deserialize::begin(&mut self.owner)),
                "percent_ownership" => Ok(Deserialize::begin(&mut self.percent_ownership)),
                "representative" => Ok(Deserialize::begin(&mut self.representative)),
                "title" => Ok(Deserialize::begin(&mut self.title)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                director: Deserialize::default(),
                executive: Deserialize::default(),
                legal_guardian: Deserialize::default(),
                owner: Deserialize::default(),
                percent_ownership: Deserialize::default(),
                representative: Deserialize::default(),
                title: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let director = self.director.take()?;
            let executive = self.executive.take()?;
            let legal_guardian = self.legal_guardian.take()?;
            let owner = self.owner.take()?;
            let percent_ownership = self.percent_ownership.take()?;
            let representative = self.representative.take()?;
            let title = self.title.take()?;

            Some(Self::Out { director, executive, legal_guardian, owner, percent_ownership, representative, title })
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

    impl ObjectDeser for PersonRelationship {
        type Builder = PersonRelationshipBuilder;
    }
};
