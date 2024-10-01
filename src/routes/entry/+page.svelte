<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function handle_record_entry(event: any) {
    const { text, financial, fitness, social, mental, professional, dietary } =
      event.target;
  }
</script>

<main>
  <div class="container">
    <h1>{formattedDate}</h1>
  </div>
</main>

<style>
  main {
    background-color: linear-gradient(
      var(--bg-primary),
      var(--secondary-color)
    );
    display: flex;
    flex-direction: column;
    align-items: center;
    min-width: 100%;
  }

  .container {
    background-color: transparent;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 90%;
    border: 1px solid white;
    border-radius: 65px;
    padding: 1rem 0rem;
  }

  .container div {
    border-bottom: 1px solid black;
    padding-bottom: 0.5rem;
  }

  .text {
    color: black;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .chart-wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
