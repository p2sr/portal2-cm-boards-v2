use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use raze::api::*;
use raze::utils::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::Write;
use sqlx::PgPool;
use crate::config::Config;
use crate::tools::datamodels::{Demos, DemoInsert};
use std::str;

#[derive(Debug)]
struct ReceivedPart {
    content_type: String,
    content_disposition: Option<String>,
    content_data: Vec<u8>,
}

impl Display for ReceivedPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let content_disposition = match self.content_disposition.as_ref() {
            None => "".to_string(),
            Some(cd) => cd.to_string(),
        };

        write!(
            f,
            "content-type: {}\ncontent-disposition: {}\ncontent-length: {}\n",
            self.content_type,
            content_disposition,
            self.content_data.len()
        )
    }
}

// I had a fundamental misunderstanding of multipart. I can send over multiple form fields, not just files. I need to handle the different fields now.
//  a. Handle renaming/db interactions (update demo table/specific time that is being uploaded)
//  b. Pass to backblaze
//  c. Look to see if there is anything special needed for auto-submit
//  d. Integrate Parsing
// Code Reference: https://github.com/Ujang360/actix-multipart-demo/blob/main/src/main.rs
#[post("/demo")]
pub async fn receive_multiparts(mut payload: Multipart, config: web::Data<Config>, pool: web::Data<PgPool>) -> impl Responder {
    let mut received_parts = Vec::new();
    let mut file_id: Option<String> = None;
    let mut values = DemoInsert {
        file_id: "None".to_string(),
        partner_name: None,
        parsed_successfully: false,
        sar_version: None,
        cl_id: 0,
    };
    //println!("{} - {} - {}", config.backblaze.keyid, config.backblaze.key, config.backblaze.bucket);
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().to_string();

        // Note: content_disposition() now returns a &ContentDisposition, rather than an Option<ContentDisposition>
        let content_disposition = Some(format!("{:#?}", field.content_disposition()));
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }

        let name = field.content_disposition().get_name();
        let field_name = if let Some(name) = name {
            name
        } else {
            "NO-KEY-PROVIDED"
        };

        let file_name = field.content_disposition().get_filename();

        // Handle the case where we were passed a file
        if let Some(file_name) = file_name {
            use std::fs;
            match fs::create_dir_all("./demos") {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to create demo directory locally -> {}", e))
                },
            }
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("./demos/{}", file_name));
            match file {
                Ok(mut res) => match res.write_all(&content_data) {
                    Ok(_) => (),
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("Failed to write demo locally -> {}", e))
                    }
                },
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to write demo locally -> {}", e))
                }
            };
            // TODO: Parse Demo
            // TODO: Setup BackBlaze credentials in .env
            let client = reqwest::ClientBuilder::new().build().unwrap();
            // Ref: https://docs.rs/raze/0.4.1/raze/api/fn.b2_authorize_account.html
            let auth = b2_authorize_account(
                &client,
                format!("{}:{}", config.backblaze.keyid, config.backblaze.key)
            )
            .await
            .unwrap();
            let upload_auth = b2_get_upload_url(&client, &auth, config.backblaze.bucket.clone())
                .await
                .unwrap();
            let file = tokio::fs::File::open(format!("./demos/{}", file_name))
                .await
                .unwrap();
            let metadata = file.metadata().await.unwrap();
            let size = metadata.len();
            let modf = metadata
                .modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                * 1000;

            let param = FileParameters {
                file_path: file_name,
                file_size: size,
                content_type: None,
                content_sha1: Sha1Variant::HexAtEnd,
                last_modified_millis: modf,
            };

            let stream = reader_to_stream(file);
            let stream = BytesStreamHashAtEnd::wrap(stream);
            let stream = BytesStreamThrottled::wrap(stream, 500000000);

            let body = reqwest::Body::wrap_stream(stream);
            let resp1 = b2_upload_file(&client, &upload_auth, body, param)
                .await
                .unwrap();
            file_id = resp1.file_id;
            // Delete Demo
            let res = remove_file(format!("./demos/{}", file_name));
            match res {
                Ok(_) => (),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to delete demo locally -> {}", e))
                }
            }
            let x = ReceivedPart {
                content_data,
                content_type,
                content_disposition,
            };
            received_parts.push(x);
        } else {
            // Handle the case where we are passed a text value.
            let result_string = match str::from_utf8(&content_data) {
                Ok(our_string) => our_string,
                Err(e) => {
                    eprintln!("Invalid UTF-8 sequence: {}", e);
                    "ERROR"
                }
            };
            match field_name {
                "partner_name" => values.partner_name = Some(result_string.to_string()),
                "parsed_successfully" => {
                    values.parsed_successfully = {
                        match result_string {
                            "false" => false,
                            "true" => true,
                            _ => false,
                        }
                    }
                }
                "sar_version" => values.sar_version = Some(result_string.to_string()),
                "cl_id" => values.cl_id = result_string.parse::<i64>().unwrap_or(0),
                _ => eprintln!("Got an unexpected field."),
            }
            // println!("result: {} - {}", field_name, result_string);

            let x = ReceivedPart {
                content_data,
                content_type,
                content_disposition,
            };
            received_parts.push(x);
        }
    }
    if let Some(file_id) = file_id {
        values.file_id = file_id;
    }
    //println!("{:#?}", values);
    let res = Demos::insert_demo(&pool, values).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to add demo to database -> {}", e)),
    }
    // Debug return info
    // let mut received_parts_string = String::new();
    // let mut counter = 0;
    // #[allow(clippy::explicit_counter_loop)]
    // for received_part in received_parts {
    //     received_parts_string.push_str(&format!("\nPart {}\n", counter));
    //     received_parts_string.push_str(&received_part.to_string());
    //     counter += 1;
    // }
    // HttpResponse::Ok().body(received_parts_string)
}
