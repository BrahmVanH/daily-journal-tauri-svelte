<script lang="ts">
  import Slider from "@smui/slider";

  let loading = false;
  let financial = 0;
  let fitness = 0;
  let mental = 0;
  let dietary = 0;
  let social = 0;
  let professional = 0;
</script>

<form>
  {#if loading}
    <div>loading</div>
  {:else}
    <div class="slider-wrapper">
      <label for="financial">Financial</label>
      <Slider
      style="width: 100%; "
        min={0}
        max={5}
        step={1}
        input$aria-label="Range slider"
        bind:value={financial}
      />
      <input hidden type="number" name="financial" bind:value={financial} />
      <label for="financial">Fitness</label>
      <Slider
        min={0}
        max={5}
        step={1}
        class="slider"
        input$aria-label="Range slider"
        bind:value={fitness}
      />
      <input hidden type="number" name="fitness" bind:value={fitness} />

      <label for="financial">Mental</label>
      <Slider
        min={0}
        max={5}
        step={1}
        class="slider"
        input$aria-label="Range slider"
        bind:value={mental}
      />
      <input hidden type="number" name="mental" bind:value={mental} />

      <label for="financial">Dietary</label>
      <Slider
        min={0}
        max={5}
        step={1}
        class="slider"
        input$aria-label="Range slider"
        bind:value={dietary}
      />
      <input hidden type="number" name="dietary" bind:value={dietary} />

      <label for="financial">Social</label>
      <Slider
        min={0}
        max={5}
        step={1}
        class="slider"
        input$aria-label="Range slider"
        bind:value={social}
      />
      <input hidden type="number" name="social" bind:value={social} />
    </div>
  {/if}
</form>

<style>
  form {
    margin-top: 10%;
    height: min-content;
    padding: 3rem 2rem;
    width: 80%;
    border: 1px solid white;
    border-radius: 65px;
    display: flex;
    flex-direction: column;
    align-items: center;
    accent-color: white;

    @media (min-width: 768px) {
      width: 30%;
      margin-top: 7.5%;
      height: 60%;
    }
  }

  .slider-wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    width: 100%;
  }

  div > input {
    width: 100%;
  }

 
 
</style>
