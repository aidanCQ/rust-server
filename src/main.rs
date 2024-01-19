use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(&"dynamic", &"./src/templates/dynamic.hbs")
        .unwrap();

    // let x = format!("123");
    let static_dir = warp::path("static").and(warp::fs::dir("src/static"));

    let index = warp::path!()
        .and(warp::query::<HashMap<String, String>>())
        .map(move |map: HashMap<String, String>| {
            let mut data = BTreeMap::new();
            let val = map.get("fruit");
            let resolved = String::from("bad apple!");
            data.insert("fruit", val.unwrap_or(&resolved));
            let html = handlebars.render(&"dynamic", &data).unwrap();
            warp::reply::html(html.to_owned())
        });

    let full_router = index.or(static_dir);
    // let hello2 = warp::path!().map(|| format!("Hello, whose agent is "));
    warp::serve(full_router).run(([127, 0, 0, 1], 3000)).await;
}
