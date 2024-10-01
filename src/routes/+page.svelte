<script lang="ts">
  import SecuritiesForm from "./SecuritiesForm.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  async function run_migrations(e: any) {
    await invoke("run_migrations");
  }
</script>

<main>
  <SecuritiesForm />
  <button on:click={run_migrations}>run_migrations</button>
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
    background-color: transparent;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
  }
</style>
