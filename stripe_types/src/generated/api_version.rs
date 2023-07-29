#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApiVersion {
    V2011_01_01,
    V2011_06_21,
    V2011_06_28,
    V2011_08_01,
    V2011_09_15,
    V2011_11_17,
    V2012_02_23,
    V2012_03_25,
    V2012_06_18,
    V2012_06_28,
    V2012_07_09,
    V2012_09_24,
    V2012_10_26,
    V2012_11_07,
    V2013_02_11,
    V2013_02_13,
    V2013_07_05,
    V2013_08_12,
    V2013_08_13,
    V2013_10_29,
    V2013_12_03,
    V2014_01_31,
    V2014_03_13,
    V2014_03_28,
    V2014_05_19,
    V2014_06_13,
    V2014_06_17,
    V2014_07_22,
    V2014_07_26,
    V2014_08_04,
    V2014_08_20,
    V2014_09_08,
    V2014_10_07,
    V2014_11_05,
    V2014_11_20,
    V2014_12_08,
    V2014_12_17,
    V2014_12_22,
    V2015_01_11,
    V2015_01_26,
    V2015_02_10,
    V2015_02_16,
    V2015_02_18,
    V2015_03_24,
    V2015_04_07,
    V2015_06_15,
    V2015_07_07,
    V2015_07_13,
    V2015_07_28,
    V2015_08_07,
    V2015_08_19,
    V2015_09_03,
    V2015_09_08,
    V2015_09_23,
    V2015_10_01,
    V2015_10_12,
    V2015_10_16,
    V2016_02_03,
    V2016_02_19,
    V2016_02_22,
    V2016_02_23,
    V2016_02_29,
    V2016_03_07,
    V2016_06_15,
    V2016_07_06,
    V2016_10_19,
    V2017_01_27,
    V2017_02_14,
    V2017_04_06,
    V2017_05_25,
    V2017_06_05,
    V2017_08_15,
    V2017_12_14,
    V2018_01_23,
    V2018_02_05,
    V2018_02_06,
    V2018_02_28,
    V2018_05_21,
    V2018_07_27,
    V2018_08_23,
    V2018_09_06,
    V2018_09_24,
    V2018_10_31,
    V2018_11_08,
    V2019_02_11,
    V2019_02_19,
    V2019_03_14,
    V2019_05_16,
    V2019_08_14,
    V2019_09_09,
    V2019_10_08,
    V2019_10_17,
    V2019_11_05,
    V2019_12_03,
    V2020_03_02,
    V2020_08_27,
    V2022_08_01,
}

impl ApiVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V2011_01_01 => "2011-01-01",
            Self::V2011_06_21 => "2011-06-21",
            Self::V2011_06_28 => "2011-06-28",
            Self::V2011_08_01 => "2011-08-01",
            Self::V2011_09_15 => "2011-09-15",
            Self::V2011_11_17 => "2011-11-17",
            Self::V2012_02_23 => "2012-02-23",
            Self::V2012_03_25 => "2012-03-25",
            Self::V2012_06_18 => "2012-06-18",
            Self::V2012_06_28 => "2012-06-28",
            Self::V2012_07_09 => "2012-07-09",
            Self::V2012_09_24 => "2012-09-24",
            Self::V2012_10_26 => "2012-10-26",
            Self::V2012_11_07 => "2012-11-07",
            Self::V2013_02_11 => "2013-02-11",
            Self::V2013_02_13 => "2013-02-13",
            Self::V2013_07_05 => "2013-07-05",
            Self::V2013_08_12 => "2013-08-12",
            Self::V2013_08_13 => "2013-08-13",
            Self::V2013_10_29 => "2013-10-29",
            Self::V2013_12_03 => "2013-12-03",
            Self::V2014_01_31 => "2014-01-31",
            Self::V2014_03_13 => "2014-03-13",
            Self::V2014_03_28 => "2014-03-28",
            Self::V2014_05_19 => "2014-05-19",
            Self::V2014_06_13 => "2014-06-13",
            Self::V2014_06_17 => "2014-06-17",
            Self::V2014_07_22 => "2014-07-22",
            Self::V2014_07_26 => "2014-07-26",
            Self::V2014_08_04 => "2014-08-04",
            Self::V2014_08_20 => "2014-08-20",
            Self::V2014_09_08 => "2014-09-08",
            Self::V2014_10_07 => "2014-10-07",
            Self::V2014_11_05 => "2014-11-05",
            Self::V2014_11_20 => "2014-11-20",
            Self::V2014_12_08 => "2014-12-08",
            Self::V2014_12_17 => "2014-12-17",
            Self::V2014_12_22 => "2014-12-22",
            Self::V2015_01_11 => "2015-01-11",
            Self::V2015_01_26 => "2015-01-26",
            Self::V2015_02_10 => "2015-02-10",
            Self::V2015_02_16 => "2015-02-16",
            Self::V2015_02_18 => "2015-02-18",
            Self::V2015_03_24 => "2015-03-24",
            Self::V2015_04_07 => "2015-04-07",
            Self::V2015_06_15 => "2015-06-15",
            Self::V2015_07_07 => "2015-07-07",
            Self::V2015_07_13 => "2015-07-13",
            Self::V2015_07_28 => "2015-07-28",
            Self::V2015_08_07 => "2015-08-07",
            Self::V2015_08_19 => "2015-08-19",
            Self::V2015_09_03 => "2015-09-03",
            Self::V2015_09_08 => "2015-09-08",
            Self::V2015_09_23 => "2015-09-23",
            Self::V2015_10_01 => "2015-10-01",
            Self::V2015_10_12 => "2015-10-12",
            Self::V2015_10_16 => "2015-10-16",
            Self::V2016_02_03 => "2016-02-03",
            Self::V2016_02_19 => "2016-02-19",
            Self::V2016_02_22 => "2016-02-22",
            Self::V2016_02_23 => "2016-02-23",
            Self::V2016_02_29 => "2016-02-29",
            Self::V2016_03_07 => "2016-03-07",
            Self::V2016_06_15 => "2016-06-15",
            Self::V2016_07_06 => "2016-07-06",
            Self::V2016_10_19 => "2016-10-19",
            Self::V2017_01_27 => "2017-01-27",
            Self::V2017_02_14 => "2017-02-14",
            Self::V2017_04_06 => "2017-04-06",
            Self::V2017_05_25 => "2017-05-25",
            Self::V2017_06_05 => "2017-06-05",
            Self::V2017_08_15 => "2017-08-15",
            Self::V2017_12_14 => "2017-12-14",
            Self::V2018_01_23 => "2018-01-23",
            Self::V2018_02_05 => "2018-02-05",
            Self::V2018_02_06 => "2018-02-06",
            Self::V2018_02_28 => "2018-02-28",
            Self::V2018_05_21 => "2018-05-21",
            Self::V2018_07_27 => "2018-07-27",
            Self::V2018_08_23 => "2018-08-23",
            Self::V2018_09_06 => "2018-09-06",
            Self::V2018_09_24 => "2018-09-24",
            Self::V2018_10_31 => "2018-10-31",
            Self::V2018_11_08 => "2018-11-08",
            Self::V2019_02_11 => "2019-02-11",
            Self::V2019_02_19 => "2019-02-19",
            Self::V2019_03_14 => "2019-03-14",
            Self::V2019_05_16 => "2019-05-16",
            Self::V2019_08_14 => "2019-08-14",
            Self::V2019_09_09 => "2019-09-09",
            Self::V2019_10_08 => "2019-10-08",
            Self::V2019_10_17 => "2019-10-17",
            Self::V2019_11_05 => "2019-11-05",
            Self::V2019_12_03 => "2019-12-03",
            Self::V2020_03_02 => "2020-03-02",
            Self::V2020_08_27 => "2020-08-27",
            Self::V2022_08_01 => "2022-08-01",
        }
    }
}

impl std::str::FromStr for ApiVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2011-01-01" => Ok(Self::V2011_01_01),
            "2011-06-21" => Ok(Self::V2011_06_21),
            "2011-06-28" => Ok(Self::V2011_06_28),
            "2011-08-01" => Ok(Self::V2011_08_01),
            "2011-09-15" => Ok(Self::V2011_09_15),
            "2011-11-17" => Ok(Self::V2011_11_17),
            "2012-02-23" => Ok(Self::V2012_02_23),
            "2012-03-25" => Ok(Self::V2012_03_25),
            "2012-06-18" => Ok(Self::V2012_06_18),
            "2012-06-28" => Ok(Self::V2012_06_28),
            "2012-07-09" => Ok(Self::V2012_07_09),
            "2012-09-24" => Ok(Self::V2012_09_24),
            "2012-10-26" => Ok(Self::V2012_10_26),
            "2012-11-07" => Ok(Self::V2012_11_07),
            "2013-02-11" => Ok(Self::V2013_02_11),
            "2013-02-13" => Ok(Self::V2013_02_13),
            "2013-07-05" => Ok(Self::V2013_07_05),
            "2013-08-12" => Ok(Self::V2013_08_12),
            "2013-08-13" => Ok(Self::V2013_08_13),
            "2013-10-29" => Ok(Self::V2013_10_29),
            "2013-12-03" => Ok(Self::V2013_12_03),
            "2014-01-31" => Ok(Self::V2014_01_31),
            "2014-03-13" => Ok(Self::V2014_03_13),
            "2014-03-28" => Ok(Self::V2014_03_28),
            "2014-05-19" => Ok(Self::V2014_05_19),
            "2014-06-13" => Ok(Self::V2014_06_13),
            "2014-06-17" => Ok(Self::V2014_06_17),
            "2014-07-22" => Ok(Self::V2014_07_22),
            "2014-07-26" => Ok(Self::V2014_07_26),
            "2014-08-04" => Ok(Self::V2014_08_04),
            "2014-08-20" => Ok(Self::V2014_08_20),
            "2014-09-08" => Ok(Self::V2014_09_08),
            "2014-10-07" => Ok(Self::V2014_10_07),
            "2014-11-05" => Ok(Self::V2014_11_05),
            "2014-11-20" => Ok(Self::V2014_11_20),
            "2014-12-08" => Ok(Self::V2014_12_08),
            "2014-12-17" => Ok(Self::V2014_12_17),
            "2014-12-22" => Ok(Self::V2014_12_22),
            "2015-01-11" => Ok(Self::V2015_01_11),
            "2015-01-26" => Ok(Self::V2015_01_26),
            "2015-02-10" => Ok(Self::V2015_02_10),
            "2015-02-16" => Ok(Self::V2015_02_16),
            "2015-02-18" => Ok(Self::V2015_02_18),
            "2015-03-24" => Ok(Self::V2015_03_24),
            "2015-04-07" => Ok(Self::V2015_04_07),
            "2015-06-15" => Ok(Self::V2015_06_15),
            "2015-07-07" => Ok(Self::V2015_07_07),
            "2015-07-13" => Ok(Self::V2015_07_13),
            "2015-07-28" => Ok(Self::V2015_07_28),
            "2015-08-07" => Ok(Self::V2015_08_07),
            "2015-08-19" => Ok(Self::V2015_08_19),
            "2015-09-03" => Ok(Self::V2015_09_03),
            "2015-09-08" => Ok(Self::V2015_09_08),
            "2015-09-23" => Ok(Self::V2015_09_23),
            "2015-10-01" => Ok(Self::V2015_10_01),
            "2015-10-12" => Ok(Self::V2015_10_12),
            "2015-10-16" => Ok(Self::V2015_10_16),
            "2016-02-03" => Ok(Self::V2016_02_03),
            "2016-02-19" => Ok(Self::V2016_02_19),
            "2016-02-22" => Ok(Self::V2016_02_22),
            "2016-02-23" => Ok(Self::V2016_02_23),
            "2016-02-29" => Ok(Self::V2016_02_29),
            "2016-03-07" => Ok(Self::V2016_03_07),
            "2016-06-15" => Ok(Self::V2016_06_15),
            "2016-07-06" => Ok(Self::V2016_07_06),
            "2016-10-19" => Ok(Self::V2016_10_19),
            "2017-01-27" => Ok(Self::V2017_01_27),
            "2017-02-14" => Ok(Self::V2017_02_14),
            "2017-04-06" => Ok(Self::V2017_04_06),
            "2017-05-25" => Ok(Self::V2017_05_25),
            "2017-06-05" => Ok(Self::V2017_06_05),
            "2017-08-15" => Ok(Self::V2017_08_15),
            "2017-12-14" => Ok(Self::V2017_12_14),
            "2018-01-23" => Ok(Self::V2018_01_23),
            "2018-02-05" => Ok(Self::V2018_02_05),
            "2018-02-06" => Ok(Self::V2018_02_06),
            "2018-02-28" => Ok(Self::V2018_02_28),
            "2018-05-21" => Ok(Self::V2018_05_21),
            "2018-07-27" => Ok(Self::V2018_07_27),
            "2018-08-23" => Ok(Self::V2018_08_23),
            "2018-09-06" => Ok(Self::V2018_09_06),
            "2018-09-24" => Ok(Self::V2018_09_24),
            "2018-10-31" => Ok(Self::V2018_10_31),
            "2018-11-08" => Ok(Self::V2018_11_08),
            "2019-02-11" => Ok(Self::V2019_02_11),
            "2019-02-19" => Ok(Self::V2019_02_19),
            "2019-03-14" => Ok(Self::V2019_03_14),
            "2019-05-16" => Ok(Self::V2019_05_16),
            "2019-08-14" => Ok(Self::V2019_08_14),
            "2019-09-09" => Ok(Self::V2019_09_09),
            "2019-10-08" => Ok(Self::V2019_10_08),
            "2019-10-17" => Ok(Self::V2019_10_17),
            "2019-11-05" => Ok(Self::V2019_11_05),
            "2019-12-03" => Ok(Self::V2019_12_03),
            "2020-03-02" => Ok(Self::V2020_03_02),
            "2020-08-27" => Ok(Self::V2020_08_27),
            "2022-08-01" => Ok(Self::V2022_08_01),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ApiVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ApiVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ApiVersion"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApiVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ApiVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApiVersion::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
