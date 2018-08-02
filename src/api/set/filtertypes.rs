/// Available languages for the language filter
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SetBlock {
    IceAge,
    Mirage,
    Tempest,
    Urza,
    Masques,
    Invasion,
    Odyssey,
    Onslaught,
    Mirrodin,
    Kamigawa,
    Ravnica,
    TimeSpiral,
    Lorwyn,
    Shadowmoor,
    Alara,
    Zendikar,
    ScarsOfMirrodin,
    Innistrad,
    ReturnToRavnica,
    Theros,
    KhansOfTarkir,
    BattleForZendikar,
    ShadowsOverInnistrad,
    Kaladesh,
    Amonkhet,
    Ixalan
}

impl SetBlock {
    #[allow(dead_code)]
    /// Creates the representation expected by the language filter
    pub fn as_str(self) -> &'static str {
        use self::SetBlock::*;
        match self {
            IceAge => "Ice Age",
            Mirage => "Mirage",
            Tempest => "Tempest",
            Urza => "Urza",
            Masques => "Masques",
            Invasion => "Invasion",
            Odyssey => "Odyssey",
            Onslaught => "Onslaught",
            Mirrodin => "Mirrodin",
            Kamigawa => "Kamigawa",
            Ravnica => "Ravnica",
            TimeSpiral => "TimeSpiral",
            Lorwyn => "Lorwyn",
            Shadowmoor => "Shadowmoor",
            Alara => "Alara",
            Zendikar => "Zendikar",
            ScarsOfMirrodin => "ScarsOfMirrodin",
            Innistrad => "Innistrad",
            ReturnToRavnica => "Return to Ravnica",
            Theros => "Theros",
            KhansOfTarkir => "Khans of Tarkir",
            BattleForZendikar => "Battle for Zendikar",
            ShadowsOverInnistrad => "Shadows over Innistrad",
            Kaladesh => "Kaladesh",
            Amonkhet => "Amonkhet",
            Ixalan => "Ixalan"
        }
    }
}