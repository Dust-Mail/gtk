use std::{io::Read, rc::Rc};

use crate::error::Result;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).map_err(Rc::new)?;

        if count == 0 {
            break;
        }

        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn sha256_hex<R: Read>(reader: R) -> Result<String> {
    let digest = sha256_digest(reader)?;

    let hex = HEXUPPER.encode(digest.as_ref());

    Ok(hex)
}
