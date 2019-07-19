//// Documentation attributes

// html_favicon_url
//
// Control the favicon in the browser when viewing your docs.
//
// The attribute adds this HTML:
//
//     <link rel="shortcut icon" href="{}"> 
//
#![doc(html_favicon_url = "https://example.com/favicon.ico")]

// html_logo_url
//
// Control the logo in the upper left hand side of the docs.
//
// The attribute adds this HTML:
//
//     <a href='index.html'><img src='{}' alt='logo' width='100'></a>
//
#![doc(html_logo_url = "https://example.com/logo.jpg")]

////

#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, guard, middleware, web, App, Error, Responder, HttpRequest, HttpResponse, HttpServer, Result};
use bytes::Bytes;
use futures::unsync::mpsc;
use futures::{future::ok, Future, Stream};

/// HTTP status page handlers
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/invalid
///
///   * You should see the page for HTTP status 404 "not found".
///
/// This function shows a typical example handler for HTTP status code 404.
/// The repo contains simple status pages for a bunch of HTTP status codes.
///
/// We prefer the file path of `http/status/XXX.html` because we want our 
/// apps to be able to work well with multiple protocols and results.
/// You can set the file paths as you like, both here and on disk.
///
/// TODO: add handlers for more HTTP status codes, and ideally dynamically.
///
fn http_status_404(
) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/http/status/404.html")?.
    set_status_code(StatusCode::NOT_FOUND))
}

/// Demo favicon file.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-favicon-file>
///
///   * You should see the favicon file, which is `favicon.ico`.
///
//#[get("/demo-favicon-file")]
fn demo_favicon_file(
) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// Demo static page.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-static-page>
///
///   * You should see the page "Demo static page".
//
/// This funciton demonstrates a typical handler for a static page.
///
//#[get("/demo-static-page")]
fn demo_static_page(
    session: Session, req: HttpRequest
) -> Result<HttpResponse> {
    println!("{:?}", req);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/demo-static-page.html"))
    )
}

/// Demo session counter.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-session-counter>
///
///   * You should see the page "Demo session counter".
///
///   * You should see console output that says "session counter:" and a number.
///
///   * Reload the web page, then you should see the console output increment.
///
/// This function demonstrates a simple handler for a dynamic page
/// that increments a session counter of visits to the page.
///
//#[get("/demo-session-counter")]
fn demo_session_counter(
    session: Session, req: HttpRequest
) -> Result<HttpResponse> {
    println!("{:?}", req);
    let mut counter;
    if let Some(x) = session.get::<i32>("counter")? {
        counter = x + 1;
    } else {
        counter = 1;
    }
    session.set("counter", counter)?;
    println!("session counter:{}", counter);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/demo-session-counter.html")))
}

/// Demo path parameters.
///
/// To use this:
///
///    * Run the app then browse <http://127.0.0.1:8080/demo-path-parameters/a/b/c>
///
///    * You should see the text "Demo path parameters: a, b, c".
///
/// This function demonstrates a URL route that uses a path parameter.
///
///   * The `path` variable in this demo contains 3 strings.
///
///   * The `path` variable parameters are `path.0`, `path.1`, `path.2`.
///
/// This function is connected via Actix:
///
/// ```
/// .service(
///     web::resource("/demo-path-parameters/{x}/{y}/{z}")
///     .route(web::get().to(demo_path_parameters)),
/// )
/// ```
///
fn demo_path_parameters(
    req: HttpRequest, 
    path: web::Path<(String, String, String,)>
) -> HttpResponse {
    println!("{:?}", req);
    let text = format!("Demo path parameters: {}, {}, {}", path.0, path.1, path.2);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(text)
}

/// Demo async future.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-async-future>
///
///   * You should see the text "Demo async future".
///     
/// This function demonstrates an asynchronous handler,
/// where the handler returns a future.
///
fn demo_async_future(
    req: HttpRequest
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("Demo async future"))
}

/// Demo async future with path parameters.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-async-future-with-path-parameters/a/b/c>
///
///   * You should see the text "Demo async future with path parameters: a, b, c".
///
/// This section demonstrates an asynchronous handler,
/// specifically where the handler returns a future.
/// The handler expects 3 named parameters x, y, z.
///
fn demo_async_future_with_path_parameters(
    req: HttpRequest
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Demo async future with path parameters: {}, {}, {}",
            req.match_info().get("x").unwrap(),
            req.match_info().get("y").unwrap(),
            req.match_info().get("z").unwrap(),
        )))
}

/// Demo streaming body.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-streaming-body>
///
///   * You should see the text "Demo streaming body".
///
/// This function demonstrates a streaming body handler,
/// which is good for returning asynchronous data.
///
fn demo_streaming_body() -> HttpResponse {
    let text = "Demo streaming body";
    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));
    HttpResponse::Ok()
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

/// Demo streaming body with path parameters.
///
/// To use this:
///
///   * Run the app then browse <http://127.0.0.1:8080/demo-streaming-body-with-path-parameters/a/b/c>
///
///   * You should see the text "Demo streaming body with path parameters: a, b, c".
///
/// This function demonstrates a streaming body handler,
/// which is good for returning asynchronous data.
///
fn demo_streaming_body_with_path_parameters(
    path: web::Path<(String, String, String,)>
) -> HttpResponse {
    let text = format!("Demo streaming body with path parameters: {}, {}, {}", path.0, path.1, path.2);
    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));
    HttpResponse::Ok()
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

//////////////////////////////////////////////////////////////////////////

/// Askama templates
///
/// Templates are a way to render HTML with variables, akin to Jinja, Handlebars, etc.

extern crate askama; // Askama provides the Template trait and custom derive macro
use askama::Template; // Bring trait in scope
#[derive(Template)] // Generate the code
#[template(path = "hello.html")] // Use the template in this path, relative to the templates dir in the crate root

/// Define a struct of fields; each field corresponds to a template variable.
/// The field name must match the template variable name in the template HTML file.
/// The name of the struct can be anything; we favor a convention <basename>Template.
struct HelloTemplate<'a> { 
    name: &'a str, 
}

/// TODO   
fn demo_askama() {
    let hello_world = HelloTemplate { name: "world" }; // Instantiate a struct that holds the template's variables
    println!("{}", hello_world.render().unwrap()); // Render the template
}

//////////////////////////////////////////////////////////////////////////

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/demo-favicon-file").route(web::get().to(demo_favicon_file)))
            .service(web::resource("/demo-static-page").route(web::get().to(demo_static_page)))
            .service(web::resource("/demo-session-counter").route(web::get().to(demo_session_counter)))
            .service(web::resource("/demo-path-parameters/{x}/{y}/{z}").route(web::get().to(demo_path_parameters)))
            .service(web::resource("/demo-async-future").route(web::get().to_async(demo_async_future)))
            .service(web::resource("/demo-async-future-with-path-parameters/{x}/{y}/{z}").route(web::get().to_async(demo_async_future_with_path_parameters)))
            .service(web::resource("/demo-streaming-body").route(web::get().to(demo_streaming_body)))
            .service(web::resource("/demo-streaming-body-with-path-parameters/{x}/{y}/{z}").route(web::get().to(demo_streaming_body_with_path_parameters)))
            //
            // TODO
            //
            .service(
                web::resource("/test")
                .to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            //
            // TODO
            //
            .service(
                web::resource("/error")
                .to(|| {
                    error::InternalError::new(
                        io::Error::new(io::ErrorKind::Other, "test"),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
            }))
            //
            // Static file handler, which serves a directory of static files.
            //
            // In a production app, this static file handler would typically
            // go to a more-specific file directory, such as for static assets.
            //
            .service(
                fs::Files::new("/static", "static")
                .show_files_listing()
            )
            //
            // Root service handler, which serves a static home page.
            //
            // In a production app, this root handler would typically serve a 
            // dynamic page e.g. a user's dashboard, rather than a static page.
            //
            .service(
                web::resource("/").
                route(
                    web::get().to(|req: HttpRequest| {
                        HttpResponse::Found()
                            .header(header::LOCATION, "static/home.html")
                            .finish()
                    })
                )
            )
            //
            // Default service handler, which goes last.
            //
            // For a GET request, this handler returns HTTP status 404, 
            // meaning "not found", and a static http status page.
            //
            // For any other request, such as POST, this handler returns
            // HTTP status that the method is not allowed.
            //
            .default_service(
                web::resource("")
                    .route(
                        web::get().to(http_status_404)
                    )
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}