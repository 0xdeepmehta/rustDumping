use super::utils::ParseError;

fn get_pubkeys() -> (u64, u64) {
    (8987316,  14681524)
}

fn get_example_keys() -> (u64, u64) {
    (5764801, 17807724)
}

fn get_loop_size(pubkey: u64, subject_number: u64) -> u64 {
    let mut v = 1;
    let mut ls = 0;

    while v != pubkey {
        v *= subject_number;
        v = v % 20201227;
        ls += 1;
    }

    ls
}

fn encrypt(key: u64, subject_number: u64) -> u64 {
    let mut v = 1;

    for _ in 0..key {
        v *= subject_number;
        v = v % 20201227;
    }

    v
}

pub fn problem1() -> Result<(), ParseError> {
    let (pub1, pub2) = get_pubkeys();

    let ls1 = get_loop_size(pub1, 7);
    let enc = encrypt(ls1, pub2);

    println!("25/1: encryption key is: {}", enc);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    Ok(())
}
