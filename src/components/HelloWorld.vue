<template>
  <section class="el-container">
    <aside class="el-aside" style="width: 200px">Aside</aside>
    <section class="el-container is-vertical">
      <header class="el-header" style="height: 60px">Header</header>
      <main class="el-main">Main</main>
    </section>
  </section>
</template>

<script>
export default {
  name: "HelloWorld",
  props: {
    msg: String,
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
