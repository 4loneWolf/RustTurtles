<script>
  import { Canvas } from '@threlte/core'
  import Init from "../src/scene.js";
</script>
  
  <Canvas>
    {onMount(() => {
      Init()
    })}
  </Canvas>

  <body id="WASD">
    <script> 
        function over(number) {
            document.getElementById("button" + number).style.backgroundColor = "rgb(134, 134, 134)"
        }
        function leave(number) {
            document.getElementById("button" + number).style.backgroundColor = "rgb(156, 156, 156)"
        }
    </script>

    <button class="first" id="button1"></button>
    <button class="first" id="button2"></button>
    <button class="first" id="button3"></button>
    <button class="first" id="button4"></button>
    <button class="first" id="button5"></button>
    <button class="first" id="button6"></button>
    <button class="first" id="button7"></button>
    <button class="first" id="button8"></button>
    <button class="first" id="button9"></button>
    <button class="first" id="button10"></button>
    <button class="first" id="button11"></button>
    <button class="first" id="button12"></button>
    <button class="first" id="button13"></button>
    <button class="first" id="button14"></button>
    <button class="first" id="button15"></button>
    <button class="first" id="button16"></button>
    <button class="dropping" id="dropUp"></button>
    <button class="dropping" id="drop"></button>
    <button class="dropping" id="dropDown"></button>
    <button class="digging" id="dig"></button>
    <button class="digging" id="digUp"></button>
    <button class="digging" id="digDown"></button>
    <button class="placing" id="placeUp"></button>
    <button class="placing" id="place"></button>
    <button class="placing" id="placeDown"></button>
    <button class="sucking" id="suckUp"></button>
    <button class="sucking" id="suck"></button>
    <button class="sucking" id="suckDown"></button>

    <script type="module" src="/scene.js">init();</script>
</body>