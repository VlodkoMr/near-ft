<template>
  <div>
    <h2 class="mb-3">
      Welcome, <b>{{ user.accountId }}</b>
      <button class="btn btn-outline-secondary sign-out" v-if="user" @click="signOut()">SIGN OUT</button>
    </h2>


    <div class="mt-5">
      <div v-if="tokenBalance > 0">
        <img src="../../public/blabla.png" alt="" width="150" class="mb-3">
        <h4>Your Balance: <b>{{ tokenBalance }} BLABLA</b></h4>

        <div class="text-left col-4 offset-4 mt-5">
          <h5 class="text-center">Transfer BlaBla Token</h5>
          <div class="mb-2">
            <label class="form-label mb-1">NEAR Address</label>
            <input type="text" required class="form-control" v-model="address">
          </div>
          <div class="mb-3">
            <label class="form-label mb-1">Amount</label>
            <input type="number" required min="0" class="form-control" v-model="amount">
          </div>
          <div class="row">
            <div class="col-6 d-grid">
              <button class="btn " :class="{ 'btn-primary': !canTransfer, 'btn-outline-info': canTransfer }" :disabled="canTransfer" @click="approve">
                1. Approve
              </button>
            </div>
            <div class="col-6 d-grid">
              <button class="btn " :class="{ 'btn-primary': canTransfer, 'btn-outline-info': !canTransfer }" :disabled="!canTransfer" @click="transfer">
                2. Transfer
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="tokenBalance == 0">
        <p>Your need some BLABLA tokens:</p>
        <button @click="claim()" class="btn btn-primary">Claim 100 BLABLA</button>
      </div>
    </div>

  </div>
</template>

<script>
import Big from 'big.js';

export default {
  name: 'Token',
  computed: {
    user() {
      return window.currentUser;
    },
    canTransfer() {
      return localStorage.getItem(this.address);
    }
  },
  data() {
    return {
      tokenBalance: 0,
      address: 'vlodkow.testnet',
      amount: '5'
    }
  },
  created() {
    this.loadMyBalance();
  },
  methods: {
    signOut() {
      window.wallet.signOut();
      window.location.replace(window.location.origin + window.location.pathname);
    },
    loadMyBalance() {
      window.contract.ft_balance_of({
        account_id: window.currentUser.accountId
      }).then((result) => {
        this.tokenBalance = result;
      });
    },
    async claim() {
      const GAS = Big(3).times(10 ** 13).toFixed();
      await window.contract.ft_mint({
        receiver_id: window.currentUser.accountId,
        amount: "100"
      }, GAS, "1000000000000000000000");
    },
    async approve() {
      localStorage.setItem(this.address, 'true');
      const GAS = Big(3).times(10 ** 13).toFixed();
      await window.contract.ft_mint({
        receiver_id: this.address,
        amount: "0"
      }, GAS, "1000000000000000000000");
    },
    async transfer() {
      if (window.currentUser.accountId === this.address) {
        alert("Error: Sender and receiver should be different");
        return;
      }
      if (parseInt(this.amount) > this.tokenBalance) {
        alert('Error: Not enough balance');
        return;
      }

      const GAS = Big(3).times(10 ** 13).toFixed();
      await window.contract.ft_transfer({
        receiver_id: this.address,
        amount: this.amount.toString()
      }, GAS, 1);
    },
  }
}
</script>

<style>
.text-left {
  text-align: left;
}

.sign-out {
  margin-left: 26px;
}
</style>
