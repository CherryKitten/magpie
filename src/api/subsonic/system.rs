use super::*;

pub(crate) async fn ping() -> Json<Response> {
    let mut res = SubsonicResponse::new();
    res.status = SubsonicStatus::Ok;

    Json(Response::SubsonicResponse(res))
}

pub(crate) async fn get_license() -> Json<Response> {
    let mut res = SubsonicResponse::new();
    res.license = Some(SubsonicLicense::new());

    Json(Response::SubsonicResponse(res))
}
