use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr, sync::OnceLock};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

const GAMES_TOML_SRC: &str = include_str!("../../../static/games.toml");
const GENERIC_IGNORE: &[&str] = &[
    "java", "javaw", "python", "python3", "wine", "wine64", "proton", "steam",
];
static INSTANCE: OnceLock<GamesDb> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipsGameTag {
    Fps,
    Rpg,
    OpenWorld,
    Survival,
    Coop,
    Competitive,
    Soulslike,
    Roguelike,
    Crafting,
    Strategy,
    Dynamism,
    Action,
    Adventure,
    Simulation,
    Sports,
    Racing,
    Sandbox,
    Mmo,
    Moba,
    Puzzle,
    Horror,
    Platformer,
    Fighting,
    Deckbuilder,
    Casual,
    HackAndSlash,
    Shooter,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    AsRefStr,
    zbus::zvariant::Type,
)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipsGames {
    TheFinals,
    CounterStrike2,
    Dota2,
    Pubg,
    ApexLegends,
    GrandTheftAutoV,
    Rust,
    RainbowSixSiege,
    TeamFortress2,
    Left4Dead2,
    Portal2,
    HalfLife2,
    EldenRing,
    DarkSoulsIII,
    BaldursGate3,
    Cyberpunk2077,
    TheWitcher3,
    Terraria,
    StardewValley,
    Minecraft,
    Factorio,
    Satisfactory,
    Hades,
    HollowKnight,
    DeepRockGalactic,
    Warframe,
    Destiny2,
    Fortnite,
    LeagueOfLegends,
    Overwatch2,
    WorldOfWarcraft,
    PathOfExile,
    PathOfExile2,
    DiabloIV,
    SeaOfThieves,
    NoMansSky,
    Subnautica,
    ArkSurvivalEvolved,
    RocketLeague,
    FallGuys,
    AmongUs,
    ItTakesTwo,
    MonsterHunterWorld,
    Palworld,
    Helldivers2,
    LethalCompany,
    EscapeFromTarkov,
    Squad,
    Arma3,
    DayZ,
    InsurgencySandstorm,
    KillingFloor2,
    Valheim,
    SeaOfStars,
    Ets2,
    MarvelRivals,
    BlackMythWukong,
    Starfield,
    RedDeadRedemption2,
    CitiesSkylines2,
    CivilizationVI,
    FootballManager,
    EaSportsFc,
    CallOfDutyWarzone,
    CallOfDutyModernWarfare3,
    CallOfDutyBlackOps6,
    Battlefield2042,
    Payday3,
    RiskOfRain2,
    VampireSurvivors,
    SonsOfTheForest,
    TheForest,
    GreenHell,
    Phasmophobia,
    Repo,
    Grounded,
    VRising,
    CoreKeeper,
    ProjectZomboid,
    Enshrouded,
    SlayTheSpire,
    Balatro,
    WarhammerDarktide,
    Remnant2,
    LiesOfP,
    Sekiro,
    Skyrim,
    Fallout4,
    Fallout76,
    Kenshi,
    RimWorld,
    OxygenNotIncluded,
    CultOfTheLamb,
    DontStarveTogether,
    Raft,
    ReadyOrNot,
    Gtfo,
    Back4Blood,
    Vermintide2,
    Chivalry2,
    Mordhau,
    ForHonor,
    HaloInfinite,
    Titanfall2,
    Splitgate,
    XDefiant,
    NarakaBladepoint,
    Smite,
    Brawlhalla,
    MultiVersus,
    StreetFighter6,
    Tekken8,
    MortalKombat1,
    Unknown,
}

impl fmt::Display for ClipsGames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

impl FromStr for ClipsGames {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.eq_ignore_ascii_case("unknown") {
            return Ok(Self::Unknown);
        }

        if let Some(game) = Self::from_variant_ident(trimmed) {
            return Ok(game);
        }

        let db = GamesDb::get();
        let lower = trimmed.to_ascii_lowercase();

        if let Some(&game) = db.name_or_slug_to_variant.get(&lower) {
            return Ok(game);
        }

        match db.fuzzy_match(trimmed) {
            Self::Unknown => Err(format!("unknown game: {s}")),
            game => Ok(game),
        }
    }
}

impl ClipsGames {
    pub fn from_variant_ident(s: &str) -> Option<Self> {
        Self::iter().find(|g| g.as_ref() == s)
    }

    pub fn from_title(title: &str) -> Self {
        GamesDb::get().fuzzy_match(title)
    }

    pub fn from_process_name(process: &str) -> Self {
        GamesDb::get().fuzzy_match(process)
    }

    pub fn from_steam_appid(appid: u64) -> Self {
        GamesDb::get()
            .appid_to_variant
            .get(&appid)
            .copied()
            .unwrap_or(Self::Unknown)
    }

    pub fn display_name(&self) -> &'static str {
        GamesDb::get()
            .record_for(*self)
            .map(|r| r.display_name.as_str())
            .unwrap_or("Unknown")
    }

    pub fn slug(&self) -> &'static str {
        GamesDb::get()
            .record_for(*self)
            .map(|r| r.slug.as_str())
            .unwrap_or("unknown")
    }

    pub fn steam_appids(&self) -> &'static [u64] {
        GamesDb::get()
            .record_for(*self)
            .map(|r| r.steam_appids.as_slice())
            .unwrap_or(&[])
    }

    pub fn all() -> impl Iterator<Item = ClipsGames> {
        ClipsGames::iter().filter(|g| *g != ClipsGames::Unknown)
    }
}

#[derive(Debug, Deserialize)]
struct GameEntry {
    variant: String,
    slug: String,
    display_name: String,
    #[serde(default)]
    steam_appids: Vec<u64>,
    #[serde(default)]
    titles: Vec<String>,
    #[serde(default)]
    processes: Vec<String>,
    #[serde(default)]
    prefixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GamesFile {
    #[serde(rename = "game")]
    games: Vec<GameEntry>,
}

struct GameRecord {
    variant: ClipsGames,
    slug: String,
    display_name: String,
    steam_appids: Vec<u64>,
    contains_patterns: Vec<String>,
    prefix_patterns: Vec<String>,
}

struct GamesDb {
    records: Vec<GameRecord>,
    variant_to_index: HashMap<ClipsGames, usize>,
    appid_to_variant: HashMap<u64, ClipsGames>,
    name_or_slug_to_variant: HashMap<String, ClipsGames>,
    fuzzy_order: Vec<usize>,
}

impl GamesDb {
    fn get() -> &'static Self {
        INSTANCE.get_or_init(Self::init)
    }

    fn init() -> Self {
        let parsed: GamesFile = toml::from_str(GAMES_TOML_SRC).expect("games.toml is invalid");

        let mut records = Vec::with_capacity(parsed.games.len());
        let mut variant_to_index = HashMap::with_capacity(parsed.games.len());
        let mut appid_to_variant = HashMap::new();
        let mut name_or_slug_to_variant = HashMap::new();

        for entry in parsed.games {
            let variant = ClipsGames::from_variant_ident(&entry.variant).unwrap_or_else(|| {
                panic!(
                    "games.toml contains variant '{}' with no matching enum",
                    entry.variant
                )
            });

            let idx = records.len();
            variant_to_index.insert(variant, idx);

            for appid in &entry.steam_appids {
                appid_to_variant.insert(*appid, variant);
            }

            name_or_slug_to_variant.insert(entry.display_name.to_ascii_lowercase(), variant);
            name_or_slug_to_variant.insert(entry.slug.to_ascii_lowercase(), variant);

            let mut contains_patterns: Vec<String> = std::iter::once(&entry.display_name)
                .chain(std::iter::once(&entry.variant))
                .chain(std::iter::once(&entry.slug))
                .chain(entry.titles.iter())
                .chain(entry.processes.iter())
                .map(|s| normalize(s))
                .filter(|s| !s.is_empty())
                .collect();

            contains_patterns.sort_unstable();
            contains_patterns.dedup();

            let prefix_patterns = entry
                .prefixes
                .iter()
                .map(|p| normalize(p))
                .filter(|p| !p.is_empty())
                .collect();

            records.push(GameRecord {
                variant,
                slug: entry.slug,
                display_name: entry.display_name,
                steam_appids: entry.steam_appids,
                contains_patterns,
                prefix_patterns,
            });
        }

        let mut fuzzy_order: Vec<usize> = (0..records.len()).collect();
        fuzzy_order.sort_by_key(|&idx| {
            std::cmp::Reverse(
                records[idx]
                    .contains_patterns
                    .iter()
                    .map(|p| p.len())
                    .max()
                    .unwrap_or(0),
            )
        });

        Self {
            records,
            variant_to_index,
            appid_to_variant,
            name_or_slug_to_variant,
            fuzzy_order,
        }
    }

    fn record_for(&self, game: ClipsGames) -> Option<&GameRecord> {
        self.variant_to_index.get(&game).map(|&i| &self.records[i])
    }

    fn fuzzy_match(&self, input: &str) -> ClipsGames {
        let normalized = normalize(input);
        if normalized.is_empty() {
            return ClipsGames::Unknown;
        }

        for &idx in &self.fuzzy_order {
            let record = &self.records[idx];
            if record
                .prefix_patterns
                .iter()
                .any(|p| normalized.starts_with(p))
            {
                return record.variant;
            }
        }

        if GENERIC_IGNORE.contains(&normalized.as_str()) {
            return ClipsGames::Unknown;
        }

        for &idx in &self.fuzzy_order {
            let record = &self.records[idx];
            if record
                .contains_patterns
                .iter()
                .any(|p| normalized.contains(p))
            {
                return record.variant;
            }
        }

        ClipsGames::Unknown
    }
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
