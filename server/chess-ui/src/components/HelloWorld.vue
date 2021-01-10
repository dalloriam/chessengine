<template>
  <div class="hello">
    <div id="myBoard" class="merida chessground small"></div>
  </div>
</template>

<script>
const Chessground = require("chessground").Chessground;

import { API } from "../service/api";

export default {
  name: "HelloWorld",
  props: {},
  data: () => ({
    board: null,
    api: new API(),
    position: "",
  }),
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const initialPos = await this.api.getPosition();
      this.position = initialPos;
      console.log("Init pos: ", initialPos);
      const config = {
        fen: initialPos,
        events: { move: this.onMove },
      };

      this.board = Chessground(document.getElementById("myBoard"), config);
    },

    setPosition(fen) {
      this.position = fen;
      const config = { fen: fen, highlight: { lastMove: true } };
      this.board.set(config);
    },
    async onMove(orig, dest) {
      const newPos = await this.api.move(orig, dest);
      if (!newPos.position) {
        console.log(newPos.error);
        this.setPosition(this.position);
      } else {
        this.setPosition(newPos.position);
      }
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#myBoard {
  border: 1px solid red;
  min-width: 400px;
  min-height: 400px;
}
</style>
