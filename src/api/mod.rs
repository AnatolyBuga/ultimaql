pub mod routers;

use crate::marketdata::models::{Spot, YieldCurve, ImplVolCurve};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ultima QL"
    ),
    paths(
        routers::health_check
    ),
    components(
        schemas(Spot, YieldCurve, ImplVolCurve)
    ),
    tags(
        (name = "Ultima QL", description = "Ultimate Business Intellegence endpoints.")
    ),
)]
pub(crate) struct ApiDoc;