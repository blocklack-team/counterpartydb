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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DexOrder {
    source: String,
    get_asset: String,
    get_quantity: i64,
    give_asset: String,
    give_quantity: i64,
    expiration: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BtcPay {
    order_0: String,
    order_1: String,
    amount: i64,
    recipient: String,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CounterPartyTransaction {
    pub transaction: InputOutput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuance {
    asset: String,
    quantity: i64,
    divisible: bool,
    call: i64,
    call_date: i64,
    call_price: i64,
    len: i64,
    description: String,
    issuer: String,
}

pub enum CounterPartyMessage {
    Sweep(Sweep),
    EnchanceSend(EnchanceSend),
    DexOrder(DexOrder),
    BtcPay(BtcPay),
    Issuance(Issuance),
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
        let mut result = Vec::new();
        for vout in &self.transaction.vout {
            if vout.scriptpubkey_type == "op_return" {
                let first_input = &self.transaction.vin[0];
                //get the key for decode the data if it is an enhanced send, the key is the txid of the first input
                let prev_hash = match Vec::from_hex(&first_input.txid) {
                    Ok(d) => d,
                    Err(_e) => continue,
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
                println!("data_slice: {:?}", data_slice.as_hex().to_string());
                match self.is_counterparty(data_slice) {
                    false => {
                        println!("Not a counterparty message");
                        continue;
                    }
                    true => result.extend_from_slice(data_slice),
                }
            }
            if vout.scriptpubkey_type == "multisig" {
                //get the data from the scriptpubkey asm, because is a old counterparty message
                let mut scrip_bytes = Vec::from_hex(vout.scriptpubkey.as_str()).unwrap();
                if scrip_bytes.len() == 105 {
                    let first_input = &self.transaction.vin[0];
                    //get the key for decode the data if it is an enhanced send, the key is the txid of the first input
                    let prev_hash = match Vec::from_hex(&first_input.txid) {
                        Ok(d) => d,
                        Err(_e) => continue,
                    };
                    //init the rc4 key
                    let rc4 = RC4Key::<U32>::from_slice(&prev_hash);
                    //init the rc4 cipher with the key
                    let mut cipher = Rc4::new(rc4);

                    let mut raw1 = scrip_bytes.get(3..34).unwrap().to_vec();
                    let raw2 = scrip_bytes.get(37..68).unwrap().to_vec();
                    raw1.extend(raw2);
                    let data = raw1.as_mut_slice();
                    cipher.apply_keystream(data);
                    match self.is_counterparty(data) || self.is_counterparty(&result) {
                        false => {
                            println!("Not a counterparty message");
                            continue;
                        }
                        true => result.extend_from_slice(data),
                    }
                    continue;
                }
                let length = scrip_bytes[1];
                scrip_bytes.drain(0..length as usize + 4);
                scrip_bytes.drain(scrip_bytes.len() - 2..scrip_bytes.len());
                match self.is_counterparty(&scrip_bytes) || self.is_counterparty(&result) {
                    false => {
                        println!("Not a counterparty message");
                        continue;
                    }
                    true => result.extend_from_slice(&scrip_bytes),
                }
            }
        }
        result.into()
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

    fn decode_dex_order(&self, hex_data: &[u8]) -> Option<DexOrder> {
        //from the 1st to the 8th byte is the asset
        let give_asset_bytes = hex_data.get(1..9).unwrap();
        //from the 9th to the 16th byte is the quantity
        let give_quantity_bytes = hex_data.get(9..17).unwrap();
        //from the 17th to the 24th byte is the get asset
        let get_asset_bytes = hex_data.get(17..25).unwrap();
        //from the 24th byte to the 32th byte is the get quantity
        let get_quantity_bytes = hex_data.get(25..33).unwrap();
        //from the 32th byte to the 36th byte is the expiration
        let expiration_bytes = hex_data.get(33..37).unwrap();

        let give_asset = self.get_asset_name(
            u64::from_str_radix(give_asset_bytes.as_hex().to_string().as_str(), 16).unwrap(),
        );
        let give_quantity =
            i64::from_str_radix(give_quantity_bytes.as_hex().to_string().as_str(), 16);
        let get_asset = self.get_asset_name(
            u64::from_str_radix(get_asset_bytes.as_hex().to_string().as_str(), 16).unwrap(),
        );
        let get_quantity =
            i64::from_str_radix(get_quantity_bytes.as_hex().to_string().as_str(), 16);
        let expiration = i32::from_str_radix(expiration_bytes.as_hex().to_string().as_str(), 16);
        Some(DexOrder {
            source: self.get_source().unwrap(),
            give_asset: give_asset,
            give_quantity: give_quantity.unwrap(),
            get_asset: get_asset,
            get_quantity: get_quantity.unwrap(),
            expiration: expiration.unwrap(),
        })
    }

    fn decode_btc_pay(&self, hex_data: &[u8]) -> Option<BtcPay> {
        let order_0 = hex_data.get(1..33).unwrap();
        let order_1 = hex_data.get(33..65).unwrap();
        let amount = self.transaction.vout[0].value as i64;
        let recipient = self.transaction.vout[0]
            .scriptpubkey_address
            .as_ref()
            .unwrap();
        Some(BtcPay {
            order_0: order_0.as_hex().to_string(),
            order_1: order_1.as_hex().to_string(),
            amount: amount,
            recipient: recipient.to_string(),
            source: self.get_source().unwrap(),
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

    fn decode_issuance_pre2022(&self, hex_data: &[u8]) -> Option<Issuance> {
        let asset_hex = hex_data.get(4..12).unwrap();
        let quantity_hex = hex_data.get(12..20).unwrap();
        let divisible_hex = hex_data.get(20..21).unwrap();
        let call_hex = hex_data.get(21..22).unwrap();
        let call_date_hex = hex_data.get(22..26).unwrap();
        let call_price_hex = hex_data.get(26..30).unwrap();
        let len_hex = hex_data.get(30..31).unwrap();
        let description_hex = hex_data.get(28..).unwrap();
        let asset_name = self.get_asset_name(
            u64::from_str_radix(asset_hex.as_hex().to_string().as_str(), 16).unwrap(),
        );
        let quantity = i64::from_str_radix(quantity_hex.as_hex().to_string().as_str(), 16);
        let divisible =
            i64::from_str_radix(divisible_hex.as_hex().to_string().as_str(), 16).unwrap() == 1;
        let call = i64::from_str_radix(call_hex.as_hex().to_string().as_str(), 16);
        let call_date = i64::from_str_radix(call_date_hex.as_hex().to_string().as_str(), 16);
        let call_price = i64::from_str_radix(call_price_hex.as_hex().to_string().as_str(), 16);
        let len = i64::from_str_radix(len_hex.as_hex().to_string().as_str(), 16);
        let description = self.hex2aq(description_hex.as_hex().to_string().as_str());
        Some(Issuance {
            issuer: self.get_source().unwrap(),
            asset: asset_name,
            quantity: quantity.unwrap(),
            divisible: divisible,
            call: call.unwrap(),
            call_date: call_date.unwrap(),
            call_price: call_price.unwrap(),
            len: len.unwrap(),
            description: description.unwrap_or("".to_string()),
        })
    }

    pub fn get_tx_decoded(&self) -> Option<CounterPartyMessage> {
        let data = self.decode_op_return();
        match data {
            None => return None,
            Some(dat_hex) => {
                println!("dat_hex: {:?}", dat_hex.as_hex().to_string());
                //split the data in the first 8 bytes and the rest of the data
                let cp_msg = dat_hex.get(8..).unwrap();
                //get the first byte of the data, this is the type of the message (enhanced send, issuance, etc)
                let mut hex_id = cp_msg.get(0..1).unwrap();
                if hex_id == [0] {
                    hex_id = cp_msg.get(3..4).unwrap();
                }
                println!("hex_id: {:?}", hex_id);
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
                }
                if hex_id == DEX_ORDER {
                    let dex_order = self.decode_dex_order(cp_msg);
                    match dex_order {
                        None => return None,
                        Some(dex_order) => return Some(CounterPartyMessage::DexOrder(dex_order)),
                    }
                }
                if hex_id == BTC_PAY {
                    let btc_pay = self.decode_btc_pay(cp_msg);
                    match btc_pay {
                        None => return None,
                        Some(btc_pay) => return Some(CounterPartyMessage::BtcPay(btc_pay)),
                    }
                }
                if hex_id == ISSUANCE {
                    let issuance = self.decode_issuance_pre2022(cp_msg);
                    match issuance {
                        None => return None,
                        Some(issuance) => return Some(CounterPartyMessage::Issuance(issuance)),
                    }
                } else {
                    return None;
                }
            }
        }
    }
}
