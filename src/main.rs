
use num_bigint::BigInt;
mod  shamir_secret_sharing;
use crate::shamir_secret_sharing::ShamirSecretShare;

fn main() {
     let sss = ShamirSecretShare {
         n: 2,
         k: 5,
         p: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap()
         };
    
     let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();
    
     let shares = sss.split(secret.clone());
    
     println!("shares: {:?}", shares);
     
}
