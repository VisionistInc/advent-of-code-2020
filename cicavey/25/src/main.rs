fn calc_loop_size(pk: u64) -> i32 {
    let sn = 7;
    let mut v = 1;

    for i in 1..10_000_000 {
        v *= sn;
        v %= 20201227;

        if v == pk {
            return i;
        }
    }

    -1
}

fn derive_enc(sn: u64, loop_size: i32) -> u64 {
    let mut v = 1;

    for _ in 0..loop_size {
        v *= sn;
        v %= 20201227;
    }

    v
}

fn main() {
    // test
    // let card_pk: u64 = 5764801;
    // let door_pk: u64 = 17807724;

    let card_pk: u64 = 1614360;
    let door_pk: u64 = 7734663;

    let card_ls = calc_loop_size(card_pk);
    let door_ls = calc_loop_size(door_pk);

    let door_enc = derive_enc(door_pk, card_ls);
    let card_enc = derive_enc(card_pk, door_ls);

    dbg!(card_ls, door_ls, door_enc, card_enc);
}
