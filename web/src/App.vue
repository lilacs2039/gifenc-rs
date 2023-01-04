<script setup lang="ts">
import { ref, reactive, inject } from "vue";
import encode_gif_init from "./wasm/gif_encoder";
import { encode_gif } from "./wasm/gif_encoder";

const url = ref("");

// wasm初期化
encode_gif_init("wasm/gif_encoder_bg.wasm").then(() => {
  // b64 -> image bytes
  const b64 =
    "iVBORw0KGgoAAAANSUhEUgAAAEQAAAA3CAYAAAC1pezdAAAAAXNSR0IArs4c6QAABHlJREFUaEPtml1IVEEUx49QsoJhqUmlUithWCAbZAph9CBoQhkYgVYESQtF4UsG0ZdhIeXLSpAgfVLkS4EWWIIPgQSJRSJY6MOupFZoVtKGooFxNsdmr3e+XPfeMe487a5z58787v9/zpm5xng8nhlw2hyBGAdIuBocIAZ3OEAcIPyA6SjEUYijEKWiwrGM/pZxgTshB3Zk7YeklSncpzv2YwRefXgMgfEOJRXwOmulkBWuZCjxXBCCoBeEULC1dNfAz8mvEYPRAgiCWP6+BIrLs5VgGMG0PuqB6c0tEYGxGYi8PWQf/T8bdQHApOxl9u9l3An5UnFCeUWzFyw0vliuENU4EbeKjWTiuxjXg5vtSjayEIgLEv0HpeIEQlibGQvZu9ZBXFyM6aonJmZg7OMvePNiFERgUC0YX75l3BYStASIrD0QxLai1ZC2KV44cbrDUF8Qhv3j4O+cijhNRxWIij0ycmMhd3eqEghjZwQjqxhWmo4qkER/BRw+UcBd5EJVwRo0OPYb2h8Ocm3EC7hRBXIor55bVyw2DAKJBYXEEuz3JdgLsdmv53G1FUh+mXq8kPUUQnl2Y3CuO2YbYzMLsrYBWYyYIYLT+XwYLh9pZXazHEhlURNzMntOpUN80jLRmoBkkM/9fzMIpuPUjARhJkKFHMi6yx1fGyAy6hBlDFb8wfqk+doAmFnEdsuYKQQXUujdwCy4cNIIo6NpVKge7EDgIoiel5+4FtEWyL5KN3OxuLC2xgFh9UkPgMHZmx9uT9w5Y3WKzXuuABqvahBUzRQisgsGQlHFSWDQaZQGhDDS3ClzEC7dLzZVjhYxhAeE+F/kFRYIogYSeLFqpT/jnoeOLVoA4dUeotjBA0EgojpyCtNCX7vahkJKwf1Rc0Nv6DdiI/y8pIHIwKChLBnLqCqEgMBSm7Q18VuYruIphFaHNgrhxRC63DYDYaRgBENgYOGGDY8EsOF3/IzFHZ1ttLCMKMs01wdgKDACdxrZVS4Nxkwt/02WIUXZ3q3nRYkm7O8Xayvn1Rl0HYJHEFps7hZaqTZVB6Duer0UlKPeMjjpywWVkp0MrIVl6HKbt2Is0I6X1nGhVJ2phLLq8KoX49C9mrdh6ZU1iDZAZPYzxD64p0G5kyyDMYMET96Ro2jrr02WIdkDfW18ulIeUehEVMZK07YqxCyNPn13RXiuobD+sK5onZ3pp8N+M4KxBYionogGFIRxrPws9HVMm/JEMLacqeKpO11hsp72YkLBbFO6/pbUfS0/ZIZJF0z1e6RU3/CkKuL3MhgzfD4fUxn0RGIzuwFc81+GR/WQmUxgqidPCgrWFBW127mnaayBVGoXM2WQcS0BErpZMBmm/BulwKDH8VBH9CYPFYFbfNkyn6UKelLWASF3VQCDl5C6g/73KroukSIMYPpSyuxa64HMzkLWRrILZvXj2UMrIKHJKARdVTAy9tAPCJnRIoJZKAjrg6rEI47URqr20FchErCs6mJbULVqgar3cYAYiDlAHCB8EzkKMfD5A4hWsA9l8vSqAAAAAElFTkSuQmCC";
  const bin = window.atob(b64);
  // const buffer = new TextEncoder().encode(bin);
  var buffer = new Uint8Array(bin.length);
  for (var i = 0; i < bin.length; i++) {
    buffer[i] = bin.charCodeAt(i);
  }
  const buf = encode_gif(buffer);
  // Blobを作成
  try {
    var blob = new Blob([buf], {
      type: "image/gif",
    });
    url.value = URL.createObjectURL(blob);
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <div>
    <div>
      <img
        alt="Vue logo"
        class="logo"
        src="./assets/logo.svg"
        width="125"
        height="125"
      />
    </div>
    <div class="">GIF example</div>

    <img :src="url" />
  </div>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }
}
</style>
