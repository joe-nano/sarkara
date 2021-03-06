use rand::{ Rng, CryptoRng };
use crate::{ Packing, Error };

pub mod dilithium;


pub trait Signature {
    type PrivateKey: Packing;
    type PublicKey: Packing;
    type Signature: Packing;

    fn keypair<R: Rng + CryptoRng>(r: R) -> (Self::PrivateKey, Self::PublicKey);

    fn signature<R: Rng + CryptoRng>(r: R, sk: &Self::PrivateKey, data: &[u8]) -> Self::Signature;

    fn verify(pk: &Self::PublicKey, sig: &Self::Signature, data: &[u8]) -> Result<(), Error>;
}

pub trait DeterministicSignature: Signature {
    fn signature(sk: &Self::PrivateKey, data: &[u8]) -> Self::Signature;
}
