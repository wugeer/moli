use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;

const KEY_CONFIG_ENV: &str = "MOLI_KEY_CONFIG";
const CONFIG_FILE_NAME: &str = "key_bindings.ron";
const CONFIG_DIR_NAME: &str = "moli";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    PrevMonth,
    NextMonth,
    PrevYear,
    NextYear,
    BackToToday,
    OpenJumpPrompt,
}

#[derive(Clone, Debug)]
pub struct KeyBindings {
    bindings: Vec<(Binding, Action)>,
    labels: HashMap<Action, Vec<Binding>>,
}

impl KeyBindings {
    // return the label of the binding for the action
    pub fn labels_for(&self, action: Action) -> Vec<String> {
        self.labels
            .get(&action)
            .map(|bindings| bindings.iter().map(|b| b.label()).collect())
            .unwrap_or_default()
    }

    fn from_config(config: KeyBindingConfig) -> Self {
        let mut bindings = Vec::new();
        let mut labels: HashMap<Action, Vec<Binding>> = HashMap::new();
        bind_action(
            &mut bindings,
            &mut labels,
            Action::Quit,
            config.quit,
            &["Esc", "q", "Q"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::MoveLeft,
            config.move_left,
            &["h", "H"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::MoveRight,
            config.move_right,
            &["l", "L"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::MoveUp,
            config.move_up,
            &["k", "K"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::MoveDown,
            config.move_down,
            &["j", "J"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::PrevMonth,
            config.prev_month,
            &["Left"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::NextMonth,
            config.next_month,
            &["Right"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::PrevYear,
            config.prev_year,
            &["Up"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::NextYear,
            config.next_year,
            &["Down"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::BackToToday,
            config.back_to_today,
            &["t", "T"],
        );
        bind_action(
            &mut bindings,
            &mut labels,
            Action::OpenJumpPrompt,
            config.open_jump_prompt,
            &["g+d"],
        );
        KeyBindings { bindings, labels }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings::from_config(KeyBindingConfig::default())
    }
}

#[derive(Default)]
pub struct BindingResolver {
    pending: Vec<(usize, usize)>,
}

impl BindingResolver {
    pub fn process(&mut self, bindings: &KeyBindings, event: KeyEvent) -> Option<Action> {
        let mut new_pending = Vec::new();
        let current = std::mem::take(&mut self.pending);
        for (idx, progress) in current {
            let (binding, action) = &bindings.bindings[idx];
            if binding.matches_at(progress, event) {
                let next = progress + 1;
                if next == binding.len() {
                    return Some(*action);
                }
                new_pending.push((idx, next));
            }
        }

        for (idx, (binding, action)) in bindings.bindings.iter().enumerate() {
            if binding.matches_at(0, event) {
                if binding.len() == 1 {
                    return Some(*action);
                }
                new_pending.push((idx, 1));
            }
        }

        self.pending = new_pending;
        None
    }
}

#[derive(Clone, Debug)]
struct Binding {
    sequence: Vec<KeyPress>,
}

impl Binding {
    fn len(&self) -> usize {
        self.sequence.len()
    }

    fn matches_at(&self, index: usize, event: KeyEvent) -> bool {
        self.sequence
            .get(index)
            .map(|press| press.matches(event))
            .unwrap_or(false)
    }

    fn label(&self) -> String {
        self.sequence
            .iter()
            .map(|press| press.label())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Clone, Copy, Debug)]
struct KeyPress {
    code: KeyCode,
    modifiers: KeyModifiers,
}

impl KeyPress {
    fn matches(&self, event: KeyEvent) -> bool {
        if self.code != event.code {
            return false;
        }
        let mut event_modifiers = normalize_modifiers(event.modifiers);
        if matches!(self.code, KeyCode::Char(_)) && !self.modifiers.contains(KeyModifiers::SHIFT) {
            event_modifiers.remove(KeyModifiers::SHIFT);
        }
        self.modifiers == event_modifiers
    }

    fn label(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl".into());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt".into());
        }
        if self.modifiers.contains(KeyModifiers::SUPER) {
            parts.push("Meta".into());
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift".into());
        }
        parts.push(get_key_code_label(self.code));
        parts.join("+")
    }
}

pub fn load_key_bindings() -> KeyBindings {
    let path = env::var_os(KEY_CONFIG_ENV)
        .map(PathBuf::from)
        .or_else(default_config_path);
    if let Some(path) = path
        && let Some(bindings) = load_from_path(&path)
    {
        return bindings;
    }
    KeyBindings::default()
}

/// Load key bindings from the specified path
fn load_from_path(path: &Path) -> Option<KeyBindings> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("moli: failed to read key config {path:?}: {err}");
            return None;
        }
    };
    parse_config(&content).map(|config| KeyBindings::from_config(config.into_inner()))
}

/// Parse the RON configuration content
fn parse_config(content: &str) -> Option<ConfigFile> {
    match ron::from_str(content) {
        Ok(parsed) => Some(parsed),
        Err(err) => {
            eprintln!("moli: failed to parse key config: {err}");
            None
        }
    }
}

fn default_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut dir| {
        dir.push(CONFIG_DIR_NAME);
        dir.push(CONFIG_FILE_NAME);
        dir
    })
}

/// Keep Shift/Ctrl/Alt/Super modifiers and drop the rest
fn normalize_modifiers(modifiers: KeyModifiers) -> KeyModifiers {
    modifiers
        & (KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConfigFile {
    Direct(KeyBindingConfig),
    Wrapped { bindings: KeyBindingConfig },
}

impl ConfigFile {
    fn into_inner(self) -> KeyBindingConfig {
        match self {
            ConfigFile::Direct(inner) => inner,
            ConfigFile::Wrapped { bindings } => bindings,
        }
    }
}

/// Key binding configuration loaded from file
#[derive(Debug, Default, Deserialize)]
struct KeyBindingConfig {
    quit: Option<Vec<String>>,
    move_left: Option<Vec<String>>,
    move_right: Option<Vec<String>>,
    move_up: Option<Vec<String>>,
    move_down: Option<Vec<String>>,
    prev_month: Option<Vec<String>>,
    next_month: Option<Vec<String>>,
    prev_year: Option<Vec<String>>,
    next_year: Option<Vec<String>>,
    back_to_today: Option<Vec<String>>,
    open_jump_prompt: Option<Vec<String>>,
}

/// Bind an action to the provided key entries
fn bind_action(
    bindings: &mut Vec<(Binding, Action)>,
    labels: &mut HashMap<Action, Vec<Binding>>,
    action: Action,
    entries: Option<Vec<String>>,
    fallback: &[&str],
) {
    let tokens = entries.unwrap_or_else(|| fallback.iter().map(|s| s.to_string()).collect());
    let mut is_add = false;
    for token in tokens {
        match parse_binding(&token) {
            Some(binding) => {
                labels.entry(action).or_default().push(binding.clone());
                bindings.push((binding, action));
                is_add = true;
            }
            None => eprintln!("moli: unknown key binding token '{token}'"),
        }
    }
    if !is_add {
        eprintln!(
            "moli: no key bindings configured for {:?}; action disabled",
            action
        );
    }
}

/// Parse a key binding sequence from a string
fn parse_binding(raw: &str) -> Option<Binding> {
    let mut sequence = Vec::new();
    let mut modifiers = KeyModifiers::empty();
    for part in raw.split('+') {
        let token = part.trim();
        if token.is_empty() {
            continue;
        }
        if let Some(modifier) = parse_modifier(token) {
            modifiers |= modifier;
            continue;
        }
        let code = parse_key_code(token)?;
        sequence.push(KeyPress {
            code,
            modifiers: normalize_modifiers(modifiers),
        });
        modifiers = KeyModifiers::empty();
    }
    if !modifiers.is_empty() {
        eprintln!("moli: dangling modifiers in '{raw}'");
    }
    if sequence.is_empty() {
        None
    } else {
        Some(Binding { sequence })
    }
}

/// Parse modifier keys such as Ctrl/Shift
fn parse_modifier(token: &str) -> Option<KeyModifiers> {
    match token.to_ascii_lowercase().as_str() {
        "ctrl" | "control" => Some(KeyModifiers::CONTROL),
        "alt" | "option" => Some(KeyModifiers::ALT),
        "shift" => Some(KeyModifiers::SHIFT),
        "meta" | "super" | "cmd" | "command" => Some(KeyModifiers::SUPER),
        _ => None,
    }
}

/// Parse an individual key code
fn parse_key_code(raw: &str) -> Option<KeyCode> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut chars = trimmed.chars();
    if let (Some(ch), None) = (chars.next(), chars.next()) {
        return Some(KeyCode::Char(ch));
    }
    let lowered = trimmed.to_ascii_lowercase();
    match lowered.as_str() {
        "esc" | "escape" => Some(KeyCode::Esc),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "space" => Some(KeyCode::Char(' ')),
        "enter" | "return" => Some(KeyCode::Enter),
        "backspace" => Some(KeyCode::Backspace),
        "tab" => Some(KeyCode::Tab),
        "delete" => Some(KeyCode::Delete),
        "insert" => Some(KeyCode::Insert),
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        "pageup" | "page_up" => Some(KeyCode::PageUp),
        "pagedown" | "page_down" => Some(KeyCode::PageDown),
        _ => parse_function_key(&lowered),
    }
}

/// Parse function keys F{1-12}
fn parse_function_key(token: &str) -> Option<KeyCode> {
    if let Some(rest) = token.strip_prefix('f')
        && let Ok(num) = rest.parse::<u8>()
    {
        return Some(KeyCode::F(num));
    }
    None
}

/// Map key codes to human-readable labels
fn get_key_code_label(code: KeyCode) -> String {
    match code {
        KeyCode::Char(' ') => "Space".to_string(),
        KeyCode::Char(ch) => ch.to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::F(n) => format!("F{n}"),
        other => format!("{other:?}"),
    }
}
