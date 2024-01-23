use bech32::{self, u5, ToBase32, Variant};
use bitcoin::consensus::encode::deserialize;
use bitcoin::hex::{DisplayHex, FromHex};
use bitcoin::script;
use bitcoin::{
    hashes::{ripemd160, sha256, sha256d, Hash},
    Address, PublicKey, ScriptBuf, Transaction, Witness,
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::*;
use serde_json::Value;
use std::str::FromStr;

use crate::constants::NETWORK;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vin {
    pub txid: String,
    pub vout: u32,
    pub prevout: Option<Vout>,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub is_coinbase: bool,
    pub sequence: u64,
    pub witness: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InputOutput {
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub data_hex: Option<String>,
}

fn get_string_from_json(value: &serde_json::Value, key: &str) -> Result<String, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("Key {} missing or not a string", key))
        .map(|s| s.to_string())
}

fn get_u64_from_json(value: &serde_json::Value, key: &str) -> Result<u64, String> {
    value[key]
        .as_u64()
        .ok_or_else(|| format!("Key {} missing or not a u64", key))
}

fn get_bool_from_json(value: &serde_json::Value, key: &str) -> Result<bool, String> {
    value[key]
        .as_bool()
        .ok_or_else(|| format!("Key {} missing or not a bool", key))
}

fn get_vec_from_json<T>(value: &Value, key: &str) -> Result<Vec<T>, String>
where
    T: DeserializeOwned,
{
    value[key]
        .as_array()
        .ok_or_else(|| format!("Key {} missing or not an array", key))?
        .iter()
        .map(|item| serde_json::from_value(item.clone()).map_err(|e| e.to_string()))
        .collect()
}

pub fn decode_tx(tx_json: &Value) -> InputOutput {
    let vin = tx_json["vin"].as_array().unwrap();
    let vout = tx_json["vout"].as_array().unwrap();
    let mut inputs: Vec<Vin> = Vec::new();
    for input in vin {
        let prevout = &input["prevout"];
        let inputxcp = Vin {
            txid: input["txid"].as_str().unwrap().to_string(),
            vout: input["vout"].as_u64().unwrap() as u32,
            prevout: Some(Vout {
                scriptpubkey: get_string_from_json(&prevout, "scriptpubkey").unwrap(),
                scriptpubkey_asm: get_string_from_json(&prevout, "scriptpubkey_asm").unwrap(),
                scriptpubkey_type: get_string_from_json(&prevout, "scriptpubkey_type").unwrap(),
                scriptpubkey_address: get_string_from_json(&prevout, "scriptpubkey_address").ok(),
                value: get_u64_from_json(&prevout, "value").unwrap(),
            }),
            scriptsig: get_string_from_json(input, "scriptsig").unwrap(),
            scriptsig_asm: get_string_from_json(input, "scriptsig_asm").unwrap(),
            is_coinbase: get_bool_from_json(input, "is_coinbase").unwrap(),
            sequence: get_u64_from_json(input, "sequence").unwrap(),
            witness: get_vec_from_json(input, "witness").ok(),
        };
        inputs.push(inputxcp);
    }
    let mut outputs: Vec<Vout> = Vec::new();
    for output in vout {
        let outputxcp = Vout {
            scriptpubkey: get_string_from_json(output, "scriptpubkey").unwrap(),
            scriptpubkey_address: get_string_from_json(output, "scriptpubkey_address").ok(),

            scriptpubkey_type: get_string_from_json(output, "scriptpubkey_type").unwrap(),
            scriptpubkey_asm: get_string_from_json(output, "scriptpubkey_asm").unwrap(),
            value: get_u64_from_json(output, "value").unwrap(),
        };
        outputs.push(outputxcp);
    }
    InputOutput {
        vin: inputs,
        vout: outputs,
        data_hex: None,
    }
}

pub async fn request_tx(tx_hash: &str) -> Option<InputOutput> {
    let url = format!("https://mempool.space/api/tx/{}", tx_hash);
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    if res.status() != 200 {
        return None;
    }
    let responsebody = res.text().await.unwrap();
    let body: Value = serde_json::from_str(&responsebody).unwrap();
    let output = decode_tx(&body);
    //printtln!("output: {:?}", output);
    Some(output)
}

pub fn get_address_from_ripedmd(ripedmd: &Vec<u8>) -> String {
    let hex_data = ripedmd.as_slice();
    //create checksum
    let sh2 = sha256d::Hash::hash(hex_data);
    let checksum = sh2.as_byte_array().get(0..4).unwrap();
    let mut b58 = hex_data.to_vec();
    //append checksum
    b58.extend_from_slice(checksum);
    let address = bs58::encode(b58).into_string();
    address.into()
}

pub fn get_address_from_script(scriptsig: &ScriptBuf, witness: Option<&Witness>) -> Option<String> {
    let script_pubk_type = script_pubkey_type(scriptsig, witness);
    //if p2pkh < 25 bytes is a ripe160 hash
    if script_pubk_type == "p2pkh" && scriptsig.len() <= 25 {
        let mut ripedmd = scriptsig.as_bytes().to_vec();
        //remove first 4 bytes Opcodes
        ripedmd.drain(0..3);
        //remove last 2 bytes Opcodes
        ripedmd.drain(ripedmd.len() - 2..ripedmd.len());
        ripedmd.insert(0, 0x00);
        let hex_data = ripedmd.as_slice();
        //create checksum
        let address = get_address_from_ripedmd(&hex_data.to_vec());
        return address.into();
    }
    //if p2pkh > 25 bytes is a scriptpubkey
    if script_pubk_type == "p2pkh" && scriptsig.len() >= 106 {
        let mut scriptsig_bytes = scriptsig.as_bytes().to_vec();
        let byte_length = scriptsig_bytes[0];
        //remove signature jus keep the pubkey
        scriptsig_bytes.drain(0..byte_length as usize + 2);
        let pubkeystr = scriptsig_bytes.as_hex().to_string();
        let pubkey = PublicKey::from_str(&pubkeystr).unwrap();
        let address = Address::p2pkh(&pubkey, NETWORK);
        return address.to_string().into();
    }
    if script_pubk_type == "p2sh-p2wpkh" {
        let mut script_sig = scriptsig.as_bytes().to_vec();
        //remove first byte Opcode
        script_sig.drain(0..1);
        // in a p2sh-p2wpkh the script_sig is a push of 22 bytes so to obtain the pubkey we need to remove the first 2 bytes
        // and make a sha256 of the remaining 20 bytes
        let sha2_of_script_sig = sha256::Hash::hash(&script_sig);
        // then we make a ripemd160 of the sha256 of the script_sig
        let mut ripemd160_of_sha2 = ripemd160::Hash::hash(&sha2_of_script_sig.to_byte_array())
            .to_byte_array()
            .to_vec();
        // we add the version byte to the ripemd160 of the sha256 of the script_sig that is 0x05 for mainnet
        ripemd160_of_sha2.insert(0, 0x05);
        let address = get_address_from_ripedmd(&ripemd160_of_sha2.to_vec());
        return address.into();
    }
    None
}

fn script_pubkey_type(script: &ScriptBuf, witness: Option<&Witness>) -> String {
    let script_bytes = script.as_bytes();
    //if contains witness data is a p2wpkh
    let witness_arr = match witness {
        Some(w) => w.to_vec(),
        None => Vec::new(),
    };
    if script.is_p2pkh() {
        return "p2pkh".to_string();
    }
    if script.is_p2sh() {
        return "p2sh".to_string();
    }
    if script.is_p2wpkh() {
        return "p2wpkh".to_string();
    }
    if script.is_multisig() {
        return "multisig".to_string();
    }
    //if witness is not empty and script is > 22 bytes is a p2sh-p2wpkh
    if !witness_arr.is_empty() && script_bytes.len() >= 22 {
        return "p2sh-p2wpkh".to_string();
    }
    //DER signature start with 0x30
    if script_bytes.get(1) == Some(&0x30) {
        return "p2pkh".to_string();
    }
    if script.is_op_return() {
        return "op_return".to_string();
    }
    if script.is_witness_program() {
        return "witness_program".to_string();
    }
    "unknown".to_string()
}

pub async fn deserialize_rawtx(rawtx: &String) -> Option<InputOutput> {
    let deserialized = deserialize::<Transaction>(&Vec::from_hex(rawtx).unwrap());
    match deserialized {
        Ok(d) => {
            let mut inputs = Vec::<Vin>::new();
            let mut outputs = Vec::<Vout>::new();
            d.input.iter().for_each(|i| {
                let witness = &i.witness;
                let addrs = get_address_from_script(&i.script_sig, witness.into());
                let prev_o = Some(Vout {
                    scriptpubkey: i.script_sig.to_hex_string(),
                    scriptpubkey_asm: i.script_sig.to_asm_string(),
                    scriptpubkey_type: script_pubkey_type(&i.script_sig, witness.into()),
                    scriptpubkey_address: addrs.clone(),
                    value: 0,
                });
                let inputxcp = Vin {
                    txid: i.previous_output.txid.to_string(),
                    vout: i.previous_output.vout,
                    prevout: prev_o,
                    scriptsig: i.script_sig.to_string(),
                    scriptsig_asm: i.script_sig.to_string(),
                    is_coinbase: d.is_coinbase(),
                    sequence: i.sequence.to_consensus_u32() as u64,
                    witness: None,
                };
                inputs.push(inputxcp);
            });
            d.output.iter().for_each(|o| {
                let outputxcp = Vout {
                    scriptpubkey: o.script_pubkey.to_hex_string().to_string(),
                    scriptpubkey_asm: o.script_pubkey.to_string(),
                    scriptpubkey_type: script_pubkey_type(&o.script_pubkey, None),
                    scriptpubkey_address: get_address_from_script(&o.script_pubkey, None),
                    value: o.value.to_sat(),
                };
                outputs.push(outputxcp);
            });
            Some(InputOutput {
                vin: inputs,
                vout: outputs,
                data_hex: None,
            })
        }
        Err(_e) => None,
    }
}
