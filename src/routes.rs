use crate::graphql::{RequestContext, TestSchema};
use crate::settings::Settings;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use uuid::Uuid;

#[get("/ecv")]
pub async fn ecv() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body("OK"))
}

#[get("/graphql")]
pub async fn playground() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
}

#[post("/graphql")]
pub async fn index(
    schema: web::Data<TestSchema>,
    http_request: HttpRequest,
    gql_request: Request,
) -> Response {
    let traceparent_header_value = http_request.headers().get("traceparent");

    let traceparent = if traceparent_header_value.is_some() {
        String::from(traceparent_header_value.unwrap().to_str().unwrap_or(""))
    } else {
        Uuid::new_v4().to_string()
    };

    let mut request = gql_request.into_inner();

    // let mut cookies_hash_map: HashMap<String, String> = HashMap::new();
    // if let Ok(cookies) = http_request.cookies() {
    //     for cookie in cookies.iter() {
    //         cookies_hash_map.insert(cookie.name().to_string(), cookie.value().to_string());
    //     }
    // }

    request = request.data(RequestContext {
        traceparent,
        headers: http_request.headers().to_owned(),
    });
    schema.execute(request).await.into()
}

pub fn init(settings: Settings) -> impl Fn(&mut web::ServiceConfig) {
    move |service_config: &mut web::ServiceConfig| {
        if settings.graphql.playground_enabled {
            service_config.service(playground);
        }
        service_config.service(ecv);
        service_config.service(index);
    }
}
