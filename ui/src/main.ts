import { invoke } from "@tauri-apps/api/tauri";
import { register } from '@tauri-apps/api/globalShortcut';
import { readText } from '@tauri-apps/api/clipboard';

async function copyData(){
  const clipboardText = await readText();
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgEl.textContent = clipboardText 
}
await register('CommandOrControl+Shift+C', () => {
  copyData()
});

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
