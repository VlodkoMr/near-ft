<template>
  <div class="container">
    <SignIn v-if="!user"></SignIn>
    <Token v-if="user"></Token>
  </div>
</template>

<script>
import SignIn from '@/components/SignIn';
import Token from '@/components/Token';

export default {
  name: 'App',
  computed: {
    user() {
      return window.currentUser;
    }
  },
  components: {
    SignIn,
    Token
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
