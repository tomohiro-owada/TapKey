use base64::Engine;
use image::{GrayImage, Luma};
use qrcode::QrCode;
use std::net::IpAddr;

/// ローカルIPアドレスを取得
pub fn get_local_ip() -> Option<IpAddr> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

/// QRコードをBase64エンコードされたPNG画像として生成
pub fn generate_qr_code(url: &str) -> Result<String, String> {
    let code = QrCode::new(url.as_bytes()).map_err(|e| format!("QRコード生成失敗: {}", e))?;

    // QRコードのモジュールを取得
    let qr_width = code.width();
    let scale = 8; // スケールファクター
    let margin = 4; // マージン
    let img_size = (qr_width + margin * 2) * scale;

    // グレースケール画像を作成
    let mut img = GrayImage::new(img_size as u32, img_size as u32);

    // 背景を白に
    for pixel in img.pixels_mut() {
        *pixel = Luma([255u8]);
    }

    // QRコードを描画
    for y in 0..qr_width {
        for x in 0..qr_width {
            let color = if code[(x, y)] == qrcode::Color::Dark {
                Luma([0u8])
            } else {
                Luma([255u8])
            };

            // スケール適用して描画
            for sy in 0..scale {
                for sx in 0..scale {
                    let px = (x + margin) * scale + sx;
                    let py = (y + margin) * scale + sy;
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }

    // PNGとしてエンコード
    let mut png_data: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder
        .encode(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ColorType::L8,
        )
        .map_err(|e| format!("PNG変換失敗: {}", e))?;

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&png_data);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

/// サーバーURLを生成
pub fn get_server_url(port: u16) -> String {
    if let Some(ip) = get_local_ip() {
        format!("http://{}:{}", ip, port)
    } else {
        format!("http://localhost:{}", port)
    }
}

/// サーバーURL（PIN付き）を生成
pub fn get_server_url_with_pin(port: u16, pin: &str) -> String {
    let base_url = get_server_url(port);
    if pin.is_empty() {
        base_url
    } else {
        format!("{}?pin={}", base_url, pin)
    }
}
