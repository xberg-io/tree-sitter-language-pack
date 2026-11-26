use aiken/collection/list
use cardano/transaction.{Transaction, OutputReference}

pub type Datum {
  owner: ByteArray,
  deadline: Int,
}

pub type Action {
  Claim
  Cancel
}

pub type Lovelace = Int

const minimum_lovelace: Int = 2000000

fn is_signed(tx: Transaction, key: ByteArray) -> Bool {
  list.has(tx.extra_signatories, key)
}

validator vesting {
  spend(datum: Option<Datum>, _redeemer: Action, _ref: OutputReference, tx: Transaction) {
    expect Some(d) = datum
    is_signed(tx, d.owner)
  }
}

test signed_by_owner() {
  True
}
