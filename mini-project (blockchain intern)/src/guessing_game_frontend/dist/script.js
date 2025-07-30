import { guessing_game_backend } from "../../../.dfx/local/canisters/guessing_game_backend";

window.submit = async function () {
  const input = document.getElementById("guess");
  const guess = parseInt(input.value);
  const result = await guessing_game_backend.submit_guess(guess);
  document.getElementById("result").innerText = result.message;
};
