use anyhow::{Context, Result};
use oxide_db::cell::LayoutView;
use oxide_db::geometry::Rect;
use oxide_tech::tech::Technology;
use std::path::Path;

const PX_PER_LAMBDA: f64 = 20.0;
const MARGIN: f64 = 2.0;

pub fn export_svg(layout: &LayoutView, tech: &Technology, path: &Path) -> Result<()> {
    let bbox = padded_bbox(layout);
    let svg_str = render_svg(layout, tech, &bbox);
    std::fs::write(path, svg_str).context("writing SVG")?;
    Ok(())
}

pub fn export_png(layout: &LayoutView, tech: &Technology, path: &Path) -> Result<()> {
    let bbox = padded_bbox(layout);
    let img = render_rgba(layout, tech, &bbox);
    img.save(path).context("saving PNG")?;
    Ok(())
}

fn padded_bbox(layout: &LayoutView) -> Rect {
    match layout.bounding_rect() {
        Some(b) => b.expanded(MARGIN),
        None => Rect::new(-MARGIN, -MARGIN, MARGIN * 2.0, MARGIN * 2.0),
    }
}

// ── SVG renderer ──────────────────────────────────────────────────────────────

fn render_svg(layout: &LayoutView, tech: &Technology, bbox: &Rect) -> String {
    use svg::node::element::{Line as SvgLine, Rectangle as SvgRect, Text as SvgText};
    use svg::Document;

    let w = (bbox.width * PX_PER_LAMBDA).ceil() as u32;
    let h = (bbox.height * PX_PER_LAMBDA).ceil() as u32;
    let (w, h) = (w.max(1), h.max(1));

    // Lambda → SVG pixel (Y is flipped: lambda Y grows up, SVG Y grows down)
    let to_x = |lx: f64| -> f64 { (lx - bbox.x) * PX_PER_LAMBDA };
    let to_y = |ly: f64| -> f64 { (bbox.y + bbox.height - ly) * PX_PER_LAMBDA };

    let mut doc = Document::new()
        .set("viewBox", format!("0 0 {} {}", w, h))
        .set("width", format!("{}px", w))
        .set("height", format!("{}px", h));

    // Background
    doc = doc.add(
        SvgRect::new()
            .set("x", 0i32)
            .set("y", 0i32)
            .set("width", w)
            .set("height", h)
            .set("fill", "#1a1a1a"),
    );

    // Shapes sorted by z_order (bottom layers first)
    let mut shapes: Vec<&oxide_db::shape::Shape> = layout.shapes.iter().collect();
    shapes.sort_by_key(|s| tech.layer(&s.layer).map(|l| l.z_order).unwrap_or(0));

    for shape in &shapes {
        let r = shape.bounding_rect();
        let x = to_x(r.x);
        let y = to_y(r.y1()); // top-left corner in SVG space
        let sw = r.width * PX_PER_LAMBDA;
        let sh = r.height * PX_PER_LAMBDA;

        let (fr, fg, fb, fa) = tech
            .layer(&shape.layer)
            .map(|l| (l.color[0], l.color[1], l.color[2], l.color[3]))
            .unwrap_or((128, 128, 128, 200));

        doc = doc.add(
            SvgRect::new()
                .set("x", x)
                .set("y", y)
                .set("width", sw)
                .set("height", sh)
                .set("fill", format!("rgb({},{},{})", fr, fg, fb))
                .set("fill-opacity", format!("{:.3}", fa as f64 / 255.0)),
        );
    }

    // Scale bar: 10λ at bottom-left
    let bar_lambda = if bbox.width >= 10.0 { 10.0f64 } else { 1.0f64 };
    let bar_px = bar_lambda * PX_PER_LAMBDA;
    let bar_x = 8.0f64;
    let bar_y = h as f64 - 10.0;

    doc = doc.add(
        SvgLine::new()
            .set("x1", bar_x)
            .set("y1", bar_y)
            .set("x2", bar_x + bar_px)
            .set("y2", bar_y)
            .set("stroke", "#aaaaaa")
            .set("stroke-width", 2),
    );
    doc = doc.add(
        SvgLine::new()
            .set("x1", bar_x).set("y1", bar_y - 4.0)
            .set("x2", bar_x).set("y2", bar_y + 4.0)
            .set("stroke", "#aaaaaa").set("stroke-width", 1),
    );
    doc = doc.add(
        SvgLine::new()
            .set("x1", bar_x + bar_px).set("y1", bar_y - 4.0)
            .set("x2", bar_x + bar_px).set("y2", bar_y + 4.0)
            .set("stroke", "#aaaaaa").set("stroke-width", 1),
    );
    doc = doc.add(
        SvgText::new(format!("{}λ", bar_lambda as i32))
            .set("x", bar_x + bar_px / 2.0)
            .set("y", bar_y - 5.0)
            .set("text-anchor", "middle")
            .set("font-family", "monospace")
            .set("font-size", "10")
            .set("fill", "#aaaaaa"),
    );

    format!("{}", doc)
}

// ── PNG renderer ──────────────────────────────────────────────────────────────

fn render_rgba(layout: &LayoutView, tech: &Technology, bbox: &Rect) -> image::RgbaImage {
    let w = (bbox.width * PX_PER_LAMBDA).ceil() as u32;
    let h = (bbox.height * PX_PER_LAMBDA).ceil() as u32;
    let (w, h) = (w.max(1), h.max(1));

    let mut img = image::RgbaImage::from_pixel(w, h, image::Rgba([26u8, 26, 26, 255]));

    let mut shapes: Vec<&oxide_db::shape::Shape> = layout.shapes.iter().collect();
    shapes.sort_by_key(|s| tech.layer(&s.layer).map(|l| l.z_order).unwrap_or(0));

    for shape in &shapes {
        let r = shape.bounding_rect();
        let color = tech
            .layer(&shape.layer)
            .map(|l| image::Rgba([l.color[0], l.color[1], l.color[2], l.color[3]]))
            .unwrap_or(image::Rgba([128u8, 128, 128, 200]));

        // Lambda → pixel coords (Y flipped)
        let x0 = ((r.x - bbox.x) * PX_PER_LAMBDA).floor() as u32;
        let y0 = ((bbox.y + bbox.height - r.y1()) * PX_PER_LAMBDA).floor() as u32;
        let x1 = ((r.x1() - bbox.x) * PX_PER_LAMBDA).ceil() as u32;
        let y1 = ((bbox.y + bbox.height - r.y) * PX_PER_LAMBDA).ceil() as u32;
        let x0 = x0.min(w);
        let x1 = x1.min(w);
        let y0 = y0.min(h);
        let y1 = y1.min(h);

        for py in y0..y1 {
            for px in x0..x1 {
                let dst = *img.get_pixel(px, py);
                img.put_pixel(px, py, alpha_blend(dst, color));
            }
        }
    }

    img
}

fn alpha_blend(dst: image::Rgba<u8>, src: image::Rgba<u8>) -> image::Rgba<u8> {
    let sa = src[3] as f32 / 255.0;
    let da = dst[3] as f32 / 255.0;
    let oa = sa + da * (1.0 - sa);
    if oa < 0.001 {
        return image::Rgba([0, 0, 0, 0]);
    }
    let mix = |sc: u8, dc: u8| -> u8 {
        ((sc as f32 * sa + dc as f32 * da * (1.0 - sa)) / oa) as u8
    };
    image::Rgba([mix(src[0], dst[0]), mix(src[1], dst[1]), mix(src[2], dst[2]), (oa * 255.0) as u8])
}
