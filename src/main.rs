use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::net::SocketAddr;

mod config;
use config::{load_config, GatewayConfig, ServiceConfig};

mod backend;
use backend::{ggml, openai};

// type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let gateway_config = load_config("config.yml");

    let socket_addr = format!(
        "{ip}:{port}",
        ip = gateway_config.socket_addr.ip,
        port = gateway_config.socket_addr.port
    );
    let addr: SocketAddr = socket_addr.parse().unwrap();

    let new_service = make_service_fn(move |_| {
        let config = gateway_config.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let config = config.clone();
                handle_request(req, config)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(
    req: Request<Body>,
    config: GatewayConfig,
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    // get service config
    let service_config = match get_service_config(path.clone(), &config.services) {
        Some(service_config) => service_config,
        None => {
            return not_found();
        }
    };

    match service_config.ty {
        config::ServiceType::Openai => openai::handle_openai_request(req, service_config).await,
        config::ServiceType::Llama2 => ggml::handle_llama_request(req, service_config).await,
        config::ServiceType::Test => Ok(Response::new(Body::from("echo test"))),
    }
}

fn get_service_config<'a>(path: &str, services: &'a [ServiceConfig]) -> Option<&'a ServiceConfig> {
    services.iter().find(|c| path.starts_with(&c.path))
}

fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    Ok(response)
}
