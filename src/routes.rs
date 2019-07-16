#![allow(non_snake_case)]

use libra_client::client_proxy::ClientProxy;
use libra_wallet::key_factory::{ChildNumber, ExtendedPrivKey, KeyFactory, Seed};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::result::Result;

#[get("/")]
pub fn index() -> Template {
    let addr = "9a25ef3e96884708a1197f098981172a791857af2d1260fc2be5bce68e642cc1";

    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        context.insert("error".to_string(), get_proxy_result.err().unwrap());
        return Template::render("err", &context);
    }

    let mut client_proxy = get_proxy_result.unwrap();
    let params = ["query", addr];
    let balance = client_proxy.get_balance(&params).unwrap();
    let seq = client_proxy.get_sequence_number(&params).unwrap();
    context.insert("balance".to_string(), balance);
    context.insert("seq".to_string(), seq.to_string());
    context.insert("addr".to_string(), addr.to_string());
    Template::render("index", &context)
}

#[get("/balance")]
pub fn balance() -> Template {
    index()
}

#[get("/events")]
pub fn events() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("events", context)
}

#[derive(FromForm)]
pub struct TransferForm {
    transferTo: String,
    numberOfCoins: String,
    gas_unit_price: String,
    max_gas_amount: String,
}

#[get("/transfer")]
pub fn transfer() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("transfer", context)
}

#[post("/transfer", data = "<data>")]
pub fn transfer_libra(data: Form<TransferForm>) -> Template {
    let addr = "9a25ef3e96884708a1197f098981172a791857af2d1260fc2be5bce68e642cc1";

    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        context.insert("error".to_string(), get_proxy_result.err().unwrap());
        return Template::render("err", &context);
    }

    let mut client_proxy = get_proxy_result.unwrap();
    let params = [
        "query",
        "0",
        data.transferTo.as_str(),
        data.numberOfCoins.as_str(),
        data.gas_unit_price.as_str(),
        data.max_gas_amount.as_str(),
    ];
    // let resp = client_proxy.transfer_coins(&params, true);

    let resp = client_proxy
        .transfer_coins(&params, true)
        .map_err(|e| format!("{}", e));
    match resp {
        Ok(_) => context.insert("msg".to_string(), addr.to_string()),
        Err(e) => context.insert("msg".to_string(), e),
    };

    Template::render("transfer", &context)
}

#[get("/mint")]
pub fn mint() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("mint", context)
}

#[derive(FromForm)]
pub struct MintForm {
    transferTo: String,
    numberOfCoins: String,
}

#[post("/mint", data = "<data>")]
pub fn mint_libra(data: Form<MintForm>) -> Redirect {
    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        context.insert("msg".to_string(), get_proxy_result.err().unwrap());
        return Redirect::to("/balance");
    }
    let mut client_proxy = get_proxy_result.unwrap();

    let params = [
        "mint",
        data.transferTo.as_str(),
        data.numberOfCoins.as_str(),
    ];

    let resp = client_proxy
        .mint_coins(&params, true)
        .map_err(|e| format!("{}", e));

    if resp.is_err() {
        context.insert("msg".to_string(), resp.err().unwrap());
    }

    Redirect::to("/balance")
}

fn get_client_proxy() -> Result<ClientProxy, String> {
    let client_proxy = ClientProxy::new(
        "ac.testnet.libra.org",
        "80",
        "trusted_peers.config.toml",
        "",
        false,
        None,
        None,
    )
    .map_err(|e| format!("{}", e))?;

    Ok(client_proxy)
}

// // get the first child
// fn get_address(m: &Mnemonic) -> ExtendedPrivKey {
//     let seed = Seed::new(m, "salt");
//     let key_factory = KeyFactory::new(&seed).unwrap();

//     let child_private_0 = key_factory.private_child(ChildNumber::new(0)).unwrap();
//     child_private_0
// }
