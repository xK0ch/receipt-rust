use crate::receipt::model::ReceiptView;
use crate::receipt::Receipt;

impl From<Receipt> for ReceiptView {
    fn from(receipt: Receipt) -> Self {
        ReceiptView {
            id: receipt.id,
            sum: receipt.sum,
        }
    }
}
