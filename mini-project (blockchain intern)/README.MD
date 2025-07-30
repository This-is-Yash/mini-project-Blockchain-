# 🎮 Guessing Game on Blockchain

This is a simple **Guessing Game** built using **Internet Computer (ICP)** and **Rust canisters**. Players submit their guesses to the blockchain, which checks if it's correct, and stores guesses in a transparent and immutable ledger.

---

## 🛠 Features

- **Blockchain-powered**: All guesses and results are stored on the blockchain for transparency and immutability.
- **Simple Game**: The player needs to guess the secret number (between 1-10).
- **Real-time Results**: Instantly check if the guess is correct or not.

---

## 🎮 Gameplay

- The secret number is randomly set on canister deployment (currently `7` by default).
- Users submit their guesses (a number between 1 and 10).
- The canister checks if the guess is correct and returns a result with a message.
- The game is simple: **one guess per user** is recorded on the blockchain.
- The results (correct/incorrect) and all guesses are stored immutably, making it a perfect use case for blockchain technology.

---

## 📁 Project Structure

guessing_game/
├── src/
│   ├── guessing_game_backend/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── guessing_game_frontend/
│   │   └── dist/index.html
│   │   └── dist/script.js
├── dfx.json
├── README.md
