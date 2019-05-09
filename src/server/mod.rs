use crate::config::Config;
use crate::util;
use crate::content::Content;
use std::path::PathBuf;
use actix_web::{server, App, HttpRequest, Result, dev::Handler, http};
use actix_web::fs::{StaticFileConfig, NamedFile};
use actix_web::http::header::DispositionType;

pub struct Server {
    pub generated_address: String,
    pub address: String,
    pub route: String
}

#[derive(Default)]
struct HeaderConfig;


impl StaticFileConfig for HeaderConfig {
    fn content_disposition_map(typ: mime::Name) -> DispositionType {
        DispositionType::Attachment
    }
}

struct Handle(PathBuf);

impl<S> Handler<S> for Handle {
    type Result = Result<NamedFile<HeaderConfig>>;

    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        Ok(NamedFile::open_with_config(self.0.clone(), HeaderConfig)?)
    }
}

impl Server {
    pub fn new(cfg: &Config) -> Server {
        let address = format!("{}:{}", cfg.iface, cfg.port);
        let rand_path = util::random_url_path();
        let generated_address = format!("http://{}/{}", address, rand_path);
        let route = format!("/{}", rand_path);

        Server {generated_address, address, route}
    }

    pub fn serve(&self, content: Content) {

        let sys = actix::System::new("Qr Transfer");

        server::new( move || {
            let cloned = content.path.clone();
            App::new()
                .resource(r"/{param}", |r|
                    r.method(http::Method::GET).h(Handle(cloned)))
                .finish()
            })
            .bind(&self.address).expect("Cannot bind to address")
            .start();

        let _ = sys.run();
    }
}