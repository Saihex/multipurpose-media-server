use actix_web::{
    web::{self, Bytes},
    App, HttpResponse, HttpServer,
};
use async_stream::stream;
use image::{imageops, ImageFormat};
use std::{
    collections::HashMap, io::{Cursor, Read}, path::PathBuf
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening...");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index_handler)))
            .service(web::resource("/{filename:.*}").route(web::get().to(resize_image)))
        // Route for image resizing
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn index_handler() -> HttpResponse {
    if let Ok(content) = std::fs::read_to_string("/collection/index.html") {
        HttpResponse::Ok().body(content)
    } else {
        HttpResponse::InternalServerError().body("Failed to read index.html")
    }
}

async fn resize_image(
    info: web::Path<(String,)>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let filename = &info.0;
    let path = PathBuf::from(format!("/collection/{}", filename));

    // Determine the file extension
    let extension = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_lowercase(),
        None => return HttpResponse::BadRequest().finish(), // Invalid file path
    };

    // Parse the downscale query parameter
    let jas = &String::new();
    let downscale_param = query.get("downscale").unwrap_or(jas);

    let dat_mode: u8 = match downscale_param.as_str() {
        "m" => 1,
        "s" => 2,
        "fm" => 3,
        "fs" => 4,
        _ => 0,
    };

    // Check if it's a supported image format (JPEG or PNG)
    if valid_image_file(&extension) && dat_mode != 0 {
        // Load the image
        let image = match image::open(&path) {
            Ok(img) => img,
            Err(_) => return HttpResponse::NotFound().content_type("text/plain").body("404: Not found."),
        };

        let resized_image = match dat_mode {
            1 => imageops::resize(
                &image,
                image.width() / 2,
                image.height() / 2,
                imageops::FilterType::Nearest,
            ),
            2 => imageops::resize(
                &image,
                image.width() / 4,
                image.height() / 4,
                imageops::FilterType::Nearest,
            ),
            3 => imageops::resize(
                &image,
                512,
                solve_ratio(image.width(), image.height(), 512),
                imageops::FilterType::Nearest,
            ),
            4 => imageops::resize(
                &image,
                256,
                solve_ratio(image.width(), image.height(), 256),
                imageops::FilterType::Nearest,
            ),
            _ => image.into(),
        };
        // Convert the resized image to a byte vector
        let only_rgb = extension.as_str() == "jpeg" || extension.as_str() == "jpg";
        let buf = Vec::new();
        let mut cursor = Cursor::new(buf);

        if only_rgb {
            let mut rgb_image =
                image::ImageBuffer::new(resized_image.width(), resized_image.height());

            for (x, y, pixel) in resized_image.enumerate_pixels() {
                let rgb_pixel = image::Rgb([pixel[0], pixel[1], pixel[2]]);
                rgb_image.put_pixel(x, y, rgb_pixel);
            }

            match rgb_image.write_to(
                &mut cursor,
                ImageFormat::from_extension(&extension).unwrap(),
            ) {
                Ok(_) => {
                    return HttpResponse::Ok()
                        .append_header(("Cache-Control", "public, max-age=7200"))
                        .content_type(format_to_content_type(&extension))
                        .body(cursor.into_inner())
                }
                Err(e) => {
                    println!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        } else {
            match resized_image.write_to(
                &mut cursor,
                ImageFormat::from_extension(&extension).unwrap(),
            ) {
                Ok(_) => {
                    return HttpResponse::Ok()
                        .append_header(("Cache-Control", "public, max-age=7200"))
                        .content_type(format_to_content_type(&extension))
                        .body(cursor.into_inner())
                }
                Err(e) => {
                    println!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
    } else {
        // If it's not an image format, serve the file directly
        match std::fs::File::open(&path) {
            Ok(mut file) => {
                let large_data_stream = stream! {

                    let mut chunk = vec![0u8; 10 * 1024 * 1024]; // I decalare the chunk size here as 10 mb

                    loop {

                        match file.read(&mut chunk) {

                            Ok(n) => {
                                if n == 0 {
                                    break;
                                }


                                yield Result::<Bytes, std::io::Error>::Ok(Bytes::from(chunk[..n].to_vec())); // Yielding the chunk here

                            }

                            Err(e) => {

                                yield Result::<Bytes, std::io::Error>::Err(e);
                                break;
                            }
                        }
                    }
                };

                let file_extension = &path
                    .extension()
                    .map(|ext| ext.to_string_lossy().into_owned())
                    .unwrap();

                HttpResponse::Ok()
                    .append_header(("Cache-Control", "public, max-age=7200"))
                    .content_type(format_to_content_type(file_extension))
                    .streaming(large_data_stream)
            }
            Err(_) => HttpResponse::NotFound().content_type("text/plain").body("404: Not found."),
        }
    }
}

fn format_to_content_type(file_extension: &String) -> &str {
    match file_extension.as_str() {
        "jpeg" | "jpg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}

fn valid_image_file(file_extension: &String) -> bool {
    match file_extension.as_str() {
        "jpg" | "jpeg" => true,
        "png" => true,
        _ => false,
    }
}

fn solve_ratio(a: u32, b: u32, d: u32) -> u32 {
    // Calculate the value of X using the given ratios
    let x = (d * b) / a;
    // Return the value of X
    x
}
