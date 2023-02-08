use std::{path::Path, fs};

use uuid::Uuid;
use salvo::{hyper::Method, prelude::*};

use crate::database::{Post, RB};

#[handler]
pub async fn posts(req: &mut Request, res: &mut Response) {
    match req.method() {
        &Method::GET => match req.query::<i32>("id") {
            Some(id) => {
                let post = Post::select_by_column(&mut RB.clone(), "id", id)
                    .await
                    .unwrap();
                res.render(Json(post));
            }
            _ => {
                let post_vector = Post::select_all(&mut RB.clone()).await.unwrap();
                res.render(Json(post_vector));
            }
        },
        &Method::POST => {
            let file = req.file("file").await;
            if let Some(file) = file {
                
                let dest = format!("img/{}", format!("{}{}", Uuid::new_v4(), file.name().unwrap_or("file")));
                fs::copy(&file.path(), Path::new(&dest)).unwrap();

                let post_data = req.parse_body::<Post>().await.unwrap();
                let post = Post::new(post_data.title, post_data.description, Some(dest));
                Post::insert(&mut RB.clone(), &post).await.unwrap();

                res.render(Json(post));
            } else {
                res.set_status_code(StatusCode::BAD_REQUEST);
                res.render(Text::Plain("file not found in the request"));
            }
        }
        &Method::DELETE => {
            let post_id = req.query::<i32>("id").unwrap();
            Post::delete_in_column(&mut RB.clone(), "id", &[post_id.to_string()])
                .await
                .unwrap();
            res.render(Text::Html("The text was deleted successfully"));
        }
        &Method::PUT => {
            let post_id = req.query::<i32>("id").unwrap();
            let post = req.parse_json::<Post>().await.unwrap();

            Post::update_by_column_value(&mut RB.clone(), &post, "id", &rbs::Value::I32(post_id))
                .await
                .unwrap();
            res.render(Text::Html("The text was updated successfully"));
        }
        _ => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Html("the methos is not valid"));
        }
    }
}
