use crate::types::{SessionRequest, SessionResponse, StopRequest};
use shiplift::{Docker, ContainerOptions, RmContainerOptions};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::json;

pub async fn start_instance(req: SessionRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let docker = Docker::new();
    let uuid = Uuid::new_v4().to_string();

    let container_opts = ContainerOptions::builder("docker.io/library/icp_image")
        .name(&uuid)
        .env(vec!["PASSWORD=icpad", "DOCKER_USER=node"])
        .expose(8443, "tcp", 0)
        .auto_remove(true)
        .tty(true)
        .cmd(vec![])
        .build();

    match docker.containers().create(&container_opts).await {
        Ok(info) => {
            let id = info.id;
            let _ = docker.containers().get(&id).start().await;

            let response = SessionResponse {
                container_id: id.clone(),
                editor_url: format!("https://fdbe92cd247b.ngrok-free.app/{}", id), // Replace with reverse proxy logic
            };
            Ok(warp::reply::with_status(json(&response), StatusCode::OK))
        }
        Err(e) => {
            eprintln!("Docker start error: {:?}", e);
            Err(warp::reject())
        }
    }
}

pub async fn stop_instance(req: StopRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let docker = Docker::new();
    let container = docker.containers().get(&req.container_id);

    match container.stop(None).await {
        Ok(_) => {
            let _ = container.remove(RmContainerOptions::builder().force(true).build()).await;
            Ok(warp::reply::with_status("Stopped", StatusCode::OK))
        }
        Err(_) => Err(warp::reject()),
    }
}
