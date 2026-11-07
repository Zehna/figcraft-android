//! Unit tests for touch and mobile interface adaptations.

use egui::{Modifiers, Pos2, Rect, Vec2, pos2, vec2};
use figcraft_engine::Session;

use crate::touch::{ModifierPuck, SafeAreaInsets};
use crate::ui_scale::{plan, plan_touch};
use crate::{FigApp, services};

#[test]
fn safe_area_insets_on_fig_app() {
    let session = Session::new();
    let services = services::Mock::default();
    let mut app = FigApp::new(session, Box::new(services));

    assert_eq!(app.ui.safe_area, SafeAreaInsets::default());

    app.set_safe_area_insets(32.0, 12.0, 28.0, 12.0);
    assert_eq!(app.ui.safe_area.top, 32.0);
    assert_eq!(app.ui.safe_area.right, 12.0);
    assert_eq!(app.ui.safe_area.bottom, 28.0);
    assert_eq!(app.ui.safe_area.left, 12.0);
}

#[test]
fn touch_scale_125_on_mobile_bounds() {
    let phone_px = Some((1080.0, 2400.0));
    let native = Some(2.75);

    // Desktop plan is clamped down to 1.0 because window points (392) < 800
    let desk = plan(1.25, native, phone_px);
    assert_eq!(desk.zoom, 1.0);
    assert!(desk.limited);

    // Touch plan uses mobile constraint (320) so 1.25x zoom is achieved without limiting
    let touch = plan_touch(1.0, native, phone_px, true);
    assert_eq!(touch.zoom, 1.25);
    assert!(!touch.limited);
}

#[test]
fn modifier_puck_hold_and_lock_states() {
    let mut puck = ModifierPuck::default();
    puck.enabled = true;

    assert!(!puck.is_shift());
    assert!(!puck.is_alt());
    assert!(!puck.is_ctrl());

    // Holding Shift momentarily
    puck.hold_shift = true;
    assert!(puck.is_shift());
    puck.begin_frame();
    assert!(!puck.is_shift());

    // Locking Alt
    puck.lock_alt = true;
    assert!(puck.is_alt());
    puck.begin_frame();
    assert!(puck.is_alt()); // Stays true across frames

    // Locking Ctrl
    puck.lock_ctrl = true;
    assert!(puck.is_ctrl());
    puck.lock_ctrl = false;
    assert!(!puck.is_ctrl());
}

#[test]
fn toolbar_pill_respects_safe_area_bottom() {
    let canvas = Rect::from_min_max(pos2(0.0, 0.0), pos2(400.0, 800.0));
    let width = 240.0;

    let p0 = crate::toolbar::pill_rect(canvas, width);
    let p_safe = crate::toolbar::pill_rect_with_inset(canvas, width, 34.0);

    // Safe area inset shifts toolbar pill up by 34 points
    assert!((p0.center().y - p_safe.center().y - 34.0).abs() < 1e-4);
}
