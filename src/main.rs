extern crate dialoguer;
extern crate prettytable;
use clap::{App, Arg};
use dialoguer::{Confirm, Input};
use prettytable::format;
use prettytable::{Cell, Row, Table};
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
  let matches = App::new("pm")
    .about("Prediction Market CLI")
    .version("0.0.1")
    .author("Aaron Janse")
    .arg(
      Arg::with_name("token-file")
        .long("token-file")
        .global(true)
        .default_value("/tmp/pmarket-token.txt"),
    )
    .subcommand(
      App::new("signup")
        .arg(
          Arg::with_name("username")
            .long("username")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("password")
            .long("password")
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      App::new("signin")
        .arg(
          Arg::with_name("username")
            .long("username")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("password")
            .long("password")
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      App::new("admin")
        .about("Tools for admins")
        .subcommand(
          App::new("create-stock")
            .about("Create a stock")
            .arg(
              Arg::with_name("event-id")
                .long("event-id")
                .takes_value(true)
                .required(true)
                .about("Event ID"),
            )
            .arg(
              Arg::with_name("title")
                .long("title")
                .takes_value(true)
                .required(true)
                .about("Stock title"),
            ),
        )
        .subcommand(
          App::new("create-event")
            .about("Create a event")
            .arg(
              Arg::with_name("title")
                .long("title")
                .takes_value(true)
                .required(true)
                .about("Event title"),
            )
            .arg(
              Arg::with_name("description")
                .long("description")
                .takes_value(true)
                .required(true)
                .about("Event description"),
            )
            .arg(
              Arg::with_name("opens")
                .long("opens")
                .takes_value(true)
                .required(true)
                .about("Seconds since Unix epoch"),
            )
            .arg(
              Arg::with_name("closes")
                .long("closes")
                .takes_value(true)
                .required(true)
                .about("Seconds since Unix epoch"),
            ),
        ),
    )
    .subcommand(
      App::new("list")
        .alias("ls")
        .about("List events")
        .after_help("By default, tradable events are shown."),
    )
    .subcommand(
      App::new("portfolio")
        .about("Your current shares")
        .after_help("By default, tradable events are shown."),
    )
    .subcommand(
      App::new("event")
        .about("View details about an event")
        .arg(Arg::with_name("id").takes_value(true).about("Event ID")),
    )
    .subcommand(
      App::new("stock")
        .about("View details about a stock")
        .arg(Arg::with_name("id").takes_value(true).about("Stock ID")),
    )
    .subcommand(App::new("buy").about("Buy shares of a stock"))
    .subcommand(App::new("sell").about("Sell shares of a stock"))
    .get_matches();

  println!("{:?}", matches.subcommand_name());
  let needs_auth = match matches.subcommand_name() {
    Some("buy") | Some("sell") | Some("portfolio") | Some("admin") => true,
    _ => false,
  };

  let mut token: String = "".to_string();
  let token_file = matches.value_of("token-file").unwrap();
  if needs_auth {
    token = fs::read_to_string(token_file).unwrap().trim().to_string();
  }
  let tmp_tok_val_str = format!("SESSION-TOKEN={}", token);
  let token_cookie_value = HeaderValue::from_str(&tmp_tok_val_str).unwrap();

  let client = reqwest::Client::new();

  match matches.subcommand_name() {
    Some("signup") => {
      let (_, signup_res) = matches.subcommand();
      let signup = signup_res.unwrap();

      let mut req_map = HashMap::new();
      req_map.insert("username", signup.value_of("username").unwrap());
      req_map.insert("password", signup.value_of("password").unwrap());

      println!("SENDING");
      let res = client
        .post("http://localhost:8080/auth/signup")
        .json(&req_map)
        .send()
        .unwrap();
      println!("DONE SENDING");
    }
    Some("signin") => {
      let (_, signup_res) = matches.subcommand();
      let signup = signup_res.unwrap();

      let mut req_map = HashMap::new();
      req_map.insert("username", signup.value_of("username").unwrap());
      req_map.insert("password", signup.value_of("password").unwrap());

      println!("SENDING");
      let res = client
        .post("http://localhost:8080/auth/signin")
        .json(&req_map)
        .send()
        .unwrap();
      println!("DONE SENDING");
      let got_token: String = res
        .cookies()
        .filter(|x| x.name() == "SESSION-TOKEN")
        .next()
        .unwrap()
        .value()
        .to_string();
      let mut file = fs::File::create(token_file).unwrap();
      file.write_all(got_token.as_bytes());
      println!("TOK : {:?}", got_token);
    }
    Some("list") => {
      // [{"id":1,"title":"USA 2020 Presedential Election","description":"Who will win?","opens":"2021-01-01T00:00:00Z","closes":"2021-01-01T00:00:00Z","stocks":[{"id":1,"event_id":null,"title":"Trump","price":null},{"id":2,"event_id":null,"title":"Biden","price":null},{"id":3,"event_id":null,"title":"Other","price":null}]}]
      #[derive(Deserialize, Debug)]
      struct Event {
        id: u32,
        title: String,
      }

      let events: Vec<Event> = match client.get("http://localhost:8080/event").send() {
        Ok(x) => x,
        Err(err) => {
          println!("{}", err);
          return;
        }
      }
      .json()
      .unwrap();
      let mut table = Table::new();
      table.set_titles(Row::new(vec![
        Cell::new("ID").style_spec("bFc"),
        // Cell::new("State").style_spec("bFc"),
        // Cell::new("Market Cap").style_spec("brFc"),
        Cell::new("Title").style_spec("bFc"),
      ]));

      for event in events {
        table.add_row(Row::new(vec![
          Cell::new(&format!("{}", event.id)),
          // Cell::new("Trading").style_spec("Fg"),
          // Cell::new("$1,229,102").style_spec("r"),
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
    Some("event") => {
      println!(
        "\x1b[2mTitle:\x1b[m   \x1b[93m{}\x1b[m",
        "Who will win the 2020 USA Presedential Election?"
      );
      println!(
        "\x1b[2mCreated:\x1b[m {}  \
         \x1b[2mOpens:\x1b[m {}  \
         \x1b[2mCloses:\x1b[m {} ",
        "2019-01-01", "2019-10-01", "2020-11-23"
      );
      println!();

      let mut table = Table::new();
      table.add_row(Row::new(vec![
        Cell::new("0"),
        Cell::new("57¢").style_spec("r"),
        Cell::new("Joe Biden"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("1"),
        Cell::new("43¢").style_spec("r"),
        Cell::new("Donald Trump"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("2"),
        Cell::new("1¢").style_spec("r"),
        Cell::new("Bernie Sanders"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("3"),
        Cell::new("1¢").style_spec("r"),
        Cell::new("Other"),
      ]));

      table.set_titles(Row::new(vec![
        Cell::new("ID").style_spec("bFc"),
        Cell::new("Price").style_spec("bFc"),
        Cell::new("Name").style_spec("bFc"),
      ]));
      table.set_format(
        format::FormatBuilder::new()
          .column_separator(' ')
          .padding(1, 0)
          .build(),
      );

      table.printstd();
    }
    Some("buy") => {
      println!("\x1b[mYour Balance:\x1b[m \x1b[92m$1,317\x1b[m");
      println!("\x1b[mEvent Title:\x1b[m  \x1b[93m2020 USA Presedential Election\x1b[m");
      println!("\x1b[mStock Title:\x1b[m  \x1b[93mJoe Biden\x1b[m");
      println!();

      let confirm_stock = Confirm::new()
        .with_prompt("Is this the correct stock?")
        .interact()
        .unwrap();
      if !confirm_stock {
        println!("Purchase cancelled.");
        return;
      }

      println!();

      let mut table = Table::new();

      table.set_titles(Row::new(vec![
        Cell::new("Offers").style_spec("bFc"),
        Cell::new("Price").style_spec("bFc"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("173").style_spec("c"),
        Cell::new("53¢").style_spec("c"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("120").style_spec("c"),
        Cell::new("54¢").style_spec("c"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("97").style_spec("c"),
        Cell::new("55¢").style_spec("c"),
      ]));
      table.add_row(Row::new(vec![
        Cell::new("50").style_spec("c"),
        Cell::new("56¢").style_spec("c"),
      ]));
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
      let price = price_str.parse::<i32>();

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
      let count = count_str.parse::<i32>();
    }
    Some("admin") => {
      let (_, admin_res) = matches.subcommand();
      let admin = admin_res.unwrap();

      use chrono::prelude::*;
      use chrono::{DateTime, TimeZone, Utc};
      match admin.subcommand_name() {
        Some("create-event") => {
          let (_, create_event_res) = admin.subcommand();
          let create_event = create_event_res.unwrap();

          println!("{:?}", create_event.value_of("opens"));
          let opens_sec = create_event
            .value_of("opens")
            .unwrap()
            .parse::<i64>()
            .unwrap();
          let opens_time = Utc.timestamp(opens_sec, 0);
          let opens = format!("{:?}", opens_time);

          let closes_sec = create_event
            .value_of("closes")
            .unwrap()
            .parse::<i64>()
            .unwrap();
          let closes_time = Utc.timestamp(closes_sec, 0);
          let closes = format!("{:?}", closes_time);

          let mut req_map = HashMap::new();
          req_map.insert("title", create_event.value_of("title").unwrap());
          req_map.insert("description", create_event.value_of("description").unwrap());
          req_map.insert("opens", &opens);
          req_map.insert("closes", &closes);

          let res = client
            .post("http://localhost:8080/admin/event/create")
            .json(&req_map)
            .header(
              HeaderName::from_lowercase(b"cookie").unwrap(),
              token_cookie_value,
            )
            .send()
            .unwrap();
        }
        Some("create-stock") => {
          let (_, create_stock_res) = admin.subcommand();
          let create_stock = create_stock_res.unwrap();

          #[derive(Serialize)]
          struct JSON {
            title: String,
            event_id: u32,
          }

          let res = client
            .post("http://localhost:8080/admin/stock/create")
            .json(&JSON {
              title: create_stock.value_of("title").unwrap().to_string(),
              event_id: create_stock
                .value_of("event-id")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            })
            .header(
              HeaderName::from_lowercase(b"cookie").unwrap(),
              token_cookie_value,
            )
            .send()
            .unwrap();
        }
        None => println!("No admin subcommand was used"),
        _ => unreachable!(),
      }
    }
    None => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}
