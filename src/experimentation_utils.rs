use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

use bellman::groth16::Proof;
use bls12_381::Bls12;

pub fn write_to_csv(
    file_name: &str,
    proof_type: &str,
    proof_time: f64,
    r_proof_size: i32,
    s_proof_size: i32,
    verification_time: f64,
    num_constraints: i32,
) -> Result<(), Box<dyn Error>> {
    let file_exists = std::path::Path::new(file_name).exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    if !file_exists {
        writeln!(
            file,
            "proof_type,proof_time,r_proof_size,s_proof_size,verification_time,num_constraints"
        )?;
    } else {
        // Move the cursor to the end of the file to append data
        file.seek(SeekFrom::End(0))?;
    }

    writeln!(
        file,
        "{proof_type},{proof_time},{r_proof_size},{s_proof_size},{verification_time},{num_constraints}"
    )?;
    Ok(())
}

pub fn proof_to_bytes(proof: &Proof<Bls12>) -> Vec<u8> {
    let mut bytes = Vec::new();

    // Get compressed representations
    let a_bytes = proof.a.to_compressed();
    let b_bytes = proof.b.to_compressed();
    let c_bytes = proof.c.to_compressed();

    // Concatenate all bytes
    bytes.extend_from_slice(&a_bytes);
    bytes.extend_from_slice(&b_bytes);
    bytes.extend_from_slice(&c_bytes);

    bytes
}
