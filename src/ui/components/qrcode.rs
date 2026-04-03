pub fn build_qrcode_svg(target_url: &str) -> Result<String, String> {
    let qrcode = qrcode::QrCode::new(target_url)
        .map_err(|err| format!("failed to build qrcode payload: {err}"))?;

    let svg = qrcode
        .render::<qrcode::render::svg::Color>()
        .dark_color(qrcode::render::svg::Color("#000000"))
        .light_color(qrcode::render::svg::Color("#ffffff"))
        .build();

    let start = svg
        .find("<svg")
        .ok_or_else(|| "svg start tag not found in rendered qrcode".to_string())?;
    let end = svg
        .find("</svg>")
        .map(|idx| idx + "</svg>".len())
        .ok_or_else(|| "svg closing tag not found in rendered qrcode".to_string())?;

    Ok(svg[start..end].to_string())
}
