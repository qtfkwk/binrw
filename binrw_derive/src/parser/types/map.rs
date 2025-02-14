use crate::parser::{read, read::attrs, write, KeywordToken, TrySet};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
pub(crate) enum Map {
    None,
    Map(TokenStream),
    Try(TokenStream),
}

impl Map {
    pub(crate) fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub(crate) fn is_try(&self) -> bool {
        matches!(self, Self::Try(_))
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::None
    }
}

impl From<attrs::Map> for Map {
    fn from(map: attrs::Map) -> Self {
        Self::Map(map.value.to_token_stream())
    }
}

impl From<attrs::TryMap> for Map {
    fn from(try_map: attrs::TryMap) -> Self {
        Self::Try(try_map.value.to_token_stream())
    }
}

impl From<read::attrs::Repr> for Map {
    fn from(repr: read::attrs::Repr) -> Self {
        let ty = repr.value.to_token_stream();
        Self::Try(quote! { <#ty as core::convert::TryInto<_>>::try_into })
    }
}

impl From<write::attrs::Repr> for Map {
    fn from(repr: write::attrs::Repr) -> Self {
        let ty = repr.value.to_token_stream();
        Self::Try(quote! { <#ty as core::convert::TryFrom<_>>::try_from })
    }
}

impl<T: Into<Map> + KeywordToken> TrySet<Map> for T {
    fn try_set(self, to: &mut Map) -> syn::Result<()> {
        if to.is_some() {
            Err(syn::Error::new(
                self.keyword_span(),
                "conflicting map keyword",
            ))
        } else {
            *to = self.into();
            Ok(())
        }
    }
}
