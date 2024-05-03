use crate::receipt::model::ReceiptView;
use crate::receipt::Receipt;

pub fn to_view(receipt: Receipt) -> ReceiptView {
    ReceiptView {
        id: receipt.id,
        sum: receipt.sum,
    }
}
