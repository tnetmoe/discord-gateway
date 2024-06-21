// SPDX-License-Identifier: AGPL-3.0-only
use prost_build::compile_protos;
use std::io::Result;

fn main() -> Result<()> {
    compile_protos(&["gateway.proto"], &["../../proto"])?;
    Ok(())
}
