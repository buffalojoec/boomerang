pub struct ParsedIntItem(syn::LitInt);
impl ParsedIntItem {
    pub fn value<T>(&self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        self.0.base10_parse::<T>().unwrap()
    }
}
impl syn::parse::Parse for ParsedIntItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse::<syn::LitInt>()?))
    }
}

pub struct ParsedPathItem(syn::Path);
impl ParsedPathItem {
    pub fn value(&self) -> syn::Path {
        self.0.to_owned()
    }
}
impl syn::parse::Parse for ParsedPathItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse::<syn::Path>()?))
    }
}

pub struct ParsedStringItem(syn::LitStr);
impl ParsedStringItem {
    pub fn value(&self) -> String {
        self.0.value()
    }
}
impl syn::parse::Parse for ParsedStringItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse::<syn::LitStr>()?))
    }
}

pub struct ParsedStringTupleItem(syn::LitStr, syn::LitStr);
impl ParsedStringTupleItem {
    pub fn value(&self) -> (String, String) {
        (self.0.value(), self.1.value())
    }
}
impl syn::parse::Parse for ParsedStringTupleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let val1: syn::LitStr = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let val2: syn::LitStr = content.parse()?;
        Ok(Self(val1, val2))
    }
}

pub fn parse_singleton_arg<T: syn::parse::Parse>(input: syn::parse::ParseStream) -> syn::Result<T> {
    let _equals_sign = input.parse::<syn::Token![=]>()?;
    input.parse::<T>()
}

pub fn parse_bracketed_list_arg<T: syn::parse::Parse>(
    input: syn::parse::ParseStream,
) -> syn::Result<Vec<T>> {
    let _equals_sign = input.parse::<syn::Token![=]>()?;
    let content;
    syn::bracketed!(content in input);
    Ok(content
        .parse_terminated(T::parse, syn::Token![,])?
        .into_iter()
        .collect::<Vec<_>>())
}

pub fn parse_list<T: syn::parse::Parse>(input: syn::parse::ParseStream) -> syn::Result<Vec<T>> {
    Ok(input
        .parse_terminated(T::parse, syn::Token![,])?
        .into_iter()
        .collect())
}
