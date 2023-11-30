#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListIssuingToken<'a> {
    /// The Issuing card identifier to list tokens for.
    pub card: &'a str,
    /// Select Issuing tokens that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Select Issuing tokens with the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListIssuingTokenStatus>,
}
impl<'a> ListIssuingToken<'a> {
    pub fn new(card: &'a str) -> Self {
        Self {
            card,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Select Issuing tokens with the given status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListIssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}
impl ListIssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        use ListIssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for ListIssuingTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ListIssuingTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ListIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListIssuingToken<'a> {
    /// Lists all Issuing `Token` objects for a given card.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::IssuingToken>> {
        client.get_query("/issuing/tokens", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::IssuingToken> {
        stripe::ListPaginator::from_params("/issuing/tokens", self)
    }
}
impl<'a> stripe::PaginationParams for ListIssuingToken<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingToken<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingToken<'a> {
    /// Retrieves an Issuing `Token` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        token: &stripe_types::issuing_token::IssuingTokenId,
    ) -> stripe::Response<stripe_types::IssuingToken> {
        client.get_query(&format!("/issuing/tokens/{token}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Specifies which status the token should be updated to.
    pub status: UpdateIssuingTokenStatus,
}
impl<'a> UpdateIssuingToken<'a> {
    pub fn new(status: UpdateIssuingTokenStatus) -> Self {
        Self { expand: None, status }
    }
}
/// Specifies which status the token should be updated to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingTokenStatus {
    Active,
    Deleted,
    Suspended,
}
impl UpdateIssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for UpdateIssuingTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for UpdateIssuingTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for UpdateIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateIssuingToken<'a> {
    /// Attempts to update the specified Issuing `Token` object to the status specified.
    pub fn send(
        &self,
        client: &stripe::Client,
        token: &stripe_types::issuing_token::IssuingTokenId,
    ) -> stripe::Response<stripe_types::IssuingToken> {
        client.send_form(&format!("/issuing/tokens/{token}"), self, http_types::Method::Post)
    }
}
