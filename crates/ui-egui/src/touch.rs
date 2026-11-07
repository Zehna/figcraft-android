//! Touch and mobile adaptations: safe area insets, touch scaling 1.25x,
//! and virtual modifier puck for touch interactions.

use egui::{Color32, CornerRadius, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

/// Screen insets (in points) for safe areas: top (notch / status bar),
/// right, bottom (navigation / gesture pill bar), and left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for SafeAreaInsets {
    fn default() -> Self {
        #[cfg(target_os = "android")]
        {
            Self { top: 24.0, right: 0.0, bottom: 24.0, left: 0.0 }
        }
        #[cfg(not(target_os = "android"))]
        {
            Self { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
        }
    }
}

impl SafeAreaInsets {
    pub const ZERO: Self = Self { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 };

    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: if top.is_finite() && top >= 0.0 { top } else { 0.0 },
            right: if right.is_finite() && right >= 0.0 { right } else { 0.0 },
            bottom: if bottom.is_finite() && bottom >= 0.0 { bottom } else { 0.0 },
            left: if left.is_finite() && left >= 0.0 { left } else { 0.0 },
        }
    }
}

/// Floating virtual modifier puck on the canvas for touch screens.
/// Provides one-handed or latched access to Shift, Alt, and Ctrl/Cmd modifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct ModifierPuck {
    /// Whether the puck is shown. Default true on Android, false on desktop.
    pub enabled: bool,
    /// Custom position on canvas, or None to use default (bottom-left).
    pub pos: Option<Pos2>,
    /// Momentary hold states (pressed during current frame).
    pub hold_shift: bool,
    pub hold_alt: bool,
    pub hold_ctrl: bool,
    /// Locked/latched states (toggled on/off).
    pub lock_shift: bool,
    pub lock_alt: bool,
    pub lock_ctrl: bool,
}

impl Default for ModifierPuck {
    fn default() -> Self {
        #[cfg(target_os = "android")]
        let enabled = true;
        #[cfg(not(target_os = "android"))]
        let enabled = false;

        Self {
            enabled,
            pos: None,
            hold_shift: false,
            hold_alt: false,
            hold_ctrl: false,
            lock_shift: false,
            lock_alt: false,
            lock_ctrl: false,
        }
    }
}

impl ModifierPuck {
    pub fn is_shift(&self) -> bool {
        self.enabled && (self.hold_shift || self.lock_shift)
    }

    pub fn is_alt(&self) -> bool {
        self.enabled && (self.hold_alt || self.lock_alt)
    }

    pub fn is_ctrl(&self) -> bool {
        self.enabled && (self.hold_ctrl || self.lock_ctrl)
    }

    /// Reset any momentary hold states at the start of a frame.
    pub fn begin_frame(&mut self) {
        self.hold_shift = false;
        self.hold_alt = false;
        self.hold_ctrl = false;
    }

    /// Render and handle interactions for the modifier puck over the canvas.
    pub fn show(&mut self, ui: &mut Ui, canvas: Rect, safe_area: SafeAreaInsets, tokens: &crate::theme::Tokens) {
        if !self.enabled {
            return;
        }

        // Puck dimensions: a comfortable touch-sized capsule or pill.
        let puck_w = 148.0;
        let puck_h = 44.0;

        let default_pos = pos2(
            canvas.min.x + 16.0 + safe_area.left,
            (canvas.max.y - puck_h - 16.0 - safe_area.bottom).max(canvas.min.y),
        );

        let mut center = self.pos.unwrap_or(default_pos);
        // Clamp to stay within canvas boundaries.
        let min_x = canvas.min.x + safe_area.left + 4.0;
        let max_x = (canvas.max.x - puck_w - safe_area.right - 4.0).max(min_x);
        let min_y = canvas.min.y + safe_area.top + 4.0;
        let max_y = (canvas.max.y - puck_h - safe_area.bottom - 4.0).max(min_y);
        center.x = center.x.clamp(min_x, max_x);
        center.y = center.y.clamp(min_y, max_y);
        self.pos = Some(center);

        let rect = Rect::from_min_size(center, vec2(puck_w, puck_h));

        // Drag handle area on the left edge of the puck
        let handle_w = 20.0;
        let handle_rect = Rect::from_min_size(rect.min, vec2(handle_w, puck_h));
        let handle_resp = ui.interact(handle_rect, ui.scope_id().with("puck_handle"), Sense::click_and_drag());
        if handle_resp.dragged() {
            let delta = handle_resp.drag_delta();
            center.x += delta.x;
            center.y += delta.y;
            self.pos = Some(center);
        }

        let painter = ui.painter();
        // Background card
        painter.rect_filled(rect, CornerRadius::same(12), Color32::from_black_alpha(180));
        painter.rect_stroke(rect, CornerRadius::same(12), Stroke::new(1.0, tokens.border), StrokeKind::Inside);

        // Draw drag grip dots
        let dot_color = Color32::from_white_alpha(120);
        let gx = rect.min.x + 8.0;
        let cy = rect.center().y;
        for dy in [-6.0, 0.0, 6.0] {
            painter.circle_filled(pos2(gx, cy + dy), 1.5, dot_color);
        }

        // Modifier buttons: Shift, Alt, Cmd
        let btn_w = 38.0;
        let btn_h = 32.0;
        let start_x = rect.min.x + handle_w + 2.0;
        let btn_y = rect.min.y + (puck_h - btn_h) * 0.5;

        let mods = [
            ("Shift", self.is_shift(), self.lock_shift, 0),
            ("Alt", self.is_alt(), self.lock_alt, 1),
            ("Cmd", self.is_ctrl(), self.lock_ctrl, 2),
        ];

        for (label, active, locked, idx) in mods {
            let bx = start_x + (btn_w + 3.0) * idx as f32;
            let b_rect = Rect::from_min_size(pos2(bx, btn_y), vec2(btn_w, btn_h));
            let b_resp = ui.interact(b_rect, ui.scope_id().with(("puck_btn", idx)), Sense::click_and_drag());

            let is_down = b_resp.is_pointer_button_down_on();
            let clicked = b_resp.clicked();
            let double_clicked = b_resp.double_clicked();

            // Toggle lock on click / double click
            if double_clicked || clicked {
                match idx {
                    0 => self.lock_shift = !self.lock_shift,
                    1 => self.lock_alt = !self.lock_alt,
                    2 => self.lock_ctrl = !self.lock_ctrl,
                    _ => {}
                }
            } else if is_down {
                match idx {
                    0 => self.hold_shift = true,
                    1 => self.hold_alt = true,
                    2 => self.hold_ctrl = true,
                    _ => {}
                }
            }

            // Button appearance
            let bg = if locked {
                Color32::from_rgb(46, 125, 246) // Blue accent when locked
            } else if active || is_down {
                Color32::from_white_alpha(70)
            } else {
                Color32::from_white_alpha(25)
            };

            painter.rect_filled(b_rect, CornerRadius::same(6), bg);
            let text_color = if locked || active || is_down { Color32::WHITE } else { Color32::from_gray(210) };
            painter.text(
                b_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(11.0),
                text_color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_area_insets_defaults_and_validation() {
        let zero = SafeAreaInsets::ZERO;
        assert_eq!(zero.top, 0.0);
        assert_eq!(zero.right, 0.0);
        assert_eq!(zero.bottom, 0.0);
        assert_eq!(zero.left, 0.0);

        let custom = SafeAreaInsets::new(28.0, 10.0, 34.0, 10.0);
        assert_eq!(custom.top, 28.0);
        assert_eq!(custom.right, 10.0);
        assert_eq!(custom.bottom, 34.0);
        assert_eq!(custom.left, 10.0);

        // Negative or non-finite inputs clamped to 0.0
        let invalid = SafeAreaInsets::new(-5.0, f32::NAN, f32::INFINITY, -10.0);
        assert_eq!(invalid.top, 0.0);
        assert_eq!(invalid.right, 0.0);
        assert_eq!(invalid.bottom, 0.0);
        assert_eq!(invalid.left, 0.0);
    }

    #[test]
    fn modifier_puck_state_and_locks() {
        let mut puck = ModifierPuck::default();
        puck.enabled = true;

        assert!(!puck.is_shift());
        assert!(!puck.is_alt());
        assert!(!puck.is_ctrl());

        // Momentary hold
        puck.hold_alt = true;
        assert!(puck.is_alt());
        puck.begin_frame();
        assert!(!puck.is_alt());

        // Lock toggle
        puck.lock_shift = true;
        assert!(puck.is_shift());
        puck.begin_frame();
        // Stays true after frame reset because it's locked
        assert!(puck.is_shift());

        puck.lock_shift = false;
        assert!(!puck.is_shift());
    }
}
