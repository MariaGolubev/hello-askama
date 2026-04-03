use std::sync::LazyLock;

use askama::Template;

#[derive(Template)]
#[template(path = "partials/qrcode.html")]
pub struct QrCodeTemplate {}

impl askama::filters::HtmlSafe for QrCodeTemplate {}

impl QrCodeTemplate {
    pub fn qrcode() -> String {
        static LOCAL_QR_CODE: LazyLock<String> = LazyLock::new(|| {
            let address = local_ip_address::local_ip().unwrap();

            let qrcode = qrcode::QrCode::new(format!("http://{}:{}", address, 3000)).unwrap();

            let svg = qrcode
                .render::<qrcode::render::svg::Color>()
                .dark_color(qrcode::render::svg::Color("#000000"))
                .light_color(qrcode::render::svg::Color("#ffffff"))
                .build();

            svg[svg.find("<svg").unwrap()..svg.find("</svg>").unwrap() + "</svg>".len()].to_string()
        });

        LOCAL_QR_CODE.clone()
    }
}