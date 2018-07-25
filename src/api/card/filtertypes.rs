/// Available languages for the language filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardLanguage {
    ChineseSimplified,
    ChineseTraditional,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Russian,
    Spanish,
}

impl CardLanguage {
    #[allow(dead_code)]
    /// Creates the representation expected by the language filter
    pub fn as_str(self) -> &'static str {
        use self::CardLanguage::*;
        match self {
            ChineseSimplified => "Chinese Simplified",
            ChineseTraditional => "Chinese Traditional",
            French => "French",
            German => "German",
            Italian => "Italian",
            Japanese => "Japanese",
            Korean => "Korean",
            Portuguese => "Portuguese",
            Russian => "Russian",
            Spanish => "Spanish",
        }
    }
}

/// Available layouts for the layout filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardLayout {
    Normal,
    Split,
    Flip,
    DoubleFaced,
    Token,
    Plane,
    Scheme,
    Phenomenon,
    Leveler,
    Vanguard,
    Aftermath,
}

impl CardLayout {
    /// Creates the representation expected by the layout filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardLayout::*;
        match self {
            Normal => "Normal",
            Split => "Split",
            Flip => "Flip",
            DoubleFaced => "Double-Faced",
            Token => "Token",
            Plane => "Plane",
            Scheme => "Scheme",
            Phenomenon => "Phenomen",
            Leveler => "Level",
            Vanguard => "Vanguard",
            Aftermath => "Aftermath",
        }
    }
}

/// Available colors for the color filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl CardColor {
    /// Creates the representation expected by the color filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardColor::*;
        match self {
            White => "White",
            Blue => "Blue",
            Black => "Black",
            Red => "Red",
            Green => "Green",
        }
    }
}

/// Available color identities for the color identity filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardColorIdentity {
    W,
    U,
    B,
    R,
    G,
}

impl CardColorIdentity {
    /// Creates the representation expected by the color identities filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardColorIdentity::*;
        match self {
            W => "W",
            U => "U",
            B => "B",
            R => "R",
            G => "G",
        }
    }
}

/// Available supertypes for the supertype filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardSuperType {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

impl CardSuperType {
    /// Creates the representation expected by the supertypes filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardSuperType::*;
        match self {
            Basic => "Basic",
            Legendary => "Legendary",
            Ongoing => "Ongoing",
            Snow => "Snow",
            World => "World",
        }
    }
}

/// Available card types for the card type filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardType {
    Artifact,
    Conspiracy,
    Creature,
    Enchantment,
    Host,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

impl CardType {
    /// Creates the representation expected by the card types filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardType::*;
        match self {
            Artifact => "Artifact",
            Conspiracy => "Conspiracy",
            Creature => "Creature",
            Enchantment => "Enchantment",
            Host => "Host",
            Instant => "Instant",
            Land => "Land",
            Phenomenon => "Phenomenon",
            Plane => "Plane",
            Planeswalker => "Planeswalker",
            Scheme => "Scheme",
            Sorcery => "Sorcery",
            Tribal => "Tribal",
            Vanguard => "Vanguard",
        }
    }
}

/// Available rarities for the rarity filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
    Special,
    BasicLand,
}

impl CardRarity {
    /// Creates the representation expected by the rarity filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardRarity::*;
        match self {
            Common => "Common",
            Uncommon => "Uncommon",
            Rare => "Rare",
            MythicRare => "Mythic Rare",
            Special => "Special",
            BasicLand => "Basic Land",
        }
    }
}

/// Available legalities for the legality filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardLegality {
    Banned,
    Restricted,
    Legal,
}

impl CardLegality {
    /// Creates the representation expected by the legality filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardLegality::*;
        match self {
            Banned => "Banned",
            Restricted => "Restricted",
            Legal => "Legal",
        }
    }
}

/// Available game formats for the game format filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameFormat {
    AmonkhetBlock,
    BattleForZendikarBlock,
    Brawl,
    Classic,
    Commander,
    Extended,
    IceAgeBlock,
    InnistradBlock,
    InvasionBlock,
    IxalanBlock,
    KaladeshBlock,
    KamigawaBlock,
    KhansOfTarkirBlock,
    Legacy,
    LorwynShadowmoorBlock,
    MasquesBlock,
    MirageBlock,
    MirrodinBlock,
    Modern,
    OdysseyBlock,
    OnslaughtBlock,
    RavnicaBlock,
    ReturnToRavnicaBlock,
    ScarsOfMirrodinBlock,
    ShadowsOverInnistradBlock,
    ShardsOfAlaraBlock,
    Standard,
    TempestBlock,
    TherosBlock,
    TimeSpiralBlock,
    UnSets,
    UrzaBlock,
    Vintage,
    ZendikarBlock,
}

impl GameFormat {
    /// Creates the representation expected by the game format filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::GameFormat::*;
        match self {
            AmonkhetBlock => "Amonkhet Block",
            BattleForZendikarBlock => "Battle for Zendikar Block",
            Brawl => "Brawl",
            Classic => "Classic",
            Commander => "Commander",
            Extended => "Extended",
            IceAgeBlock => "Ice Age Block",
            InnistradBlock => "Innistrad Block",
            InvasionBlock => "Invasion Block",
            IxalanBlock => "Ixalan Block",
            KaladeshBlock => "Kaladesh Block",
            KamigawaBlock => "Kamigawa Block",
            KhansOfTarkirBlock => "Khans of Tarkir Block",
            Legacy => "Legacy",
            LorwynShadowmoorBlock => "Lorwyn-Shadowmoor Block",
            MasquesBlock => "Masques Block",
            MirageBlock => "Mirage Block",
            MirrodinBlock => "Mirrodin Block",
            Modern => "Modern",
            OdysseyBlock => "Odyssey Block",
            OnslaughtBlock => "Onslaught Block",
            RavnicaBlock => "Ravnica Block",
            ReturnToRavnicaBlock => "Return to Ravnica Block",
            ScarsOfMirrodinBlock => "Scars of Mirrodin Block",
            ShadowsOverInnistradBlock => "Shadows over Innistrad Block",
            ShardsOfAlaraBlock => "Shards of Alara Block",
            Standard => "Standard",
            TempestBlock => "Tempest Block",
            TherosBlock => "Theros Block",
            TimeSpiralBlock => "Time Spiral Block",
            UnSets => "Un-Sets",
            UrzaBlock => "Urza Block",
            Vintage => "Vintage",
            ZendikarBlock => "Zendikar Block",
        }
    }
}

/// Available game formats for the response field filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardResponseField {
    Type,
    ColorIdentity,
    ManaCost,
    SetName,
    ImageUrl,
    OriginalText,
    OriginalType,
    Name,
    Cmc,
    Colors,
    Types,
    Subtypes,
    Rarity,
    Set,
    Text,
    Artist,
    Number,
    Power,
    Toughness,
    Layout,
    MultiverseId,
    Rulings,
    Printing,
    Legalities,
    Id,
}

impl CardResponseField {
    /// Creates the representation expected by the response field filter
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        use self::CardResponseField::*;
        match self {
            Type => "type",
            ColorIdentity => "colorIdentity",
            ManaCost => "manaCost",
            SetName => "setName",
            ImageUrl => "imageUrl",
            OriginalText => "originalText",
            OriginalType => "originalType",
            Name => "name",
            Cmc => "cmc",
            Colors => "colors",
            Types => "types",
            Subtypes => "subtypes",
            Rarity => "rarity",
            Set => "set",
            Text => "text",
            Artist => "artist",
            Number => "number",
            Power => "power",
            Toughness => "toughness",
            Layout => "layout",
            MultiverseId => "multiverseid",
            Rulings => "rulings",
            Printing => "printing",
            Legalities => "legalities",
            Id => "id",
        }
    }
}
