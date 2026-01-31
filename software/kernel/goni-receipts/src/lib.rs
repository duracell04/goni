use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ReceiptError {
    #[error("io error: {0}")]
    Io(String),
    #[error("parse error: {0}")]
    Parse(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: Uuid,
    pub timestamp: String,
    pub action_type: String,
    pub policy_decision: String,
    pub capability_id: Option<Uuid>,
    pub input_hash: String,
    pub output_hash: String,
    pub prev_hash: Option<String>,
    pub chain_hash: String,
}

pub struct ReceiptLog {
    path: PathBuf,
    last_hash: Mutex<Option<String>>,
}

impl ReceiptLog {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ReceiptError> {
        let path = path.as_ref().to_path_buf();
        let last_hash = read_last_hash(&path)?;
        Ok(Self {
            path,
            last_hash: Mutex::new(last_hash),
        })
    }

    pub fn append(&self, mut receipt: Receipt) -> Result<(), ReceiptError> {
        let mut last = self.last_hash.lock().map_err(|_| ReceiptError::Io("lock".into()))?;
        receipt.prev_hash = last.clone();
        receipt.chain_hash = hash_receipt(&receipt);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| ReceiptError::Io(e.to_string()))?;
        let line = serde_json::to_string(&receipt).map_err(|e| ReceiptError::Parse(e.to_string()))?;
        writeln!(file, "{line}").map_err(|e| ReceiptError::Io(e.to_string()))?;
        *last = Some(receipt.chain_hash.clone());
        Ok(())
    }
}

pub fn verify_log(path: impl AsRef<Path>) -> Result<(), ReceiptError> {
    let file = File::open(path.as_ref()).map_err(|e| ReceiptError::Io(e.to_string()))?;
    let reader = BufReader::new(file);
    let mut prev: Option<String> = None;
    for line in reader.lines() {
        let line = line.map_err(|e| ReceiptError::Io(e.to_string()))?;
        let receipt: Receipt = serde_json::from_str(&line).map_err(|e| ReceiptError::Parse(e.to_string()))?;
        if receipt.prev_hash != prev {
            return Err(ReceiptError::Parse("hash chain mismatch".into()));
        }
        let expected = hash_receipt(&receipt);
        if receipt.chain_hash != expected {
            return Err(ReceiptError::Parse("chain hash invalid".into()));
        }
        prev = Some(receipt.chain_hash);
    }
    Ok(())
}

fn hash_receipt(receipt: &Receipt) -> String {
    let mut h = Sha256::new();
    h.update(receipt.receipt_id.to_string());
    h.update(&receipt.timestamp);
    h.update(&receipt.action_type);
    h.update(&receipt.policy_decision);
    if let Some(id) = receipt.capability_id {
        h.update(id.to_string());
    }
    h.update(&receipt.input_hash);
    h.update(&receipt.output_hash);
    if let Some(prev) = &receipt.prev_hash {
        h.update(prev);
    }
    format!("{:x}", h.finalize())
}

fn read_last_hash(path: &Path) -> Result<Option<String>, ReceiptError> {
    if !path.exists() {
        return Ok(None);
    }
    let file = File::open(path).map_err(|e| ReceiptError::Io(e.to_string()))?;
    let reader = BufReader::new(file);
    let mut last: Option<String> = None;
    for line in reader.lines() {
        let line = line.map_err(|e| ReceiptError::Io(e.to_string()))?;
        let receipt: Receipt = serde_json::from_str(&line).map_err(|e| ReceiptError::Parse(e.to_string()))?;
        last = Some(receipt.chain_hash);
    }
    Ok(last)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn receipt_chain_verifies() {
        let path = "target/test_receipts.jsonl";
        let _ = fs::remove_file(path);
        let log = ReceiptLog::open(path).unwrap();
        let r1 = Receipt {
            receipt_id: Uuid::new_v4(),
            timestamp: "t1".into(),
            action_type: "demo".into(),
            policy_decision: "allow".into(),
            capability_id: None,
            input_hash: "a".into(),
            output_hash: "b".into(),
            prev_hash: None,
            chain_hash: "".into(),
        };
        log.append(r1).unwrap();
        verify_log(path).unwrap();
    }
}
