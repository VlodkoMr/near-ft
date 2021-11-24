<template>
  <div>
    <img src="../../public/blabla.png" alt="" width="150">
    <h1 class="mt-4">BLA BLA Token</h1>
    <p>
      Please SignIn using NEAR wallet to join BlaBla community:
    </p>
    <button @click="signIn()" class="btn btn-primary">Sign In</button>
  </div>
</template>

<script>
export default {
  name: 'SignIn',
  methods: {
    signIn() {
      window.wallet.requestSignIn(
          window.nearConfig.contractName,
          'NEAR NFT Test'
      );
    }
  }
}
</script>
