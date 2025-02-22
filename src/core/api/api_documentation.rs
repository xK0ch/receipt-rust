use crate::core::ApiError;
use crate::receipt::ReceiptView;
use crate::receipt::{
    __path_create_receipt, __path_delete_receipt, __path_get_all_receipts, __path_get_one_receipt,
};
use crate::receipt_item::{ReceiptItemCreateOrder, ReceiptItemUpdateOrder, ReceiptItemView};
use crate::receipt_item::{
    __path_create_receipt_item, __path_delete_receipt_item,
    __path_get_all_receipt_items_by_receipt, __path_get_one_receipt_item,
    __path_update_receipt_item,
};
use utoipa::OpenApi;

#[rustfmt::skip]
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            ApiError,
            ReceiptItemCreateOrder,
            ReceiptItemUpdateOrder,
            ReceiptItemView,
            ReceiptView
        )
    ),
    info(description = "Api for creating Receipts and the corresponding ReceiptsItems"),
    paths(
        create_receipt,
        create_receipt_item,
        delete_receipt,
        delete_receipt_item,
        get_all_receipts,
        get_all_receipt_items_by_receipt,
        get_one_receipt,
        get_one_receipt_item,
        update_receipt_item
    )
)]
pub struct ApiDoc;
