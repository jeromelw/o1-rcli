use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
//rcli csv -i test.csv -d '|' -o output.json
use clap::Parser;
use rcli::get_content;
use rcli::get_reader;
use rcli::{
    process_chacha_generate, process_csv, process_decode, process_decrypt, process_encode,
    process_encrypt, process_generate, process_genpass, process_sign, process_verify,
    Base64SubCommand, ChachaSubCommand, Opts, SubCommand, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercace,
                opts.no_number,
                opts.no_symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let encode = process_sign(&mut reader, &key, opts.format)?;
                println!("Signature result: {}", encode);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("Signature verified");
                } else {
                    println!("Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                process_generate(opts.output, opts.format)?;
            }
        },
        SubCommand::Chacha(subcmd) => match subcmd {
            ChachaSubCommand::Encrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let (ret, nonce) = process_encrypt(&mut reader, &key, opts.format)?;
                let ret = URL_SAFE_NO_PAD.encode(ret);

                let nonce = URL_SAFE_NO_PAD.encode(nonce);
                println!("Encrypted: {}", ret);
                println!("Nonce: {}", nonce);
            }
            ChachaSubCommand::Decrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let nonce = URL_SAFE_NO_PAD.decode(&opts.nonce)?;
                let result = process_decrypt(&mut reader, &key, &nonce, opts.format)?;
                println!("Decrypted: {:?}", String::from_utf8(result)?)
            }
            ChachaSubCommand::Generate(opts) => {
                process_chacha_generate(opts.output, opts.format)?;
            }
        },
    }
    Ok(())
}
