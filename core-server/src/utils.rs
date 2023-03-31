use chashmap::WriteGuard;
use yrs::{updates::decoder::Decode, Transact, Update};

pub fn apply_update(doc: &WriteGuard<String, yrs::Doc>, update: &Vec<u8>) {
    let mut tx = doc.transact_mut();

    let update_to_apply = Update::decode_v1(&*update).unwrap();

    tx.apply_update(update_to_apply);
}
