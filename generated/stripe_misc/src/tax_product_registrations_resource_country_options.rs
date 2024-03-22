#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptions {
    pub ae: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub at: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub au: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub be: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub bg: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ca: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCanada>,
    pub ch: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub cl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub co: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub cy: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub cz: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub de: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub dk: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ee: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub es: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fi: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub gb: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub gr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hu: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub id: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub ie: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub is: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub it: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub jp: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub kr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub lt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lu: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lv: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mx: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub my: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub nl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub no: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub nz: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub pl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub pt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ro: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sa: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub se: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sg: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub si: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sk: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub th: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub tr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub us: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
    pub vn: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub za: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
}
#[cfg(feature = "min-ser")]
pub struct TaxProductRegistrationsResourceCountryOptionsBuilder {
    ae: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    at: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    au: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    be: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    bg: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ca: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCanada>>,
    ch: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    cl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    co: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    cy: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    cz: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    de: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    dk: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ee: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    es: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    fi: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    fr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    gb: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    gr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    hr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    hu: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    id: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    ie: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    is: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    it: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    jp: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    kr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    lt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    lu: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    lv: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    mt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    mx: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    my: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    nl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    no: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    nz: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    pl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    pt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ro: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sa: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    se: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sg: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    si: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sk: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    th: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    tr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    us: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUnitedStates>>,
    vn: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    za: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptions>,
        builder: TaxProductRegistrationsResourceCountryOptionsBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxProductRegistrationsResourceCountryOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "ae" => Ok(Deserialize::begin(&mut self.ae)),
                "at" => Ok(Deserialize::begin(&mut self.at)),
                "au" => Ok(Deserialize::begin(&mut self.au)),
                "be" => Ok(Deserialize::begin(&mut self.be)),
                "bg" => Ok(Deserialize::begin(&mut self.bg)),
                "ca" => Ok(Deserialize::begin(&mut self.ca)),
                "ch" => Ok(Deserialize::begin(&mut self.ch)),
                "cl" => Ok(Deserialize::begin(&mut self.cl)),
                "co" => Ok(Deserialize::begin(&mut self.co)),
                "cy" => Ok(Deserialize::begin(&mut self.cy)),
                "cz" => Ok(Deserialize::begin(&mut self.cz)),
                "de" => Ok(Deserialize::begin(&mut self.de)),
                "dk" => Ok(Deserialize::begin(&mut self.dk)),
                "ee" => Ok(Deserialize::begin(&mut self.ee)),
                "es" => Ok(Deserialize::begin(&mut self.es)),
                "fi" => Ok(Deserialize::begin(&mut self.fi)),
                "fr" => Ok(Deserialize::begin(&mut self.fr)),
                "gb" => Ok(Deserialize::begin(&mut self.gb)),
                "gr" => Ok(Deserialize::begin(&mut self.gr)),
                "hr" => Ok(Deserialize::begin(&mut self.hr)),
                "hu" => Ok(Deserialize::begin(&mut self.hu)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "ie" => Ok(Deserialize::begin(&mut self.ie)),
                "is" => Ok(Deserialize::begin(&mut self.is)),
                "it" => Ok(Deserialize::begin(&mut self.it)),
                "jp" => Ok(Deserialize::begin(&mut self.jp)),
                "kr" => Ok(Deserialize::begin(&mut self.kr)),
                "lt" => Ok(Deserialize::begin(&mut self.lt)),
                "lu" => Ok(Deserialize::begin(&mut self.lu)),
                "lv" => Ok(Deserialize::begin(&mut self.lv)),
                "mt" => Ok(Deserialize::begin(&mut self.mt)),
                "mx" => Ok(Deserialize::begin(&mut self.mx)),
                "my" => Ok(Deserialize::begin(&mut self.my)),
                "nl" => Ok(Deserialize::begin(&mut self.nl)),
                "no" => Ok(Deserialize::begin(&mut self.no)),
                "nz" => Ok(Deserialize::begin(&mut self.nz)),
                "pl" => Ok(Deserialize::begin(&mut self.pl)),
                "pt" => Ok(Deserialize::begin(&mut self.pt)),
                "ro" => Ok(Deserialize::begin(&mut self.ro)),
                "sa" => Ok(Deserialize::begin(&mut self.sa)),
                "se" => Ok(Deserialize::begin(&mut self.se)),
                "sg" => Ok(Deserialize::begin(&mut self.sg)),
                "si" => Ok(Deserialize::begin(&mut self.si)),
                "sk" => Ok(Deserialize::begin(&mut self.sk)),
                "th" => Ok(Deserialize::begin(&mut self.th)),
                "tr" => Ok(Deserialize::begin(&mut self.tr)),
                "us" => Ok(Deserialize::begin(&mut self.us)),
                "vn" => Ok(Deserialize::begin(&mut self.vn)),
                "za" => Ok(Deserialize::begin(&mut self.za)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                ae: Deserialize::default(),
                at: Deserialize::default(),
                au: Deserialize::default(),
                be: Deserialize::default(),
                bg: Deserialize::default(),
                ca: Deserialize::default(),
                ch: Deserialize::default(),
                cl: Deserialize::default(),
                co: Deserialize::default(),
                cy: Deserialize::default(),
                cz: Deserialize::default(),
                de: Deserialize::default(),
                dk: Deserialize::default(),
                ee: Deserialize::default(),
                es: Deserialize::default(),
                fi: Deserialize::default(),
                fr: Deserialize::default(),
                gb: Deserialize::default(),
                gr: Deserialize::default(),
                hr: Deserialize::default(),
                hu: Deserialize::default(),
                id: Deserialize::default(),
                ie: Deserialize::default(),
                is: Deserialize::default(),
                it: Deserialize::default(),
                jp: Deserialize::default(),
                kr: Deserialize::default(),
                lt: Deserialize::default(),
                lu: Deserialize::default(),
                lv: Deserialize::default(),
                mt: Deserialize::default(),
                mx: Deserialize::default(),
                my: Deserialize::default(),
                nl: Deserialize::default(),
                no: Deserialize::default(),
                nz: Deserialize::default(),
                pl: Deserialize::default(),
                pt: Deserialize::default(),
                ro: Deserialize::default(),
                sa: Deserialize::default(),
                se: Deserialize::default(),
                sg: Deserialize::default(),
                si: Deserialize::default(),
                sk: Deserialize::default(),
                th: Deserialize::default(),
                tr: Deserialize::default(),
                us: Deserialize::default(),
                vn: Deserialize::default(),
                za: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ae = self.ae.take()?;
            let at = self.at.take()?;
            let au = self.au.take()?;
            let be = self.be.take()?;
            let bg = self.bg.take()?;
            let ca = self.ca.take()?;
            let ch = self.ch.take()?;
            let cl = self.cl.take()?;
            let co = self.co.take()?;
            let cy = self.cy.take()?;
            let cz = self.cz.take()?;
            let de = self.de.take()?;
            let dk = self.dk.take()?;
            let ee = self.ee.take()?;
            let es = self.es.take()?;
            let fi = self.fi.take()?;
            let fr = self.fr.take()?;
            let gb = self.gb.take()?;
            let gr = self.gr.take()?;
            let hr = self.hr.take()?;
            let hu = self.hu.take()?;
            let id = self.id.take()?;
            let ie = self.ie.take()?;
            let is = self.is.take()?;
            let it = self.it.take()?;
            let jp = self.jp.take()?;
            let kr = self.kr.take()?;
            let lt = self.lt.take()?;
            let lu = self.lu.take()?;
            let lv = self.lv.take()?;
            let mt = self.mt.take()?;
            let mx = self.mx.take()?;
            let my = self.my.take()?;
            let nl = self.nl.take()?;
            let no = self.no.take()?;
            let nz = self.nz.take()?;
            let pl = self.pl.take()?;
            let pt = self.pt.take()?;
            let ro = self.ro.take()?;
            let sa = self.sa.take()?;
            let se = self.se.take()?;
            let sg = self.sg.take()?;
            let si = self.si.take()?;
            let sk = self.sk.take()?;
            let th = self.th.take()?;
            let tr = self.tr.take()?;
            let us = self.us.take()?;
            let vn = self.vn.take()?;
            let za = self.za.take()?;

            Some(Self::Out {
                ae,
                at,
                au,
                be,
                bg,
                ca,
                ch,
                cl,
                co,
                cy,
                cz,
                de,
                dk,
                ee,
                es,
                fi,
                fr,
                gb,
                gr,
                hr,
                hu,
                id,
                ie,
                is,
                it,
                jp,
                kr,
                lt,
                lu,
                lv,
                mt,
                mx,
                my,
                nl,
                no,
                nz,
                pl,
                pt,
                ro,
                sa,
                se,
                sg,
                si,
                sk,
                th,
                tr,
                us,
                vn,
                za,
            })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptions {
        type Builder = TaxProductRegistrationsResourceCountryOptionsBuilder;
    }
};
