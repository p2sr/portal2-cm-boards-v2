use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::OpenOptions;
use std::io::Write; // bring trait into scope

pub struct DemoData {
    id: i32,
    drive_url: Option<String>,
    partner_name: Option<String>,
    parsed_successfully: bool,
    sar_version: Option<String>,
    cl_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DemoUpload {
    sar_version: Option<String>,
}

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

// TODO: A lot of this is just example code for debugging, not super useful, will need to
//  a. Handle renaming/db interactions (update demo table/specific time that is being uploaded)
//  b. Pass to google drive
//  c. Look to see if there is anything special needed for auto-submit
//  d. Integrate Parsing
// Code Reference: https://github.com/Ujang360/actix-multipart-demo/blob/main/src/main.rs
// Google Drive API: https://docs.rs/google-drive/0.2.4/google_drive/
#[post("/demo")]
pub async fn receive_multiparts(mut payload: Multipart) -> impl Responder {
    let mut received_parts = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().to_string();
        // let x = field.content_disposition().get_filename();
        // println!("{:#?}", x.unwrap());
        // content_disposition() now returns a &ContentDisposition, rather than an Option<ContentDisposition>
        let content_disposition = Some(format!("{:#?}", field.content_disposition()));
        let mut content_data = Vec::new();
        while let Some(Ok(chunk)) = field.next().await {
            content_data.extend(chunk);
        }
        // let mut file = OpenOptions::new().create(true).write(true).open("./demo.dem").unwrap();
        // file.write_all(&content_data);
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
