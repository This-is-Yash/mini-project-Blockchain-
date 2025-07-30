# 🎓 Certificate Issuer on Internet Computer (ICP)

This project is a blockchain-based certificate system built on the Internet Computer using Rust canisters and a web frontend.

## 🛠 Features

- 📝 Issue certificates for students and courses
- 🔒 Immutable storage on blockchain (ICP)
- 🔍 Public verification via hash
- 🧩 Frontend UI using HTML + JS

---

## 📦 Project Structure
miniproject/
├── src/
│ ├── certificate_backend/ # Rust-based canister
│ │ ├── Cargo.toml
│ │ └── src/lib.rs
│ ├── certificate_frontend/ # HTML/JS UI
│ │ └── dist/index.html
│ │ └── dist/script.js
├── dfx.json
├── README.md
