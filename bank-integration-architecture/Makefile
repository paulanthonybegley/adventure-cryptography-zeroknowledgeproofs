# Bank Trust-less Bridge Makefile

# Directories
SRC_DIR = src
VERUS_DIR = verus-arm64-macos
SPECS_DIR = verus-specs

# Verus executable
VERUS = ./$(VERUS_DIR)/verus

.PHONY: all run verify verify-frontend verify-engine verify-shield verify-guard verify-backend clean install-verus test

all: run verify

# Run the standard Rust executable demo
run:
	@echo "=================================================="
	@echo "🚀 Running the Bank Trust-less Bridge Demo..."
	@echo "=================================================="
	cargo run

# Verify all Verus specifications
verify: verify-frontend verify-engine verify-shield verify-guard verify-backend
	@echo "=================================================="
	@echo "✅ All Verus specifications successfully verified!"
	@echo "=================================================="

# Verify individual modules
verify-frontend:
	@echo "🔍 Verifying Frontend (Schnorr Identity)..."
	cd $(VERUS_DIR) && ./verus ../$(SPECS_DIR)/frontend_verus.rs

verify-engine:
	@echo "🔍 Verifying Engine (Merkle/Polynomial Batching)..."
	cd $(VERUS_DIR) && ./verus ../$(SPECS_DIR)/engine_verus.rs

verify-shield:
	@echo "🔍 Verifying Shield (Homomorphic Privacy)..."
	cd $(VERUS_DIR) && ./verus ../$(SPECS_DIR)/shield_verus.rs

verify-guard:
	@echo "🔍 Verifying Guard (VDF Fairness)..."
	cd $(VERUS_DIR) && ./verus ../$(SPECS_DIR)/guard_verus.rs

verify-backend:
	@echo "🔍 Verifying Backend (Ledger Safety)..."
	cd $(VERUS_DIR) && ./verus ../$(SPECS_DIR)/backend_verus.rs

# Standard cargo testing (if any tests are added in the future)
test:
	cargo test

# Clean Cargo target directory
clean:
	cargo clean
	@echo "Cleaned cargo target directory."

# Helper to re-download Verus (assuming Apple Silicon macOS)
install-verus:
	@echo "Downloading Verus for Apple Silicon..."
	curl -L "https://github.com/verus-lang/verus/releases/download/release%2F0.2024.04.03.21dfcd2/verus-0.2024.04.03.21dfcd2-arm64-macos.zip" -o verus.zip
	unzip verus.zip
	mv verus-0.2024.04.03.21dfcd2-arm64-macos $(VERUS_DIR)
	rm verus.zip
	cd $(VERUS_DIR) && xattr -dr com.apple.quarantine .
	@echo "Verus toolchain installed in $(VERUS_DIR)/"
