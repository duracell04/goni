use sha2::{Digest, Sha256};

pub fn embed(text: &str, dim: usize) -> Vec<f32> {
    let mut v = vec![0.0f32; dim];
    for token in text.split_whitespace() {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let digest = hasher.finalize();
        let idx = (digest[0] as usize) % dim;
        let sign = if digest[1] % 2 == 0 { 1.0 } else { -1.0 };
        v[idx] += sign;
    }
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_embedding() {
        let a = embed("hello world", 16);
        let b = embed("hello world", 16);
        assert_eq!(a, b);
    }
}
