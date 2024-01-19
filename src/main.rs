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

    let static_dir = warp::path("static").and(warp::fs::dir("src/static"));

    let index = warp::path!()
        .and(warp::query::<HashMap<String, String>>())
        .map(move |map: HashMap<String, String>| {
            let mut data = BTreeMap::new();
            let val = map.get("message");
            let resolved = String::from("No message given. Provide a query param of the form message={yourmessage} to see it!");
            data.insert("message", val.unwrap_or(&resolved));
            let html = handlebars.render(&"dynamic", &data).unwrap();
            warp::reply::html(html.to_owned())
        });

    let full_router = index.or(static_dir);

    warp::serve(full_router).run(([127, 0, 0, 1], 3000)).await;
}
