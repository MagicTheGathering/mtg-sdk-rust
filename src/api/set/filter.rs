use itertools::Itertools;
use std::fmt::Display;
use api::set::filtertypes::SetBlock;

const SEP_AND: &str = ",";
const SEP_OR: &str = "|";

#[derive(Clone, Debug)]
pub struct SetFilterBuilder {
    filter: String,
}

impl SetFilterBuilder {
    fn new() -> SetFilterBuilder {
        SetFilterBuilder {
            filter: String::new(),
        }
    }

    /// Creates a Setilter with the specified filter parameters
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder
    ///     .name("Khans of Tarkir")
    ///     .block(SetBlock::KhansOfTarkir)
    ///     .build();
    /// assert!(filter == SetFilter("name=Khans of Tarkir&block=Khans of Tarkir".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn build(self) -> SetFilter {
        SetFilter(self.filter)
    }

    /// Create a custom filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.custom("name", "Dominaria")
    ///     .build();
    /// assert!(filter == SetFilter("name=Dominaria".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn custom<'a, T>(mut self, key: T, value: T) -> SetFilterBuilder
        where
            T: Into<&'a str>,
    {
        self.add_filter(key.into(), value.into());
        self
    }

    /// Every set that (partially) matches the specified name will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.name("Dominaria")
    ///     .build();
    /// assert!(filter == SetFilter("name=Dominaria".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn name<'a, T>(mut self, name: T) -> SetFilterBuilder
        where
            T: Into<&'a str>,
    {
        self.add_filter("name", name.into());
        self
    }

    /// Every set that (partially) matches one of the specified names will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.names(&vec!["Dominaria", "Core Set 2019"])
    ///     .build();
    /// assert!(filter == SetFilter("name=Dominaria|Core Set 2019".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn names<T>(mut self, names: &[T]) -> SetFilterBuilder
        where
            T: Display,
    {
        let values = names.into_iter().join(SEP_OR);
        self.add_filter("name", &values);
        self
    }

    /// Every set that (partially) matches the specified block will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.block(SetBlock::Amonkhet)
    ///     .build();
    /// assert!(filter == SetFilter("block=Amonkhet".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn block(mut self, block: SetBlock) -> SetFilterBuilder {
        self.add_filter("block", &block.as_str());
        self
    }

    /// Every set that (partially) matches one of the specified blocks will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.blocks(&vec![SetBlock::Amonkhet, SetBlock::Ixalan])
    ///     .build();
    /// assert!(filter == SetFilter("block=Amonkhet|Ixalan".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn blocks(mut self, blocks: &[SetBlock]) -> SetFilterBuilder {
        let values = blocks.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("block", &values);
        self
    }

    fn add_filter<T>(&mut self, key: T, values: T)
        where
            T: Display,
    {
        if !self.filter.is_empty() {
            self.filter = [&self.filter, "&"].join("");
        }
        self.filter = self.filter.clone() + &[key, values].into_iter().join("=")
    }
}

/// Wrapper around the filter string to be used for filtered set api requests
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetFilter(pub String);

impl SetFilter {
    /// Creates a new SetFilterBuilder
    #[allow(dead_code)]
    pub fn builder() -> SetFilterBuilder {
        SetFilterBuilder::new()
    }
}