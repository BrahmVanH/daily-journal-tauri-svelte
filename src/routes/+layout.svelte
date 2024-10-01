<script>
  import Nav from "./Nav.svelte";
  import "./global.css";
</script>

<div class="layout">
  <Nav />
  <slot />
</div>

<style>
  .layout {
    width: 100vw;
    height: 100vh;
  }
</style>
