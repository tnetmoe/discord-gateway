// SPDX-License-Identifier: AGPL-3.0-only
use std::io::Result;
use prost_build::compile_protos;

fn main() -> Result<()> {
    compile_protos(
        &["gateway.proto"],
        &["../../proto"]
    )?;
    Ok(())
}