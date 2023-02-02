import { Computer } from '../pkg/hack_computer.js';

const computer = Computer.new();
document.querySelector('body').addEventListener('keydown', (event) => {
  const key = getCharCode(event.key);
  computer.set_keyboard(key);
  setPressedKey(key);
});
document.querySelector('body').addEventListener('keyup', (_) => {
  computer.set_keyboard(0);
  setPressedKey(0);
});
function setPressedKey(key) {
  const pressedKeyDiv = document.querySelector('.pressed-key');
  pressedKeyDiv.textContent = key === 13 ? 'Enter' : String.fromCodePoint(key);
}
function getCharCode(key) {
  if (key.length === 1) {
    return key.charCodeAt();
  }
  if (key === 'Enter') return 13;
  return 0;
}
