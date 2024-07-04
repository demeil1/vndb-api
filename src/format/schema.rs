use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "ck")]
    Cherokee,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "zh-Hans")]
    ChineseSimplified,
    #[serde(rename = "zh-Hant")]
    ChineseTraditional,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "eo")]
    Esperanto,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "iu")]
    Inuktitut,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "mk")]
    Macedonian,
    #[serde(rename = "ms")]
    Malay,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt-br")]
    PortugueseBrazil,
    #[serde(rename = "pt-pt")]
    PortuguesePortugal,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "gd")]
    ScottishGaelic,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "sl")]
    Slovene,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "tg")]
    Tagalog,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "vi")]
    Vietnamese,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Medium {
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "dvd")]
    Dvd,
    #[serde(rename = "gdr")]
    GdRom,
    #[serde(rename = "blr")]
    BlueRayDisc,
    #[serde(rename = "flp")]
    FloppyDisc,
    #[serde(rename = "cas")]
    CassetteTape,
    #[serde(rename = "mrt")]
    Cartridge,
    #[serde(rename = "mem")]
    MemoryCard,
    #[serde(rename = "umd")]
    Umd,
    #[serde(rename = "nod")]
    NintendoOpticalDisc,
    #[serde(rename = "in")]
    InternetDownload,
    #[serde(rename = "dc")]
    DownloadCard,
    #[serde(rename = "otc")]
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StaffRole {
    Scenario,
    Director,
    /// Character Design
    CharDesign,
    /// Artist
    Art,
    /// Composer
    Music,
    /// Vocals
    Songs,
    Translator,
    Editor,
    /// Quality Assurance
    Qa,
    Staff,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Platform {
    #[serde(rename = "win")]
    Windows,
    #[serde(rename = "lin")]
    Linux,
    #[serde(rename = "mac")]
    MacOs,
    #[serde(rename = "web")]
    Website,
    #[serde(rename = "tdo")]
    ThreeDO,
    #[serde(rename = "ios")]
    Apple,
    #[serde(rename = "and")]
    Android,
    #[serde(rename = "bdp")]
    BluRayPlayer,
    #[serde(rename = "dos")]
    Dos,
    #[serde(rename = "dvd")]
    DvdPlayer,
    #[serde(rename = "drc")]
    Dreamcast,
    #[serde(rename = "nes")]
    Famicom,
    #[serde(rename = "sfc")]
    SuperFamicom,
    #[serde(rename = "fm7")]
    Fm7,
    #[serde(rename = "fm8")]
    Fm8,
    #[serde(rename = "fmt")]
    FMTowns,
    #[serde(rename = "gba")]
    GameBoyAdvance,
    #[serde(rename = "gbc")]
    GameBoyColor,
    #[serde(rename = "msx")]
    Msx,
    #[serde(rename = "nds")]
    NintendoDS,
    #[serde(rename = "swi")]
    NintendoSwitch,
    #[serde(rename = "wii")]
    NintendoWii,
    #[serde(rename = "wiu")]
    NintendoWiiU,
    #[serde(rename = "n3d")]
    Nintendo3DS,
    #[serde(rename = "p88")]
    Pc88,
    #[serde(rename = "p98")]
    Pc98,
    #[serde(rename = "pce")]
    PcEngine,
    #[serde(rename = "pcf")]
    PcFX,
    #[serde(rename = "psp")]
    PlayStationPortable,
    #[serde(rename = "ps1")]
    PlayStation1,
    #[serde(rename = "ps2")]
    PlayStation2,
    #[serde(rename = "ps3")]
    PlayStation3,
    #[serde(rename = "ps4")]
    PlayStation4,
    #[serde(rename = "ps5")]
    PlayStation5,
    #[serde(rename = "psv")]
    PlayStationVita,
    #[serde(rename = "smd")]
    SegaMegaDrive,
    #[serde(rename = "scd")]
    SegaMegaCD,
    #[serde(rename = "sat")]
    SegaSaturn,
    #[serde(rename = "vnd")]
    Vnds,
    #[serde(rename = "x1s")]
    SharpX1,
    #[serde(rename = "x68")]
    SharpX68000,
    #[serde(rename = "xb1")]
    Xbox,
    #[serde(rename = "xb3")]
    Xbox360,
    #[serde(rename = "xbo")]
    XboxOne,
    /// For Xbox X and Xbox S
    #[serde(rename = "xxs")]
    XboxX,
    #[serde(rename = "mob")]
    OtherMobile,
    #[serde(rename = "oth")]
    Other,
}
