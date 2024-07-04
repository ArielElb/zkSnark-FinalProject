use ark_bls12_381::Config;
use ark_ec::bls12::Bls12;
use ark_ec::pairing::Pairing;
use ark_groth16::{PreparedVerifyingKey, Proof};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use base64::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

//Generate proof and write to file
pub fn write_proof_to_file(proof: &Proof<Bls12<Config>>, file_path: &str) -> Result<(), io::Error> {
    let mut compressed_bytes = Vec::new();
    proof.serialize_compressed(&mut compressed_bytes).unwrap();

    println!("Writing proof to file: {}", file_path);
    // print the bytes
    println!("Bytes: {:?}", compressed_bytes);
    let mut file: File = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(file_path)
        .unwrap();
    file.write_all(&compressed_bytes)?;
    file.flush()?;
    Ok(())
}

// Read proof from file
pub fn read_proof<E: Pairing>(file_path: &str) -> Result<Proof<E>, Box<dyn Error>> {
    // Open and read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Deserialize the proof from the buffer
    let proof: Proof<E> = Proof::<E>::deserialize_compressed(&mut buffer.as_slice())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    return Ok(proof);
}

// encode the proof to base64:
pub fn encode_proof<E: Pairing>(proof: &Proof<Bls12<Config>>) -> String {
    // serialize the pvk:
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();
    let proof_str = BASE64_STANDARD.encode(&proof_bytes);
    proof_str
}
// decode the proof from base64:
pub fn decode_proof<E: Pairing>(proof_str: &str) -> Result<Proof<Bls12<Config>>, Box<dyn Error>> {
    // decode the proof from base64:
    let proof_bytes = BASE64_STANDARD.decode(proof_str.as_bytes()).unwrap();
    let proof: Proof<Bls12<Config>> =
        Proof::<Bls12<Config>>::deserialize_compressed(&mut proof_bytes.as_slice())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(proof)
}

pub fn encode_pvk<E: Pairing>(pvk: &PreparedVerifyingKey<E>) -> String {
    // serialize the pvk:
    let mut pvk_bytes = Vec::new();
    pvk.serialize_compressed(&mut pvk_bytes).unwrap();
    let pvk_str = BASE64_STANDARD.encode(&pvk_bytes);
    pvk_str
}

pub fn decode_pvk<E: Pairing>(pvk_str: &str) -> Result<PreparedVerifyingKey<E>, Box<dyn Error>> {
    // decode the pvk from base64:
    let pvk_bytes = BASE64_STANDARD.decode(pvk_str.as_bytes()).unwrap();
    let pvk: PreparedVerifyingKey<E> =
        PreparedVerifyingKey::<E>::deserialize_compressed(&mut pvk_bytes.as_slice())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(pvk)
}

pub fn encode_hash(hash: &Vec<u8>) -> String {
    // encode the hash to base64:
    let hash_str = BASE64_STANDARD.encode(hash);
    hash_str
}
pub fn decode_hash(hash_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // decode the hash from base64:
    let hash_bytes = BASE64_STANDARD.decode(hash_str.as_bytes()).unwrap();
    Ok(hash_bytes)
}
