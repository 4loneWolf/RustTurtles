<script>
	let name; let message;
	function sendTestReq() {
		fetch("/turtle", {
			method: 'POST',
			body: JSON.stringify({name: name, message: message}),
			headers: {
				'Content-Type': 'application/json'
			}
			})	
	}
</script>

<center>
	<div>
		<h1 class="title is-primary is-1"> Svelte On Rust 🦀 </h1>
		<h4 class="subtitle is-4"> A compiled frontend, deserves a compiled backend</h4>
		<ul>
			<li><a href="https://svelte.dev/"> Svelte JS</a> - Cybernetically enhanced web apps</li> 
			<li><a href="https://rocket.rs/"> Rocket.rs </a> - A web framework for Rust</li>
			<li><a href="https://github.com/pateketrueke/yrv"> YRV Router </a> - Your routing bro! </li>
			<li><a href="https://bulma.io"> Bulma CSS </a>- Modern CSS framework based on Flexbox </li>
			<button on:click={sendTestReq}>
				yeba
			</button>

			<input bind:value = {name} placeholder="Turtle name">
			<input bind:value = {message} placeholder="Message (JSON)">

		</ul>
	</div>
</center>

