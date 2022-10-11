use crate::lightning;
use hex::FromHex;
use rocket::serde::{json::Json, Serialize};
use rocket::*;
use rocket_dyn_templates::{context, Template};
use tonic_openssl_lnd::lnrpc::invoice::InvoiceState;

#[derive(Serialize, Default)]
pub struct InvoiceResponse {
    payment_request: String,
    hash: String,
    paid: bool,
    preimage: String,
    description: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/create_invoice/<description>/<amount>")]
pub async fn create_invoice(description: &str, amount: u32) -> Json<InvoiceResponse> {
    let invoice = lightning::create_invoice(description, amount)
        .await
        .unwrap();

    let hash_str = invoice
        .r_hash
        .iter()
        .map(|h| format!("{h:02x}"))
        .collect::<Vec<String>>()
        .join("");

    Json(InvoiceResponse {
        payment_request: invoice.payment_request,
        hash: hash_str,
        ..Default::default()
    })
}

#[get("/invoice/<hash>")]
pub async fn lookup_invoice(hash: &str) -> Json<InvoiceResponse> {
    let hash = <[u8; 32]>::from_hex(hash).expect("Decoding failed");
    let invoice = lightning::get_invoice(&hash).await.unwrap();
    let mut preimage = String::new();
    let mut paid = false;

    if let Some(state) = InvoiceState::from_i32(invoice.state) {
        if state == InvoiceState::Settled {
            paid = true;
            preimage = invoice
                .r_preimage
                .iter()
                .map(|h| format!("{h:02x}"))
                .collect::<Vec<String>>()
                .join("");
        }
    }
    Json(InvoiceResponse {
        paid,
        preimage,
        description: invoice.memo,
        ..Default::default()
    })
}
