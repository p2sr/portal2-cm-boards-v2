#![allow(unused_imports)]
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::OpenOptions;
use std::io::Write;
use std::str;

// pub struct DemoData {
//     id: i32,
//     drive_url: Option<String>,
//     partner_name: Option<String>,
//     parsed_successfully: bool,
//     sar_version: Option<String>,
//     cl_id: i32,
// }
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct DemoUpload {
//     sar_version: Option<String>,
// }

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
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().to_string();
        // TODO: Match on data type for deserialization
        // let file_name = field.content_disposition().get_filename();
        // if let Some(file_name) = file_name {
        //     println!("{:#?}", file_name);
        // }
        // let name = field.content_disposition().get_name();
        // if let Some(name) = name {
        //     println!("{:#?}", name);
        // }
        // Note: content_disposition() now returns a &ContentDisposition, rather than an Option<ContentDisposition>
        let content_disposition = Some(format!("{:#?}", field.content_disposition()));
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }
        // TODO:: Write out the demo file locally, use that for parsing.
        // let name = field.content_disposition().get_name();
        // let name = if let Some(name) = name {
        //     name
        // } else {
        //     "notfound"
        // };
        // let mut file = OpenOptions::new().create(true).write(true).open(format!("./{}", name)).unwrap();
        // file.write_all(&content_data).unwrap();
        // TODO: Use from_utf8 to deserialize the data we recieve (SKIP OVER DEMO FILES)
        // let s = match str::from_utf8(&content_data) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         eprintln!("Invalid UTF-8 sequence: {}", e);
        //         "ERROR"
        //     },
        // };
        // println!("result: {}", s);
        let x = ReceivedPart {
            content_data,
            content_type,
            content_disposition,
        };
        received_parts.push(x);
    }
    let mut received_parts_string = String::new();
    let mut counter = 0;

    for received_part in received_parts {
        received_parts_string.push_str(&format!("\nPart {}\n", counter));
        received_parts_string.push_str(&received_part.to_string());
        counter += 1;
    }
    HttpResponse::Ok().body(received_parts_string)
}
