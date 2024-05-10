use crate::core::ApiError;
use crate::receipt::ReceiptView;
use crate::receipt::{__path_create, __path_delete, __path_get_all, __path_get_one};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    components(schemas(ReceiptView, ApiError)),
    info(description = "Api for creating Receipts and the corresponding ReceiptsItems"),
    paths(get_all, get_one, create, delete)
)]
pub struct ApiDoc;
