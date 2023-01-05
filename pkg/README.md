# gifenc-rs

WebAssembly GIF converter (callable from TypeScript) built from rust image crate.



# Demo

https://lilacs2039.github.io/rust-gif-wasm



# npm

https://www.npmjs.com/package/rust-gif-wasm



# Install

```bash
npm install gifenc-rs
```





# Usage



```vue
<script setup lang="ts">
import encode_gif_init from "gifenc-rs";
import { encode_gif } from "gifenc-rs";
    
// init wasm
encode_gif_init("wasm/gifenc-rs_bg.wasm")

// ...
// convert
const buf: Uint8Array = encode_gif(buffer as Uint8Array);

```


<details>
proper example


```vue
<script setup lang="ts">
import encode_gif_init from "./wasm/gif_encoder";
import { encode_gif } from "./wasm/gif_encoder";

// init wasm
encode_gif_init("wasm/gif_encoder_bg.wasm")

function encode(objectUrl: string) {
  const bin = window.atob(objectUrl.replace(/.+,/, ""));
  var buffer = new Uint8Array(bin.length);
  for (var i = 0; i < bin.length; i++) {
    buffer[i] = bin.charCodeAt(i);
  }
  const buf: Uint8Array = encode_gif(buffer as Uint8Array);
  // Convert to Blob
  try {
    var blob = new Blob([buf], {
      type: "image/gif",
    });
    url.value = URL.createObjectURL(blob);
    post_thumb_mes.value = `Type : ${blob.type},  Size : ${Math.round(
      blob.size / 1000
    )}kB`;
  } catch (e) {
    console.error(e);
  }
}
</script>

<template></template>
```

</details>



# Development

## Build

```bash
wasm-pack build --target web
```



## publish

```bash
npm publish ./pkg
```









