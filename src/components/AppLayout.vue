<template>
  <section class="el-container">
    <aside class="el-aside" style="width: 64px">
      <NavMenu />
    </aside>
    <main class="el-main"><AppTabs /></main>
  </section>
</template>

<script>
import NavMenu from "./NavMenu.vue";
import AppTabs from "./AppTabs.vue";

export default {
  name: "AppMenu",
  components: {
    NavMenu,
    AppTabs,
  },
};
</script>


<style>
.el-main {
  padding: 0px 0px;
}
</style>