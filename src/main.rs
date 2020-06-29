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

mod errors;
mod api;
mod commands;

fn main() {
  let matches = App::new("pm")
    .about("Prediction Market CLI")
    .version("0.0.1")
    .author("Aaron Janse")
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
      App::new("event").about("View details about an event").arg(
        Arg::with_name("id")
          .takes_value(true)
          .required(true)
          .about("Event ID"),
      ),
    )
    .subcommand(
      App::new("stock")
        .about("View details about a stock")
        .arg(Arg::with_name("id").takes_value(true).about("Stock ID")),
    )
    .subcommand(
      App::new("buy")
        .about("Buy shares of a stock")
        .arg(Arg::with_name("id").takes_value(true).about("Stock ID")),
    )
    .subcommand(
      App::new("sell")
        .about("Sell shares of a stock")
        .arg(Arg::with_name("id").takes_value(true).about("Stock ID")),
    )
    .get_matches();

  let needs_auth = match matches.subcommand_name() {
    Some("buy") | Some("sell") | Some("portfolio") | Some("admin") => true,
    _ => false,
  };

  let mut token: String = "".to_string();
  let token_file = "xx";
  if needs_auth {
    // token = fs::read_to_string(token_file).unwrap().trim().to_string();
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
    Some("list") => commands::show_event_list(),
    Some("event") => {
      let event_subcommand = matches.subcommand().1.unwrap();
      let event_id_str = event_subcommand.value_of("id").unwrap();
      let event_id = match event_id_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => {
          println!("Invalid event ID.");
          return;
        }
      };
      commands::show_event_info(event_id);
    }
    Some("stock") => {
      let stock_subcommand = matches.subcommand().1.unwrap();
      let stock_id_str = stock_subcommand.value_of("id").unwrap();
      let stock_id = match stock_id_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => {
          println!("Invalid stock ID.");
          return;
        }
      };
      commands::show_stock_info(stock_id);
    }
    Some("buy") => {
      let buy_subcommand = matches.subcommand().1.unwrap();
      let stock_id_str = buy_subcommand.value_of("id").unwrap();
      let stock_id = match stock_id_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => {
          println!("Invalid stock ID.");
          return;
        }
      };
      commands::prompt_buy(stock_id);
    }
    None => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}
