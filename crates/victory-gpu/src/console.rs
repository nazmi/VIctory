//! Which console the title runs on. Titles pick settings (resolution,
//! memory budgets) by it, so the user chooses; Series X unless told.

use std::sync::atomic::{AtomicBool, Ordering};

const GIB: usize = 1 << 30;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Console {
    #[default]
    SeriesX,
    SeriesS,
}

impl Console {
    /// Title memory in bytes: (GPU-optimal, standard). Series X in the
    /// advanced configuration; Series S has one range, counted as standard
    /// (7,776 MiB, the published console-mode figure).
    // shortcut: later system updates gave Series S titles more (amount not
    // published here); Grand Theft Auto VI sizes nothing from this figure
    // (it goes by the device type), so update it if a title does.
    pub const fn title_memory(self) -> (usize, usize) {
        match self {
            Console::SeriesX => (10 * GIB, 7 * GIB / 2),
            Console::SeriesS => (0, 7_776 << 20),
        }
    }

    /// Compute units of the GPU.
    pub const fn compute_units(self) -> u32 {
        match self {
            Console::SeriesX => 52,
            Console::SeriesS => 20,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Console::SeriesX => "Xbox Series X",
            Console::SeriesS => "Xbox Series S",
        }
    }
}

static SERIES_S: AtomicBool = AtomicBool::new(false);

/// Set before the title starts.
pub fn set(console: Console) {
    SERIES_S.store(console == Console::SeriesS, Ordering::Relaxed);
}

pub fn current() -> Console {
    if SERIES_S.load(Ordering::Relaxed) {
        Console::SeriesS
    } else {
        Console::SeriesX
    }
}

/// How much smaller than the title asks to render its render targets and
/// depth targets (`VICTORY_RENDER_SCALE`, 0 < ratio <= 1; 1 by default).
///
/// The title picks its render size from the console and ignores what the
/// display and the window say, so on a host that cannot afford 4K the only
/// lever left is the allocations themselves. Every render target and depth
/// target is made `ratio` times as large, and the window, the viewports,
/// the scissor rectangles and the texture copy boxes are scaled to match, so
/// the frame stays coherent.
pub fn render_scale() -> f32 {
    static SCALE: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
    *SCALE.get_or_init(|| {
        let Ok(text) = std::env::var("VICTORY_RENDER_SCALE") else {
            return 1.0;
        };
        match text.trim().parse::<f32>() {
            Ok(v) if v > 0.0 && v <= 1.0 => v,
            _ => {
                tracing::warn!(
                    value = %text,
                    "ignoring invalid VICTORY_RENDER_SCALE; expected a ratio above 0 and at most 1"
                );
                1.0
            }
        }
    })
}

/// `value` scaled by the render scale, at least 1.
pub fn scaled(value: u64) -> u64 {
    let scale = render_scale();
    if scale >= 1.0 {
        return value;
    }
    ((value as f64 * f64::from(scale)).round() as u64).max(1)
}
