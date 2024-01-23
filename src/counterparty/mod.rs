/* strictly based on https://github.com/Jpja/Electrum-Counterparty/blob/master/js/xcp/assets.js#L8
thanks to juan and JPJA for the help */

extern crate bs58;

use crate::bitcoin_utils::InputOutput;

use crate::constants::*;
use bech32::{self, u5, ToBase32, Variant};
use bitcoin::hashes::{sha256d, Hash};
use bitcoin::hex::{DisplayHex, FromHex};
use rc4::{consts::*, KeyInit, StreamCipher};
use rc4::{Key as RC4Key, Rc4};
use serde::*;

pub mod decode;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnchanceSend {
    asset_id: i64,
    asset_name: String,
    quantity: i64,
    recipient: String,
    memo: String,
    source: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sweep {
    destination: String,
    source: String,
    flag: u8,
    memo: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CounterPartyTransaction {
    pub transaction: InputOutput,
}

pub enum CounterPartyMessage {
    Sweep(Sweep),
    EnchanceSend(EnchanceSend),
}

impl CounterPartyTransaction {
    fn get_data_hex_from_op_return(&self, script: &[u8]) -> Vec<u8> {
        let data_hex = script[2..].to_vec();
        data_hex
    }

    fn is_counterparty(&self, data: &[u8]) -> bool {
        let prefix = data.get(0..8).unwrap();
        prefix == PREFIX
    }

    pub fn decode_op_return(&self) -> Option<Vec<u8>> {
        for vout in &self.transaction.vout {
            if vout.scriptpubkey_type == "op_return" {
                let first_input = &self.transaction.vin[0];
                //get the key for decode the data if it is an enhanced send, the key is the txid of the first input
                let prev_hash = match Vec::from_hex(&first_input.txid) {
                    Ok(d) => d,
                    Err(_e) => return None,
                };
                //init the rc4 key
                let rc4 = RC4Key::<U32>::from_slice(&prev_hash);
                //init the rc4 cipher with the key
                let mut cipher = Rc4::new(rc4);
                //get the data from the scriptpubkey asm.
                let script_vec = Vec::from_hex(&vout.scriptpubkey).unwrap();
                let mut data_hex = self.get_data_hex_from_op_return(&script_vec);
                //printtln!("data_hex: {:?}", data_hex.as_hex().to_string());
                let data_slice = data_hex.as_mut_slice();
                cipher.apply_keystream(data_slice);
                match self.is_counterparty(data_slice) {
                    false => {
                        println!("Not a counterparty message");
                        return None;
                    }
                    true => return Some(data_slice.to_vec()),
                }
            }
            if vout.scriptpubkey_type == "multisig" {
                //get the data from the scriptpubkey asm, because is a old counterparty message
                let mut scrip_bytes = Vec::from_hex(vout.scriptpubkey.as_str()).unwrap();
                let length = scrip_bytes[1];
                scrip_bytes.drain(0..length as usize + 4);
                scrip_bytes.drain(scrip_bytes.len() - 2..scrip_bytes.len());
                match self.is_counterparty(&scrip_bytes) {
                    false => {
                        println!("Not a counterparty message");
                        return None;
                    }
                    true => return scrip_bytes.into(),
                }
            }
        }
        None
    }

    fn get_asset_name(&self, id: u64) -> String {
        if id == 0 {
            return "BTC".to_string();
        } else if id == 1 {
            return "XCP".to_string();
        } else if id >= 95428956661682177 && id < 18446744073709551615 {
            return "A".to_string() + &id.to_string();
        }
        let b26_digits = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut n = id;
        let mut name = String::new();

        while n > 0 {
            let mod_ = n % 26;
            name.insert(0, b26_digits.chars().nth(mod_ as usize).unwrap());
            n -= mod_;
            n /= 26;
        }

        name
    }
    fn hex2aq(&self, hex: &str) -> Result<String, std::num::ParseIntError> {
        let mut result = String::new();

        // Iterar de dos en dos caracteres por la cadena hexadecimal
        for i in (0..hex.len()).step_by(2) {
            // Extraer un par de caracteres y convertirlos a un número
            let part = &hex[i..i + 2];
            let int = u8::from_str_radix(part, 16)?;

            // Verificar si el número está en el rango de caracteres ASCII imprimibles
            if (32..=126).contains(&int) {
                result.push(int as char);
            } else {
                result.push('?');
            }
        }

        Ok(result)
    }
    fn hex_to_address(&self, hex_data: &[u8]) -> Option<String> {
        let version_byte = hex_data.get(0..1).unwrap();
        if version_byte == BASE58_VERSION_BYTE || version_byte == BASE58_P2SH_VERSION_BYTE {
            //this show a error in rust-analyzer but it works
            let sh2 = sha256d::Hash::hash(hex_data);
            let mut address = hex_data.to_vec();
            let checksum = sh2.as_byte_array().get(0..4).unwrap();
            address.extend_from_slice(checksum);
            let address = bs58::encode(address).into_string();
            return Some(address);
        } else if version_byte == BECH_32_VERSION_BYTE {
            let hrp = "bc";
            //remove the first byte of the data, this is the version byte
            let data = hex_data.get(1..).unwrap().to_vec();
            //convert the data to base32
            let mut data_base = data.to_base32();
            //add the witness version byte to the data
            data_base.insert(0, u5::try_from_u8(0).unwrap());
            let address = bech32::encode(hrp, &data_base, Variant::Bech32).unwrap();
            return Some(address);
        }
        None
    }

    fn get_source(&self) -> Option<String> {
        let first_input = &self.transaction.vin[0];
        let source = first_input
            .prevout
            .as_ref()
            .unwrap()
            .scriptpubkey_address
            .as_ref();
        match source {
            Some(s) => return Some(s.to_string()),
            None => return None,
        }
    }

    fn decode_enchanced_send(&self, hex_data: &[u8]) -> Option<EnchanceSend> {
        //from the 1st to the 8th byte is the asset
        let asset_hex = hex_data.get(1..9).unwrap();
        let asset_name = self.get_asset_name(
            u64::from_str_radix(asset_hex.as_hex().to_string().as_str(), 16).unwrap(),
        );
        //from the 9th to the 16th byte is the quantity
        let quantity_hex = hex_data.get(9..17).unwrap();
        //from the 17th to the 24th byte is the recipient
        let recipient_hex = hex_data.get(17..38).unwrap_or("uknown".as_bytes());
        //from the 24th byte to the end is the memo
        let memo_hex = hex_data.get(38..).unwrap_or("".as_bytes());
        let recipient = self.hex_to_address(recipient_hex);
        let quantity_q = i64::from_str_radix(quantity_hex.as_hex().to_string().as_str(), 16);
        let asset_id = i64::from_str_radix(asset_hex.as_hex().to_string().as_str(), 16);
        let memo = self.hex2aq(memo_hex.as_hex().to_string().as_str());
        Some(EnchanceSend {
            asset_id: asset_id.unwrap(),
            asset_name: asset_name,
            quantity: quantity_q.unwrap(),
            recipient: recipient.unwrap_or("".to_string()),
            memo: memo.unwrap_or("".to_string()),
            source: self.get_source().unwrap(),
        })
    }

    fn decdode_sweep(&self, hex_data: &[u8]) -> Option<Sweep> {
        let recipient_hex = hex_data.get(1..22).unwrap();
        let flag_hex = hex_data.get(22..23).unwrap();
        let memo_hex = hex_data.get(24..).unwrap();
        let recipient = self.hex_to_address(recipient_hex);
        let flag = u8::from_str_radix(flag_hex.as_hex().to_string().as_str(), 16);
        let memo = self.hex2aq(memo_hex.as_hex().to_string().as_str());
        Some(Sweep {
            destination: recipient.unwrap_or("".to_string()),
            source: self.get_source().unwrap(),
            flag: flag.unwrap(),
            memo: memo.unwrap_or("".to_string()),
        })
    }

    fn decode_classic_send(&self, hex_data: &[u8]) -> Option<EnchanceSend> {
        let mut enchance_send = self.decode_enchanced_send(&hex_data[3..]);
        match &mut enchance_send {
            Some(enchance_send) => {
                enchance_send.recipient = self.transaction.vout[0]
                    .scriptpubkey_address
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string();
                return Some(enchance_send.clone());
            }
            None => {
                return {
                    println!("Error decoding classic send");
                    None
                }
            }
        }
    }

    pub fn get_tx_decoded(&self) -> Option<CounterPartyMessage> {
        let data = self.decode_op_return();
        match data {
            None => return None,
            Some(dat_hex) => {
                //split the data in the first 8 bytes and the rest of the data
                let cp_msg = dat_hex.get(8..).unwrap();
                //get the first byte of the data, this is the type of the message (enhanced send, issuance, etc)
                let hex_id = cp_msg.get(0..1).unwrap();
                if hex_id == ENCHANCED_SEND_ID {
                    //decode the enhanced send
                    let enchance_send = self.decode_enchanced_send(cp_msg);
                    match enchance_send {
                        None => return None,
                        Some(enchance_send) => {
                            return Some(CounterPartyMessage::EnchanceSend(enchance_send))
                        }
                    }
                }
                if hex_id == CLASSIC_SEND_ID {
                    //decode the classic send is the same as the enhanced send but the recipient is the first output of the tx
                    let enchance_send = self.decode_classic_send(cp_msg);
                    match enchance_send {
                        None => return None,
                        Some(enchance_send) => {
                            return Some(CounterPartyMessage::EnchanceSend(enchance_send))
                        }
                    }
                }
                if hex_id == SWEEP {
                    let sweep = self.decdode_sweep(cp_msg);
                    match sweep {
                        None => return None,
                        Some(sweep) => return Some(CounterPartyMessage::Sweep(sweep)),
                    }
                } else {
                    return None;
                }
            }
        }
    }
}
