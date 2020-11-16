/// Shamelessly stolen from https://blog.logrocket.com/file-upload-and-download-in-rust/
use std::convert::Infallible;

use bytes::BufMut;
use futures::TryStreamExt;
use warp::{hyper::StatusCode, multipart::FormData, multipart::Part, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let upload = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5000000))
        .and_then(upload);

    let router = upload.recover(rejection);
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    println!("uploading ...");
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }

                    "application/docx" => {
                        file_ending = "docx";
                    }

                    v => {
                        eprintln!("invalid type {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("error reading file {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!(
                "/tmp/uploads/{}.{}",
                uuid::Uuid::new_v4().to_string(),
                file_ending
            );

            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprintln!("error writing file {}", e);
                warp::reject::reject()
            })?;
        }
    }

    Ok("Success")
}

async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Bad request".to_string())
    } else {
        eprintln!("unhandled error {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
