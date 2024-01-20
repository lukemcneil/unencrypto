<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { putCreateGame, postJoinGame } from '$lib/functions/requests';

	export let setGameState: (new_state: string) => void;
	let name: string;
	let game_name: string;

	let team_color: string = 'Red';
	let role: string = 'Encryptor';

	async function onClickCreateGame() {
		if (name == '') {
			return;
		}
		if (game_name == '') {
			return;
		}
		const response: Promise<Response> = putCreateGame(game_name, name, team_color, role);
		response.then((response) => {
			if (response.ok) {
				localStorage.setItem('name', name);
				localStorage.setItem('game_name', game_name);
				localStorage.setItem('team', team_color);
				localStorage.setItem('role', role);
				setGameState('card');
			}
		});
	}

	async function onClickJoinGame() {
		if (name == '') {
			return;
		}
		if (game_name == '') {
			return;
		}
		const response: Promise<Response> = postJoinGame(game_name, name, team_color, role);
		response.then((response) => {
			if (response.ok) {
				localStorage.setItem('name', name);
				localStorage.setItem('game_name', game_name);
				localStorage.setItem('team', team_color);
				localStorage.setItem('role', role);
				setGameState('card');
			}
		});
	}
</script>

<main>
	<div>
		<InputField bind:value={name} text="enter your name" />
	</div>

	<div>
		<InputField bind:value={game_name} text="enter the game room" />
	</div>

	<h3>Choose Team Color</h3>
	<div>
		<label>
			<input type="radio" bind:group={team_color} value={'Red'} />
			Red Team
		</label>

		<label>
			<input type="radio" bind:group={team_color} value={'Blue'} />
			Blue Team
		</label>
	</div>

	<h3>Choose Role</h3>
	<div>
		<label>
			<input type="radio" bind:group={role} value={'Encryptor'} />
			Encryptor
		</label>

		<label>
			<input type="radio" bind:group={role} value={'Decryptor'} />
			Guesser
		</label>
	</div>

	<div>
		<Button text="Join Game" onClick={onClickJoinGame} />
	</div>

	<div>
		<Button text="Create Game" onClick={onClickCreateGame} />
	</div>
</main>

<style>
	@import '../../app.css';
</style>
