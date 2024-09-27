<script>
</script>

<nav>
  <div class="inner-nav">
    <a> History </a>
    <span class="vertical-line">|</span>
    <a> Add </a>
  </div>
</nav>

<style>
  nav {
    width: 100%;
    background-color: transparent;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow-y: hidden;
  }

  .inner-nav {
    display: flex;
    justify-content: "space-evenly";
    align-items: "center";

    position: fixed;
    z-index: 1000;
    bottom: 0;
    width: 75%;
    height: 10%;
    background-color: var(--bg-primary);
    border: 1px solid white;
    border-top-left-radius: 45px;
    border-top-right-radius: 45px;
  }

  button {
    width: 30%;
    height: 50%;
    margin: 0rem;
    padding: 0rem;
    background-color: transparent;
    color: white;
    font-size: 1.5rem;
    border: 1px solid white;
    border-radius: var(--border-rad);
  }

  a {
    width: 30%;
    height: 50%;
    margin: 0rem;
    padding: 0rem;
    background-color: transparent;
    color: white;
    font-size: 1.5rem;
    border: 1px solid white;
    border-radius: var(--border-rad);
    text-align: center;
  }

  .vertical-line {
    font-size: 48px;
    color: white;
    font-weight: 100;
  }
</style>
