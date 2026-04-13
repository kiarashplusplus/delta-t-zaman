use serde::{Serialize, Deserialize};
#[derive(Serialize)]
struct GetArgs<'a> {
    path: &'a str,
    key: &'a str,
}
