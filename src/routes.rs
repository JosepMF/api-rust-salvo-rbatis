use salvo::{prelude::*, hyper::Method};

use crate::database::{Post, RB};

mod request_mod {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PostReq {
        id: Option<i32>,
        title: Option<String>,
        description: Option<String>
    }
}

#[handler]
pub async fn posts(req: &mut Request, res: &mut Response) {
    match req.method() {
        &Method::GET => {
            let post_vector = Post::select_all(&mut RB.clone()).await.unwrap();
            res.render(Json(post_vector));
        },
        &Method::POST => {
            let post = req.parse_json::<Post>().await.unwrap();
            Post::insert(&mut RB.clone(), &post).await.unwrap();
            res.render(Json(post));
        },
        &Method::DELETE => {
            let post_id = req.query::<i32>("id").unwrap();
            Post::delete_in_column(&mut RB.clone(), "id", &[post_id.to_string()]).await.unwrap();
            res.render(Text::Html("The text was deleted successfully"));
        },
        &Method::PUT => {
            let post_id = req.query::<i32>("id").unwrap();
            let post  = req.parse_json::<Post>().await.unwrap();

            Post::update_by_column_value(&mut RB.clone(), &post, "id", &rbs::Value::I32(post_id)).await.unwrap();
            res.render(Text::Html("The text was updated successfully"));
        },
        _ => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Html("the methos is not valid"));
        }
    }
}