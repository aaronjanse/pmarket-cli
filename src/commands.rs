use super::{api, errors};
use dialoguer::{Confirm, Input};
use prettytable::format;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};

pub fn show_event_list() {
    let events = api::get_events();

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("ID").style_spec("bFc"),
        Cell::new("Title").style_spec("bFc"),
    ]));

    for event in events {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", event.id)),
            Cell::new(&event.title),
        ]));
    }
    table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(1, 1)
            .build(),
    );
    table.printstd();
}

pub fn show_event_info(event_id: u32) {
    let event = api::get_event_info(event_id);

    println!("\x1b[2mTitle:\x1b[m   \x1b[93m{}\x1b[m", event.title);

    use chrono::prelude::*;
    println!(
        "\x1b[2mCreated:\x1b[m {}  \
         \x1b[2mOpens:\x1b[m {}  \
         \x1b[2mCloses:\x1b[m {} ",
        event.created.format("%Y-%m-%d"),
        event.opens.format("%Y-%m-%d"),
        event.closes.format("%Y-%m-%d"),
    );
    println!();

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("ID").style_spec("bFc"),
        Cell::new("Price").style_spec("bFc"),
        Cell::new("Name").style_spec("bFc"),
    ]));

    for stock in event.stocks {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", stock.id)),
            Cell::new(&format!("{}¢", stock.price)).style_spec("r"),
            Cell::new(&stock.title),
        ]));
    }

    table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(1, 0)
            .build(),
    );
    table.printstd();
}

pub fn show_stock_info(stock_id: u32) -> std::result::Result<(), errors::ClientError> {
    let stock = api::get_stock_info(stock_id);
    let event = api::get_event_info(stock.event_id);

    println!("\x1b[mEvent Title:\x1b[m  \x1b[93m{}\x1b[m", event.title);
    println!("\x1b[mStock Title:\x1b[m  \x1b[93m{}\x1b[m", stock.title);
    println!();

    let mut asks_table = Table::new();
    asks_table.set_titles(Row::new(vec![
        Cell::new("Offers to sell:").style_spec("bFc"),
    ]));
    for bin in &stock.asks {
        asks_table.add_row(Row::new(vec![
            Cell::new(format!("{}x {}¢", bin.count, bin.price).as_str()).style_spec("l"),
        ]));
    } 
    asks_table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 0)
            .build(),
    );
    asks_table.printstd();

    let mut bids_table = Table::new();
    bids_table.set_titles(Row::new(vec![
        Cell::new("Offers to buy:").style_spec("bFc"),
    ]));
    for bin in &stock.bids {
        bids_table.add_row(Row::new(vec![
            Cell::new(format!("{}x {}¢", bin.count, bin.price).as_str()).style_spec("l"),
        ]));
    }
    bids_table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 0)
            .build(),
    );
    bids_table.printstd();

    Ok(())
}

pub fn prompt_buy(stock_id: u32) -> Result<(), errors::ClientError> {
    println!(
        "\x1b[mYour Balance:\x1b[m \x1b[92m{}\x1b[m",
        api::get_user_balance()
    );

    let stock = api::get_stock_info(stock_id);
    let event = api::get_event_info(stock.event_id);

    println!("\x1b[mEvent Title:\x1b[m  \x1b[93m{}\x1b[m", event.title);
    println!("\x1b[mStock Title:\x1b[m  \x1b[93m{}\x1b[m", stock.title);
    println!();

    let confirm_stock = Confirm::new()
        .with_prompt("Is this the correct stock?")
        .interact()
        .unwrap();
    if !confirm_stock {
        println!("Purchase cancelled.");
        return Ok(());
    }

    println!();

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("Offers").style_spec("bFc"),
        Cell::new("Price").style_spec("bFc"),
    ]));

    for bin in &stock.asks {
        table.add_row(Row::new(vec![
            Cell::new(format!("{}", bin.count).as_str()).style_spec("c"),
            Cell::new(format!("{}¢", bin.count).as_str()).style_spec("c"),
        ]));
    }

    if stock.asks.len() == 0 {
        println!("You're the first person to bid.");
    }

    table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 0)
            .build(),
    );

    table.printstd();

    println!();

    let price_str: String = Input::new()
        .with_prompt("Max price you'll buy at?")
        .validate_with(|input: &str| -> Result<(), &str> {
            let valid = match input.parse::<i32>() {
                Ok(amount) => 0 < amount && amount < 100,
                Err(_) => false,
            };
            if valid {
                Ok(())
            } else {
                Err("Must be a number 0 < x < 100")
            }
        })
        .interact()
        .unwrap();
    let price = price_str.parse::<u8>().unwrap();

    let count_str: String = Input::new()
        .with_prompt("How many?")
        .validate_with(|input: &str| -> Result<(), &str> {
            let valid = match input.parse::<i32>() {
                Ok(amount) => 0 < amount,
                Err(_) => false,
            };
            if valid {
                Ok(())
            } else {
                Err("Must be a number 0 < x")
            }
        })
        .interact()
        .unwrap();
    let count = count_str.parse::<u32>().unwrap();

    api::buy(stock_id, price, count);

    Ok(())
}
