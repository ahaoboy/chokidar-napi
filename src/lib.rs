#![deny(clippy::all)]
use chokidar::{chokidar, Args};
use clap::Parser;
use napi_derive::napi;

#[napi]
pub fn chokidar_start(v: Vec<String>) {
  let args = Args::parse_from(v);
  chokidar(args)
}
