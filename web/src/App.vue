<script setup lang="ts">
import { ref, reactive, inject } from "vue";
import encode_gif_init from "./wasm/gif_encoder";
import { encode_gif } from "./wasm/gif_encoder";
import imageInput from "./components/image-input.vue";

const inputB64 = ref("");
const url = ref("");
const post_thumb_mes = ref("");

// init wasm
encode_gif_init("wasm/gif_encoder_bg.wasm");

function encode(dataURL: string) {
  const bin = window.atob(dataURL.replace(/.+,/, ""));
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

<template>
  <div>
    <h2>0. Sample Image</h2>
    <div>
      <img
        alt="Vue logo"
        class="logo"
        src="./assets/logo.svg"
        width="125"
        height="125"
      />
    </div>
    <pre></pre>
    <h2>1. Input Image</h2>
    <imageInput @onChanged="(dataURL) => encode(dataURL)" />
    <pre></pre>
    <h2>2. Converted GIF Image</h2>
    <div>{{ post_thumb_mes }}</div>
    <img :src="url" />
  </div>
</template>

<style scoped>
</style>
