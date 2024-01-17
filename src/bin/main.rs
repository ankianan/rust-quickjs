use std::{fs::File, io::Read};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use quick_js::{Context};

#[get("/")]
async fn render(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body({
        let value = data.context.eval_as::<String>("renderToStringEmbed()").unwrap();
        String::from(value)
    })
}

struct AppState {
    context: Context
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
            let mut file = File::open("react-embed/index.js").unwrap();
            let mut contents = String::new();
            
            file.read_to_string(&mut contents).unwrap();
            
            let context = Context::new().unwrap();  
            let _ = context.eval(&contents);

            App::new()
            .app_data(web::Data::new(AppState {
                context
            }))
            .service(render)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}