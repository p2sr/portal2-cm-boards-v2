#![allow(unused_imports)]
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::OpenOptions;
use std::io::Write;
use std::str;

#[derive(Debug)]
pub struct DemoData {
    file_url: Option<String>,
    partner_name: Option<String>,
    parsed_successfully: bool,
    sar_version: Option<String>,
    cl_id: Option<i32>,
}

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
pub async fn receive_multiparts(mut payload: Multipart) -> impl Responder {
    let mut received_parts = Vec::new();
    let mut values = DemoData {
        file_url: None,
        partner_name: None,
        parsed_successfully: false,
        sar_version: None,
        cl_id: None,
    };
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
            // TODO: Uncomment this code when we want the demos to be written (works trust me)
            // let mut file = OpenOptions::new()
            //     .create(true)
            //     .write(true)
            //     .open(format!("./{}", file_name))
            //     .unwrap(); // TODO: unwraps...
            // file.write_all(&content_data).unwrap();

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
            // TODO: Match on field_name
            match field_name {
                "file_url" => values.file_url = Some(result_string.to_string()),
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
                "cl_id" => values.cl_id = Some(result_string.parse::<i32>().unwrap_or(0)),
                _ => eprintln!("Got an unexpected field."),
            }
            println!("result: {} - {}", field_name, result_string);

            let x = ReceivedPart {
                content_data,
                content_type,
                content_disposition,
            };
            received_parts.push(x);
        }
    }
    // TODO: Send to database.
    println!("{:#?}", values);
    let mut received_parts_string = String::new();
    let mut counter = 0;
    #[allow(clippy::explicit_counter_loop)]
    for received_part in received_parts {
        received_parts_string.push_str(&format!("\nPart {}\n", counter));
        received_parts_string.push_str(&received_part.to_string());
        counter += 1;
    }
    HttpResponse::Ok().body(received_parts_string)
}
