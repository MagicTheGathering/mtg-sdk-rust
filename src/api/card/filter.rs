use api::card::filtertypes::*;
use itertools::Itertools;
use std::fmt::Display;

const SEP_AND: &str = ",";
const SEP_OR: &str = "|";

#[derive(Clone, Debug)]
pub struct CardFilterBuilder {
    filter: String,
}

impl CardFilterBuilder {
    fn new() -> CardFilterBuilder {
        CardFilterBuilder {
            filter: String::new(),
        }
    }

    /// Creates a CardFilter with the specified filter parameters
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder
    ///     .color(CardColor::Red)
    ///     .converted_mana_cost(2)
    ///     .cardtype(CardType::Instant)
    ///     .build();
    /// assert!(filter == CardFilter("colors=Red&cmc=2&types=Instant".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn build(self) -> CardFilter {
        CardFilter(self.filter)
    }

    /// Create a custom filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.custom("name", "Shock|Mountain")
    ///     .build();
    /// assert!(filter == CardFilter("name=Shock|Mountain".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn custom<'a, T>(mut self, key: T, value: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter(key.into(), value.into());
        self
    }

    /// Every card that (partially) matches the specified name will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.name("Shock")
    ///     .build();
    /// assert!(filter == CardFilter("name=Shock".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn name<'a, T>(mut self, name: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("name", name.into());
        self
    }

    /// Every card that (partially) matches one of the specified names will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.names(&vec!["Shock", "Mountain"])
    ///     .build();
    /// assert!(filter == CardFilter("name=Shock|Mountain".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn names<T>(mut self, names: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = names.into_iter().join(SEP_OR);
        self.add_filter("name", &values);
        self
    }

    /// Every card that (partially) matches the specified name will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.name_with_language("Schock", CardLanguage::German)
    ///     .build();
    /// assert!(filter == CardFilter("name=Schock&language=German".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn name_with_language<'a, T>(mut self, name: T, language: CardLanguage) -> CardFilterBuilder
    where
        T: Into<&'a str>
    {
        self.add_filter("name", name.into());
        self.add_filter("language", &language.as_str());
        self
    }

    /// Every card that (partially) matches one of the specified names will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.names_with_language(&vec!["Schock", "Gebirge"], CardLanguage::German)
    ///     .build();
    /// assert!(filter == CardFilter("name=Schock|Gebirge&language=German".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn names_with_language<T>(
        mut self,
        names: &[T],
        language: CardLanguage,
    ) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = names.into_iter().join(SEP_OR);
        self.add_filter("name", &values);
        self.add_filter("language", &language.as_str());
        self
    }

    /// Every card name that has the specified layout will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.layout(CardLayout::DoubleFaced)
    ///     .build();
    /// assert!(filter == CardFilter("layout=Double-Faced".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn layout(mut self, layout: CardLayout) -> CardFilterBuilder {
        self.add_filter("layout", &layout.as_str());
        self
    }

    /// Every card that has one of the specified layouts will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.layouts(&vec![CardLayout::Normal, CardLayout::DoubleFaced])
    ///     .build();
    /// assert!(filter == CardFilter("layout=Normal|Double-Faced".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn layouts(mut self, layouts: &[CardLayout]) -> CardFilterBuilder {
        let values = layouts.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("layout", &values);
        self
    }

    /// Every card name that has the specified converted mana cost will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.converted_mana_cost(3)
    ///     .build();
    /// assert!(filter == CardFilter("cmc=3".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn converted_mana_cost(mut self, cmc: u8) -> CardFilterBuilder {
        self.add_filter("cmc", &cmc.to_string());
        self
    }

    /// Every card that has one of the specified converted mana costs will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.converted_mana_costs(&vec![3,5])
    ///     .build();
    /// assert!(filter == CardFilter("cmc=3|5".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn converted_mana_costs(mut self, cmcs: &[u8]) -> CardFilterBuilder {
        let values = cmcs.into_iter().map(u8::to_string).join(SEP_OR);
        self.add_filter("cmc", &values);
        self
    }

    /// Every card that includes the specified color will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.color(CardColor::Red)
    ///     .build();
    /// assert!(filter == CardFilter("colors=Red".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn color(mut self, color: CardColor) -> CardFilterBuilder {
        self.add_filter("colors", &color.as_str());
        self
    }

    /// Every card that includes one the specified colors will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.colors_or(&vec![CardColor::Red, CardColor::Blue])
    ///     .build();
    /// assert!(filter == CardFilter("colors=Red|Blue".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn colors_or(mut self, colors: &[CardColor]) -> CardFilterBuilder {
        let values = colors.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("colors", &values);
        self
    }

    /// Every card that includes all the specified colors will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.colors_and(&vec![CardColor::Red, CardColor::Blue])
    ///     .build();
    /// assert!(filter == CardFilter("colors=Red,Blue".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn colors_and(mut self, colors: &[CardColor]) -> CardFilterBuilder {
        let values = colors.into_iter().map(|value| value.as_str()).join(SEP_AND);
        self.add_filter("colors", &values);
        self
    }

    /// Every card that includes the specified color code will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.color_identity(CardColorIdentity::R)
    ///     .build();
    /// assert!(filter == CardFilter("colorIdentity=R".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn color_identity(mut self, color_identity: CardColorIdentity) -> CardFilterBuilder {
        self.add_filter("colorIdentity", &color_identity.as_str());
        self
    }

    /// Every card that includes one of the specified color codes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.color_identities_or(&vec![CardColorIdentity::R, CardColorIdentity::U])
    ///     .build();
    /// assert!(filter == CardFilter("colorIdentity=R|U".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn color_identities_or(
        mut self,
        color_identities: &[CardColorIdentity],
    ) -> CardFilterBuilder {
        let values = color_identities
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_OR);
        self.add_filter("colorIdentity", &values);
        self
    }

    /// Every card that includes all of the specified color codes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.color_identities_and(&vec![CardColorIdentity::R, CardColorIdentity::U])
    ///     .build();
    /// assert!(filter == CardFilter("colorIdentity=R,U".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn color_identities_and(
        mut self,
        color_identities: &[CardColorIdentity],
    ) -> CardFilterBuilder {
        let values = color_identities
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_AND);
        self.add_filter("colorIdentity", &values);
        self
    }

    /// Every card that (partially) matches the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.fulltype("Legendary Creature")
    ///     .build();
    /// assert!(filter == CardFilter("types=Legendary Creature".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn fulltype<'a, T>(mut self, fulltype: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("types", &fulltype.into());
        self
    }

    /// Every card that (partially) matches one of the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.fulltypes_or(&vec!["Legendary Creature", "Human"])
    ///     .build();
    /// assert!(filter == CardFilter("types=Legendary Creature|Human".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn fulltypes_or<T>(mut self, fulltypes: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = fulltypes.into_iter().join(SEP_OR);
        self.add_filter("types", &values);
        self
    }

    /// Every card that (partially) matches all of the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.fulltypes_and(&vec!["Legendary", "Creature", "Human"])
    ///     .build();
    /// assert!(filter == CardFilter("types=Legendary,Creature,Human".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn fulltypes_and<T>(mut self, fulltypes: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = fulltypes.into_iter().join(SEP_AND);
        self.add_filter("types", &values);
        self
    }

    /// Every card that is of the specified supertype will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.supertype(CardSuperType::Legendary)
    ///     .build();
    /// assert!(filter == CardFilter("supertypes=Legendary".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn supertype(mut self, supertype: CardSuperType) -> CardFilterBuilder {
        self.add_filter("supertypes", &supertype.as_str());
        self
    }

    /// Every card that is one of the specified supertypes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.supertypes_or(&vec![CardSuperType::Basic, CardSuperType::Legendary])
    ///     .build();
    /// assert!(filter == CardFilter("supertypes=Basic|Legendary".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn supertypes_or(mut self, supertypes: &[CardSuperType]) -> CardFilterBuilder {
        let values = supertypes
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_OR);
        self.add_filter("supertypes", &values);
        self
    }

    /// Every card that is all of the specified supertypes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.supertypes_and(&vec![CardSuperType::Basic, CardSuperType::Legendary])
    ///     .build();
    /// assert!(filter == CardFilter("supertypes=Basic,Legendary".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn supertypes_and(mut self, supertypes: &[CardSuperType]) -> CardFilterBuilder {
        let values = supertypes
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_AND);
        self.add_filter("supertypes", &values);
        self
    }

    /// Every card that is of the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.cardtype(CardType::Creature)
    ///     .build();
    /// assert!(filter == CardFilter("types=Creature".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn cardtype(mut self, cardtype: CardType) -> CardFilterBuilder {
        self.add_filter("types", &cardtype.as_str());
        self
    }

    /// Every card that is of one of the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.cardtypes_or(&vec![CardType::Creature, CardType::Artifact])
    ///     .build();
    /// assert!(filter == CardFilter("types=Creature|Artifact".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn cardtypes_or(mut self, cardtypes: &[CardType]) -> CardFilterBuilder {
        let values = cardtypes
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_OR);
        self.add_filter("types", &values);
        self
    }

    /// Every card that is of all of the specified types will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.cardtypes_and(&vec![CardType::Creature, CardType::Artifact])
    ///     .build();
    /// assert!(filter == CardFilter("types=Creature,Artifact".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn cardtypes_and(mut self, cardtypes: &[CardType]) -> CardFilterBuilder {
        let values = cardtypes
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_AND);
        self.add_filter("types", &values);
        self
    }

    /// Every card that is of the specified subtype will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.subtype("Human")
    ///     .build();
    /// assert!(filter == CardFilter("subtypes=Human".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn subtype<'a, T>(mut self, subtype: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("subtypes", &subtype.into());
        self
    }

    /// Every card that is of one of the specified subtypes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.subtypes_or(&vec!["Human", "Soldier"])
    ///     .build();
    /// assert!(filter == CardFilter("subtypes=Human|Soldier".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn subtypes_or<T>(mut self, subtypes: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = subtypes.into_iter().join(SEP_OR);
        self.add_filter("subtypes", &values);
        self
    }

    /// Every card that is of all of the specified subtypes will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.subtypes_and(&vec!["Human", "Soldier"])
    ///     .build();
    /// assert!(filter == CardFilter("subtypes=Human,Soldier".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn subtypes_and<T>(mut self, subtypes: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = subtypes.into_iter().join(SEP_AND);
        self.add_filter("subtypes", &values);
        self
    }

    /// Every card that is of the specified rarity will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.rarity(CardRarity::Rare)
    ///     .build();
    /// assert!(filter == CardFilter("rarity=Rare".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn rarity(mut self, rarity: CardRarity) -> CardFilterBuilder {
        self.add_filter("rarity", &rarity.as_str());
        self
    }

    /// Every card that is of one of the specified rarities will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.rarities(&vec![CardRarity::Rare, CardRarity::MythicRare])
    ///     .build();
    /// assert!(filter == CardFilter("rarity=Rare|Mythic Rare".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn rarities(mut self, rarities: &[CardRarity]) -> CardFilterBuilder {
        let values = rarities
            .into_iter()
            .map(|value| value.as_str())
            .join(SEP_OR);
        self.add_filter("rarity", &values);
        self
    }

    /// Every card that is in the specified set will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.set("AER")
    ///     .build();
    /// assert!(filter == CardFilter("set=AER".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn set<'a, T>(mut self, set: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("set", &set.into());
        self
    }

    /// Every card that is in one of the specified sets will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.sets(&vec!["AER", "M19"])
    ///     .build();
    /// assert!(filter == CardFilter("set=AER|M19".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn sets<T>(mut self, sets: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = sets.into_iter().join(SEP_OR);
        self.add_filter("set", &values);
        self
    }

    /// Every card that (partially) matches the specified set name will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.set_name("Core Set 2019")
    ///     .build();
    /// assert!(filter == CardFilter("setName=Core Set 2019".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn set_name<'a, T>(mut self, set: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("setName", &set.into());
        self
    }

    /// Every card that (partially) matches one of the specified set names will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.set_names(&vec!["Core Set 2019", "Aether Revolt"])
    ///     .build();
    /// assert!(filter == CardFilter("setName=Core Set 2019|Aether Revolt".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn set_names<T>(mut self, sets: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = sets.into_iter().join(SEP_OR);
        self.add_filter("setName", &values);
        self
    }

    /// Every card that (partially) matches the specified oracle text will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.text("deals 2 damage")
    ///     .build();
    /// assert!(filter == CardFilter("text=deals 2 damage".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn text<'a, T>(mut self, text: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("text", &text.into());
        self
    }

    /// Every card that (partially) matches one of the specified oracle texts will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.texts_or(&vec!["deals", "damage"])
    ///     .build();
    /// assert!(filter == CardFilter("text=deals|damage".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn texts_or<T>(mut self, texts: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = texts.into_iter().join(SEP_OR);
        self.add_filter("text", &values);
        self
    }

    /// Every card that (partially) matches all of the specified oracle texts will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.texts_and(&vec!["deals", "damage"])
    ///     .build();
    /// assert!(filter == CardFilter("text=deals,damage".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn texts_and<T>(mut self, texts: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = texts.into_iter().join(SEP_AND);
        self.add_filter("text", &values);
        self
    }

    /// Every card that (partially) matches the specified flavour text will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.flavor("S.N.E.A.K.")
    ///     .build();
    /// assert!(filter == CardFilter("flavor=S.N.E.A.K.".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn flavor<'a, T>(mut self, flavor: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("flavor", &flavor.into());
        self
    }

    /// Every card that (partially) matches one of the specified flavour texts will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.flavors_or(&vec!["Espionage", "Kidnapping"])
    ///     .build();
    /// assert!(filter == CardFilter("flavor=Espionage|Kidnapping".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn flavors_or<T>(mut self, flavors: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = flavors.into_iter().join(SEP_OR);
        self.add_filter("flavor", &values);
        self
    }

    /// Every card that (partially) matches all of the specified flavour texts will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.flavors_and(&vec!["Serious", "Nonstop Espionage and Kidnapping"])
    ///     .build();
    /// assert!(filter == CardFilter("flavor=Serious,Nonstop Espionage and Kidnapping".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn flavors_and<T>(mut self, flavors: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = flavors.into_iter().join(SEP_AND);
        self.add_filter("flavor", &values);
        self
    }

    /// Every card that is drawn by the specified artist will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.artist("Kev Walker")
    ///     .build();
    /// assert!(filter == CardFilter("artist=Kev Walker".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn artist<'a, T>(mut self, artist: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("artist", &artist.into());
        self
    }

    /// Every card that is drawn by one of the specified artists will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.artists(&vec!["Kev Walker", "Pete Venters"])
    ///     .build();
    /// assert!(filter == CardFilter("artist=Kev Walker|Pete Venters".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn artists<T>(mut self, artists: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = artists.into_iter().join(SEP_OR);
        self.add_filter("artist", &values);
        self
    }

    /// Every card with the specified card number will match the filter
    ///
    /// The card number may contain letters
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.number("1")
    ///     .build();
    /// assert!(filter == CardFilter("number=1".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn number<'a, T>(mut self, number: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("number", &number.into());
        self
    }

    /// Every card with one of the specified card numbers will match the filter
    ///
    /// The card number may contain letters
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.numbers(&vec!["1", "2"])
    ///     .build();
    /// assert!(filter == CardFilter("number=1|2".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn numbers<T>(mut self, numbers: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = numbers.into_iter().join(SEP_OR);
        self.add_filter("number", &values);
        self
    }

    /// Every card with the specified power will match the filter
    ///
    /// Some cards have powers like: “1+*”
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.power("1")
    ///     .build();
    /// assert!(filter == CardFilter("power=1".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn power<'a, T>(mut self, power: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("power", &power.into());
        self
    }

    /// Every card with one of the specified powers will match the filter
    ///
    /// Some cards have powers like: “1+*”
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.powers(&vec!["1", "2"])
    ///     .build();
    /// assert!(filter == CardFilter("power=1|2".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn powers<T>(mut self, powers: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = powers.into_iter().join(SEP_OR);
        self.add_filter("power", &values);
        self
    }

    /// Every card with the specified toughness will match the filter
    ///
    /// Some cards have toughness like: “1+*”
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.toughness("1")
    ///     .build();
    /// assert!(filter == CardFilter("toughness=1".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn toughness<'a, T>(mut self, toughness: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("toughness", &toughness.into());
        self
    }

    /// Every card with one of the specified toughnesses will match the filter
    ///
    /// Some cards have toughnesses like: “1+*”
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.toughnesses(&vec!["1", "2"])
    ///     .build();
    /// assert!(filter == CardFilter("toughness=1|2".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn toughnesses<T>(mut self, toughnesses: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = toughnesses.into_iter().join(SEP_OR);
        self.add_filter("toughness", &values);
        self
    }

    /// Every card with the specified loyality will match the filter
    ///
    /// This is only present for planeswalkers
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.loyality("3")
    ///     .build();
    /// assert!(filter == CardFilter("loyality=3".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn loyality<'a, T>(mut self, loyality: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("loyality", &loyality.into());
        self
    }

    /// Every card with one of the specified loyalities will match the filter
    ///
    /// This is only present for planeswalkers
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.loyalities(&vec!["3", "5"])
    ///     .build();
    /// assert!(filter == CardFilter("loyality=3|5".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn loyalities<T>(mut self, loyalities: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = loyalities.into_iter().join(SEP_OR);
        self.add_filter("loyality", &values);
        self
    }

    /// Every card that is legal in the specified game format will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.game_format(GameFormat::Standard)
    ///     .build();
    /// assert!(filter == CardFilter("gameFormat=Standard".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn game_format(mut self, format: GameFormat) -> CardFilterBuilder {
        self.add_filter("gameFormat", &format.as_str());
        self
    }

    /// Every card that is legal in the specified game formats will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.game_format(GameFormat::Standard)
    ///     .build();
    /// assert!(filter == CardFilter("gameFormat=Standard".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn game_formats(mut self, formats: &[GameFormat]) -> CardFilterBuilder {
        let values = formats.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("gameFormat", &values);
        self
    }

    /// Every card that is of the specified legality in the specified game format will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.game_format_with_legality(GameFormat::Commander, CardLegality::Banned)
    ///     .build();
    /// assert!(filter == CardFilter("gameFormat=Commander&legality=Banned".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn game_format_with_legality(
        mut self,
        format: GameFormat,
        legality: CardLegality,
    ) -> CardFilterBuilder {
        self.add_filter("gameFormat", &format.as_str());
        self.add_filter("legality", &legality.as_str());
        self
    }

    /// Every card that is of the specified legality in the specified game formats will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.game_formats_with_legality(&vec![GameFormat::Standard, GameFormat::Commander], CardLegality::Banned)
    ///     .build();
    /// assert!(filter == CardFilter("gameFormat=Standard|Commander&legality=Banned".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn game_formats_with_legality(
        mut self,
        formats: &[GameFormat],
        legality: CardLegality,
    ) -> CardFilterBuilder {
        let values = formats.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("gameFormat", &values);
        self.add_filter("legality", &legality.as_str());
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

    /// Every card with the specified multiverse Id will match the filter
    ///
    /// This is only present for planeswalkers
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.multiverse_id("409741")
    ///     .build();
    /// assert!(filter == CardFilter("multiverseid=409741".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn multiverse_id<'a, T>(mut self, multiverse_id: T) -> CardFilterBuilder
    where
        T: Into<&'a str>,
    {
        self.add_filter("multiverseid", &multiverse_id.into());
        self
    }

    /// Every card with one of the specified multiverse Ids will match the filter
    ///
    /// This is only present for planeswalkers
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.multiverse_ids(&vec!["409741", "409742"])
    ///     .build();
    /// assert!(filter == CardFilter("multiverseid=409741|409742".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn multiverse_ids<T>(mut self, multiverse_ids: &[T]) -> CardFilterBuilder
    where
        T: Display,
    {
        let values = multiverse_ids.into_iter().join(SEP_OR);
        self.add_filter("multiverseid", &values);
        self
    }

    /// Every card that contains the specified field in the response will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.contains_field(CardResponseField::ImageUrl)
    ///     .build();
    /// assert!(filter == CardFilter("contains=imageUrl".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn contains_field(mut self, field: CardResponseField) -> CardFilterBuilder {
        self.add_filter("contains", &field.as_str());
        self
    }

    /// Every card that contains one of the specified fields in the response will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = CardFilter::builder();
    /// let filter = builder.contains_fields(&vec![CardResponseField::ImageUrl, CardResponseField::MultiverseId])
    ///     .build();
    /// assert!(filter == CardFilter("contains=imageUrl|multiverseid".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn contains_fields(mut self, fields: &[CardResponseField]) -> CardFilterBuilder {
        let values = fields.into_iter().map(|value| value.as_str()).join(SEP_OR);
        self.add_filter("contains", &values);
        self
    }
}

/// Wrapper around the filter string to be used for filtered card api requests
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CardFilter(pub String);

impl CardFilter {
    /// Creates a new CardFilterBuilder
    #[allow(dead_code)]
    pub fn builder() -> CardFilterBuilder {
        CardFilterBuilder::new()
    }
}
