use crate::generated::sample_service::sample_service_server::{SampleService, SampleServiceServer};
use crate::generated::{fuga::Fuga, hoge::GetHogeRequest, hoge::Hoge};
use std::collections::HashMap;
use tonic::{Code, Request, Response, Status};

mod generated;

pub struct Service;

#[tonic::async_trait]
impl SampleService for Service {
    async fn get_hoge(&self, request: Request<GetHogeRequest>) -> Result<Response<Hoge>, Status> {
        let hoges: HashMap<&str, (&str, Fuga)> = HashMap::from([
            ("1", ("hoge1", Fuga::Foo)),
            ("2", ("hoge2", Fuga::Bar)),
            ("3", ("hoge3", Fuga::Baz)),
            ("4", ("hoge4", Fuga::Unspecified)),
        ]);

        let id: &str = &request.get_ref().id;
        let hoge = hoges.get(id);

        match hoge {
            Some(r) => Ok(Response::new(Hoge {
                id: id.to_string(),
                name: r.0.to_string(),
                fuga: r.1.clone() as i32,
            })),
            None => {
                log::warn!("cannot find {}", id);
                Err(Status::new(Code::NotFound, "hoge is not found."))
            }
        }
    }
}

pub fn create_server() -> SampleServiceServer<Service> {
    let service = Service {};
    SampleServiceServer::new(service)
}
