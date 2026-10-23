//! Example: Basic Agent Guard using Hyperion-Zero VSTE
//! Demonstrates deterministic vector intent verification and SHA-256 receipt generation.

use std::error::Error;

/// Simulated Enclave Safety Result
#[derive(Debug, PartialEq)]
pub enum SafetyResult {
    Secured { receipt: String },
    ViolationDetected { boundary_id: u32 },
}

/// Simulated Hyperion Enclave Configuration
pub struct EnclaveConfig {
    pub max_dimensions: usize,
    pub strict_mode: bool,
}

impl EnclaveConfig {
    pub fn default_strict() -> Self {
        Self {
            max_dimensions: 4,
            strict_mode: true,
        }
    }
}

/// Core Enclave Instance
pub struct HyperionEnclave {
    config: EnclaveConfig,
}

impl HyperionEnclave {
    pub fn new(config: EnclaveConfig) -> Self {
        Self { config }
    }

    /// Evaluates semantic vector embeddings against bounded convex polytopes
    pub fn verify_intent(&self, vector: &[f64]) -> SafetyResult {
        if vector.len() > self.config.max_dimensions {
            return SafetyResult::ViolationDetected { boundary_id: 101 };
        }

        // Check for out-of-bounds topological drift (hyperplane boundary check)
        for &val in vector {
            if val.abs() > 1.0 {
                return SafetyResult::ViolationDetected { boundary_id: 404 };
            }
        }

        // Mock deterministic SHA-256 state receipt generation
        let receipt_hash = format!("0x7f8a9b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a");
        SafetyResult::Secured { receipt: receipt_hash }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🛡️ Initializing Hyperion-Zero VSTE Security Enclave...");

    let config = EnclaveConfig::default_strict();
    let enclave = HyperionEnclave::new(config);

    // 1. Safe Semantic Input Vector
    let safe_vector = vec![0.12, -0.45, 0.89, 0.02];
    println!("\n[Test 1] Evaluating Safe Agent Vector: {:?}", safe_vector);

    match enclave.verify_intent(&safe_vector) {
        SafetyResult::Secured { receipt } => {
            println!("✅ STATUS: PASSED | Cryptographic Receipt: {}", receipt);
        }
        SafetyResult::ViolationDetected { boundary_id } => {
            println!("❌ STATUS: REJECTED | Hyperplane Boundary Breach: {}", boundary_id);
        }
    }

    // 2. Malicious / OOB Drift Vector (Prompt Injection Simulation)
    let hostile_vector = vec![0.12, 1.85, -0.22, 0.05];
    println!("\n[Test 2] Evaluating Hostile Vector: {:?}", hostile_vector);

    match enclave.verify_intent(&hostile_vector) {
        SafetyResult::Secured { receipt } => {
            println!("✅ STATUS: PASSED | Cryptographic Receipt: {}", receipt);
        }
        SafetyResult::ViolationDetected { boundary_id } => {
            println!("🚨 STATUS: BREACH DETECTED | Policy Violation at Polytope Boundary {}", boundary_id);
        }
    }

    Ok(())
}
