import { invoke } from "@tauri-apps/api/core";

/** @type {HTMLFormElement} **/
const form = document.getElementById("test");

form.addEventListener("submit", async (e) => {
    e.preventDefault();
    /** @type {HTMLInputElement} **/
    const input = document.getElementById("instance");
    const instanceName = input.value.trim();

    try {
        const path = await invoke("create_command", { instanceName: instanceName });
        form.reset();
    } catch (err) {
        console.error(err);
    }
})