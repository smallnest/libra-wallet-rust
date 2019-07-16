#![allow(non_snake_case)]

use libra_client::client_proxy::ClientProxy;
use log::error;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::Path;
use std::result::Result;

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        let e = get_proxy_result.err().unwrap();
        error!("failed to get client: {}", &e);
        context.insert("error".to_string(), e);
        return Template::render("err", &context);
    }

    let mut client_proxy = get_proxy_result.unwrap();
    let addr = client_proxy.accounts.get(0).unwrap().address.to_string();
    let params = ["query", addr.as_str()];
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
    let mut context = HashMap::<String, String>::new();
    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        let e = get_proxy_result.err().unwrap();
        error!("failed to get client: {}", &e);
        context.insert("error".to_string(), e);
        return Template::render("err", &context);
    }

    let client_proxy = get_proxy_result.unwrap();
    let addr = client_proxy.accounts.get(0).unwrap().address.to_string();
    context.insert("addr".to_string(), addr.to_string());
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
    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        let e = get_proxy_result.err().unwrap();
        error!("failed to get client: {}", &e);
        context.insert("error".to_string(), e);
        return Template::render("err", &context);
    }

    let mut client_proxy = get_proxy_result.unwrap();

    let params = ["state", "0"];
    let resp = client_proxy.get_latest_account_state(&params);
    if resp.is_err() {
        let e = resp.err().unwrap();
        error!("failed to get last state: {}", &e);
    }

    let params = [
        "query",
        "0",
        data.transferTo.as_str(),
        data.numberOfCoins.as_str(),
        data.gas_unit_price.as_str(),
        data.max_gas_amount.as_str(),
    ];

    let resp = client_proxy
        .transfer_coins(&params, true)
        .map_err(|e| format!("{}", e));
    match resp {
        Ok(_) => context.insert("msg".to_string(), "transfer succeeded".to_string()),
        Err(e) => {
            error!("failed to transfer: {}", &e);
            context.insert("msg".to_string(), e)
        }
    };

    Template::render("transfer", &context)
}

#[get("/mint")]
pub fn mint() -> Template {
    let mut context = HashMap::<String, String>::new();

    let get_proxy_result = get_client_proxy();
    if get_proxy_result.is_err() {
        let e = get_proxy_result.err().unwrap();
        error!("failed to get client: {}", &e);
        context.insert("msg".to_string(), e);
        return Template::render("mint", context);
    }
    let client_proxy = get_proxy_result.unwrap();
    let addr = client_proxy.accounts.get(0).unwrap().address.to_string();

    context.insert("addr".to_string(), addr.to_string());
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
        let e = get_proxy_result.err().unwrap();
        error!("failed to get client: {}", &e);
        context.insert("msg".to_string(), e);
        return Redirect::to("/balance");
    }
    let mut client_proxy = get_proxy_result.unwrap();

    let params = [
        "mint",
        data.transferTo.as_str(),
        data.numberOfCoins.as_str(),
    ];

    let get_proxy_result = client_proxy
        .mint_coins(&params, true)
        .map_err(|e| format!("{}", e));

    if get_proxy_result.is_err() {
        let e = get_proxy_result.err().unwrap();
        error!("failed to mint: {}", &e);
        context.insert("msg".to_string(), e);
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
        Some(String::from("./.wallet")),
    )
    .map_err(|e| format!("{}", e))?;

    if client_proxy.accounts.len() == 0 {
        let mut p = client_proxy;
        let resp = p.create_next_account(true); //recover
        if resp.is_err() {
            let e = resp.err().unwrap();
            error!("failed to create account: {}", &e);
            return Err(format!("{}", e));
        }
        if !Path::new("/etc/hosts").exists() {
            //initial libra
            let addr = p.accounts.get(0).unwrap().address.to_string();
            let params = ["mint", addr.as_str(), "100000"];
            let resp = p.mint_coins(&params, true);
            if resp.is_err() {
                let e = resp.err().unwrap();
                error!("failed to mint initial coin: {}", &e);
                return Err(format!("{}", e));
            }
        }

        return Ok(p);
    }

    Ok(client_proxy)
}
