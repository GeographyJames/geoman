use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validataor(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let _token = credentials.token();
    Ok(req)
}
