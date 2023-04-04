#![allow(clippy::unusual_byte_groupings)]

use miniquad::*;
use skia_safe::canvas::{SaveLayerFlags, SaveLayerRec};
use skia_safe::{image_filters, scalar, Canvas, Color, Paint, RRect, Rect};

struct Stage {
    x: f32,
    y: f32,
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let Some(skia) = ctx.skia.as_mut() else {
            return;
        };

        skia.recreate_surface(width as i32, height as i32);
    }

    fn draw(&mut self, ctx: &mut Context) {
        let Some(skia) = ctx.skia.as_mut() else {
            return;
        };

        let canvas = &mut skia.surface.canvas();
        canvas.clear(Color::from(0xff_161a1d));

        // simple rectangle
        simple_rectangle(canvas, 0., 0., 200., 200., 0., 10.);

        // blur rectangle
        {
            let rect = rect_centered(self.x, self.y, 150., 400.);
            let rrect = RRect::new_rect_xy(rect, 20., 20.);

            canvas.save();
            {
                canvas.clip_rrect(rrect, skia_safe::ClipOp::Intersect, true);
                let image_filter = image_filters::blur((10., 10.), None, None, None);

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_dither(true);
                paint.set_image_filter(image_filter);

                let ext = 20.;
                let layer_rect = Rect::from_xywh(
                    rect.x() - ext,
                    rect.y() - ext,
                    rect.width() + ext * 2.,
                    rect.height() + ext * 2.,
                );
                let layer_rec = SaveLayerRec::default()
                    .bounds(&layer_rect)
                    .paint(&paint)
                    .flags(SaveLayerFlags::INIT_WITH_PREVIOUS);

                canvas.save_layer(&layer_rec);
                {
                    canvas.draw_color(0x80_ffffff, None);
                }
                canvas.restore();
            }
            canvas.restore();
        }

        skia.dctx.flush(None);
    }
}

fn rect_centered(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_xywh(x - width / 2., y - height / 2., width, height)
}

fn rect_from_center(canvas: &mut Canvas, x: f32, y: f32, width: f32, height: f32) -> Rect {
    let dim = canvas.image_info().dimensions();
    let (cx, cy) = (dim.width as f32 / 2., dim.height as f32 / 2.);
    rect_centered(cx + x, cy + y, width, height)
}

pub fn simple_rectangle(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    roundness: scalar,
    stroke_width: scalar,
) -> RRect {
    let rect = rect_from_center(canvas, x, y, width, height);
    let rrect = RRect::new_rect_xy(rect, roundness, roundness);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    paint.set_stroke(false);
    paint.set_color(Color::from(0x80_bbddff));
    canvas.draw_rrect(rrect, &paint);

    if stroke_width > 0. {
        paint.set_stroke(true);
        paint.set_stroke_width(stroke_width);
        paint.set_color(Color::from(0xff_bbddff));
        canvas.draw_rrect(rrect, &paint);
    }

    rrect
}

fn main() {
    miniquad::start(conf::Conf::default(), |_ctx| {
        Box::new(Stage { x: 0.0, y: 0.0 })
    });
}
