//! Theme trait + shared styling helpers (label collision, field sampling).

use crate::display::{DisplayList, FontKind, Rgba};
use crate::raster::Raster;
use crate::scene::Scene;
use crate::text;

pub trait Theme {
    fn background(&self) -> Rgba;
    /// Base terrain: sea, land fills, coast, contours (pure vector).
    fn terrain(&self, scene: &Scene) -> DisplayList;
    /// Per-pixel pass between terrain and overlay (hillshade, paper grain).
    fn post_raster(&self, _scene: &Scene, _raster: &mut Raster) {}
    /// Everything above terrain: edges, cities, labels, decorations.
    fn overlay(&self, scene: &Scene) -> DisplayList;
}

/// Greedy label placement with collision rejection. Coordinates in px.
pub struct LabelPlacer {
    placed: Vec<(f32, f32, f32, f32)>, // x, y, w, h (top-left)
    width: f32,
    height: f32,
}

impl LabelPlacer {
    pub fn new(width: f32, height: f32) -> LabelPlacer {
        LabelPlacer {
            placed: Vec::new(),
            width,
            height,
        }
    }

    /// Try to claim a centered box at (cx, cy). Returns false on collision
    /// or out-of-canvas.
    pub fn try_place(&mut self, cx: f32, cy: f32, w: f32, h: f32) -> bool {
        let (x, y) = (cx - w / 2.0, cy - h / 2.0);
        if x < 1.0 || y < 1.0 || x + w > self.width - 1.0 || y + h > self.height - 1.0 {
            return false;
        }
        let pad = 3.0;
        for &(px, py, pw, ph) in &self.placed {
            if x < px + pw + pad && px < x + w + pad && y < py + ph + pad && py < y + h + pad {
                return false;
            }
        }
        self.placed.push((x, y, w, h));
        true
    }
}

/// Measure a label box for placement (single line).
pub fn label_box(s: &str, size_px: f32, font: FontKind) -> (f32, f32) {
    (text::measure(s, size_px, font), size_px * 1.15)
}

/// Bilinear sample of the scene's elevation field at normalized (x, y).
pub fn sample_elevation(scene: &Scene, x: f32, y: f32) -> f32 {
    let (w, h) = (scene.field_w, scene.field_h);
    if w == 0 || h == 0 {
        return 0.0;
    }
    let fx = (x * w as f32 - 0.5).clamp(0.0, (w - 1) as f32);
    let fy = (y * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
    let (x0, y0) = (fx as usize, fy as usize);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
    let v = |xx: usize, yy: usize| scene.elevation[yy * w + xx];
    let a = v(x0, y0) + (v(x1, y0) - v(x0, y0)) * tx;
    let b = v(x0, y1) + (v(x1, y1) - v(x0, y1)) * tx;
    a + (b - a) * ty
}

/// Language tag suffix shown in region labels ("src/auth · rs").
pub fn lang_suffix(lang: c2m_core::Lang) -> String {
    format!(" · {}", lang.tag())
}
