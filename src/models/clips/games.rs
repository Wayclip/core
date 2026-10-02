use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf, sync::OnceLock};

const BUNDLED_GAMES_JSON: &str = include_str!("../../../assets/games.json");

static REGISTRY: OnceLock<GameRegistry> = OnceLock::new();

/// Represents a game title, its identifiers, and genre metadata
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Game {
    /// Stable, lowercase identifier (e.g. `"the_finals"`)
    pub slug: String,
    /// Human-readable display name (e.g. `"THE FINALS"`)
    pub name: String,
    /// Steam App ID if the title is tracked via Steam
    pub steam_appid: Option<u32>,
    /// Icon URL of game
    pub icon_url: Option<String>,
    /// Cover Art URL of game
    pub cover_url: Option<String>,
    /// Categories and genres used for similarity and feed ranking
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Game {
    /// Creates a new game instance generating a slug from the given name
    pub fn new(name: impl Into<String>, steam_appid: Option<u32>) -> Self {
        let name = name.into();
        let slug = Self::slugify(&name);
        Self {
            slug,
            name,
            steam_appid,
            icon_url: None,
            cover_url: None,
            tags: Vec::new(),
        }
    }

    /// Converts a display name into a clean snake_case slug
    fn slugify(name: &str) -> String {
        let mut out = String::with_capacity(name.len());
        let mut last_dash = true;

        for c in name.chars() {
            if c.is_ascii_alphanumeric() {
                out.push(c.to_ascii_lowercase());
                last_dash = false;
            } else if !last_dash {
                out.push('_');
                last_dash = true;
            }
        }
        if out.ends_with('_') {
            out.pop();
        }
        out
    }

    /// Calculates a similarity score from `0.0` to `1.0` based on shared tags
    pub fn calculate_similarity_score_with(&self, other_game: &Self) -> f32 {
        if self.tags.is_empty() || other_game.tags.is_empty() {
            return 0.0;
        }

        let num_of_common = self
            .tags
            .iter()
            .filter(|t| other_game.tags.contains(t))
            .count();
        let union = self.tags.len() + other_game.tags.len() - num_of_common;

        if union == 0 {
            return 0.0;
        }

        num_of_common as f32 / union as f32
    }

    /// Finds other games in the registry with overlapping tags, ordered by similarity
    pub fn find_similar(&self, limit: usize) -> Vec<(Self, f32)> {
        let mut scores: Vec<(Self, f32)> = GameRegistry::global()
            .all()
            .iter()
            .filter(|game| game.slug != self.slug)
            .map(|game| (game.clone(), self.calculate_similarity_score_with(game)))
            .filter(|&(_, score)| score > 0.0)
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        scores.truncate(limit);
        scores
    }
}

/// Raw game entry parsed from the bundled JSON dataset
#[derive(Deserialize)]
struct RawGameEntry {
    slug: String,
    name: String,
    #[serde(default)]
    steam_appid: Option<u32>,
    #[serde(default)]
    icon_url: Option<String>,
    #[serde(default)]
    cover_url: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    exes: Vec<String>,
}

/// Custom games configured by the user in `~/.config/wayclip/games.toml`
#[derive(Default, Deserialize)]
struct UserGamesConfig {
    #[serde(default)]
    games: Vec<RawGameEntry>,
}

/// fast lookups by executable name or slug
pub struct GameRegistry {
    by_slug: HashMap<String, Game>,
    by_exe: HashMap<String, Game>,
    all: Vec<Game>,
}

impl GameRegistry {
    /// Returns the global registry instance,
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(Self::load)
    }

    /// Loads bundled games and merges any local user overrides
    fn load() -> Self {
        let mut by_slug = HashMap::new();
        let mut by_exe = HashMap::new();
        let mut all = Vec::new();

        if let Ok(entries) = serde_json::from_str::<Vec<RawGameEntry>>(BUNDLED_GAMES_JSON) {
            for entry in entries {
                let game = Game {
                    slug: entry.slug.clone(),
                    name: entry.name,
                    steam_appid: entry.steam_appid,
                    icon_url: entry.icon_url,
                    cover_url: entry.cover_url,
                    tags: entry.tags,
                };
                for exe in entry.exes {
                    by_exe.insert(normalize_exe(&exe), game.clone());
                }
                by_slug.insert(entry.slug, game.clone());
                all.push(game);
            }
        }

        if let Some(home) = std::env::var_os("HOME") {
            let config_path = PathBuf::from(home).join(".config/wayclip/games.toml");
            if let Ok(raw) = fs::read_to_string(config_path) {
                if let Ok(cfg) = toml::from_str::<UserGamesConfig>(&raw) {
                    for entry in cfg.games {
                        let game = Game {
                            slug: entry.slug.clone(),
                            name: entry.name,
                            steam_appid: entry.steam_appid,
                            icon_url: entry.icon_url,
                            cover_url: entry.cover_url,
                            tags: entry.tags,
                        };
                        for exe in entry.exes {
                            by_exe.insert(normalize_exe(&exe), game.clone());
                        }
                        by_slug.insert(entry.slug, game.clone());
                        all.push(game);
                    }
                }
            }
        }

        all.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        Self {
            by_slug,
            by_exe,
            all,
        }
    }

    /// Finds a game by executable name (case-insensitive & `.exe` || `-bin` stripped)
    pub fn by_exe(&self, exe: &str) -> Option<Game> {
        self.by_exe.get(&normalize_exe(exe)).cloned()
    }

    /// Finds a game by its stable slug
    pub fn by_slug(&self, slug: &str) -> Option<Game> {
        self.by_slug.get(slug).cloned()
    }

    /// Returns a slice of all registered games sorted alphabetically
    pub fn all(&self) -> &[Game] {
        &self.all
    }

    /// Searches games matching a substring in their display name or slug
    pub fn search(&self, query: &str, limit: usize) -> Vec<Game> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return self.all.iter().take(limit).cloned().collect();
        }

        self.all
            .iter()
            .filter(|g| g.name.to_lowercase().contains(&q) || g.slug.contains(&q))
            .take(limit)
            .cloned()
            .collect()
    }
}

/// Normalizes executable basenames
fn normalize_exe(raw: &str) -> String {
    let mut s = raw.trim().to_ascii_lowercase();
    if let Some(stripped) = s.strip_suffix(".exe") {
        s = stripped.to_string();
    }
    if let Some(stripped) = s.strip_suffix("-bin") {
        s = stripped.to_string();
    }
    s
}
