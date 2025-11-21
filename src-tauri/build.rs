// use image::GenericImageView;
fn rasterize_svg_to_png(svg_path: &str, out_png: &str) -> Result<(), Box<dyn std::error::Error>> {
  let svg = std::fs::read(svg_path)?;
  let opt = usvg::Options::default();
  let tree = usvg::Tree::from_data(&svg, &opt)?;
  let w = tree.size.width().ceil() as u32;
  let h = tree.size.height().ceil() as u32;
  let mut pixmap = tiny_skia::Pixmap::new(w, h).ok_or("pixmap create failed")?;
  resvg::render(&tree, usvg::FitTo::Original, resvg::tiny_skia::Transform::identity(), pixmap.as_mut()).ok_or("render failed")?;
  pixmap.save_png(out_png)?;
  Ok(())
}
fn main() {
  let _ = std::fs::create_dir_all("icons");
  let svg_src = "../public/votion.svg";
  let png_out = "icons/icon.png";
  // try rasterize svg to png; fallback to generated gradient when conversion fails
  if let Err(_e) = rasterize_svg_to_png(svg_src, png_out) {
    let w = 1024u32;
    let h = 1024u32;
    let img = image::ImageBuffer::from_fn(w, h, |x, _y| {
      let t = x as f32 / (w as f32 - 1.0);
      let r = (92.0 + (123.0 - 92.0) * t) as u8;
      let g = (225.0 + (97.0 - 225.0) * t) as u8;
      let b = (230.0 + (255.0 - 230.0) * t) as u8;
      image::Rgba([r, g, b, 255])
    });
    let _ = img.save(png_out);
  }
  tauri_build::build()
}