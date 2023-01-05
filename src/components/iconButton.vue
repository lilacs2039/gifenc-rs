<script setup lang="ts">
import { ref, reactive } from "vue";
// import {URL} from "url";

const props = defineProps({
 caption:String, //"no caption",
 icon:String, //"@/assets/img/download.png",
});

function iconUrl(relativePath:string){
  return new URL(relativePath, window.location.href).href
}

</script>

<template>
  <button class="icon-button shadow">
    <!-- <img :src="new URL(icon, import.meta.url).href" /> -->
    <img :src="iconUrl(icon)" />
    <div class="mask">{{caption}}</div>
  </button>
</template>

<style>
.icon-button {
  width:50px;
  height:40px;
  border: 2px solid #000;
  border-radius: 10px;
  overflow: hidden;
  position: relative; /* 相対位置指定 */
  background-color:var(--button-color);
}
.icon-button .mask {
  width: 100%;
  height: 100%;
  position: absolute; /* 絶対位置指定 */
  top: 0;
  left: 0;
  opacity: 0; /* マスクを表示しない */
  background-color: rgba(0, 0, 0, 0.4); /* マスクは半透明 */
  -webkit-transition: all 0.2s ease;
  transition: all 0.2s ease;
  text-align: center;
  /* padding-top: 80px; */
  color: #fff;
}
.icon-button:hover .mask {
  opacity: 1; /* マスクを表示する */
}

</style>
