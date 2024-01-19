<script lang="ts">
	import { onMount } from 'svelte';
	import Join from '$lib/menus/Join.svelte';
	import Answer from '$lib/menus/Answer.svelte';
	import AnswerWait from '$lib/menus/AnswerWait.svelte';
	import Guess from '$lib/menus/Guess.svelte';
	import GuessWait from '$lib/menus/GuessWait.svelte';
	import Results from '$lib/menus/Results.svelte';
	import Card from '$lib/menus/Card.svelte';

	import { deleteGame, deletePlayerFromGame, getGame } from '$lib/functions/requests';
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';

	let game_state: string | null;

	let production_url: string = 'https://unencrypto.onrender.com/api/v1/game/';
	let test_url: string = 'http://0.0.0.0:8172/api/v1/game/';

	function setGameState(new_state: string) {
		localStorage.setItem('game_state', new_state);
		game_state = new_state;
	}

	onMount(() => {
		if (!localStorage.getItem('game_state')) {
			setGameState('join');
		} else {
			game_state = localStorage.getItem('game_state');
		}
		if (window.location.href == 'http://localhost:5173/') {
			localStorage.setItem('base_server_path', test_url);
		} else {
			localStorage.setItem('base_server_path', production_url);
		}
		if (localStorage.getItem('game_name')) {
			const response: Promise<Response> = getGame(localStorage.getItem('game_name'));
			response.then((response) => {
				if (!response.ok) {
					setGameState('join');
				}
			});
		}
	});

	function onLeave() {
		if (confirm('Do you really want to leave the game?') == true) {
			const response: Promise<Response> = deletePlayerFromGame(
				localStorage.getItem('game_name'),
				localStorage.getItem('name'),
				localStorage.getItem('team'),
				localStorage.getItem('role')
			);
			response.then((response) => {
				setGameState('join');
			});
		}
	}

	function onEndGame() {
		if (confirm('Do you want to end the game for everybody?')) {
			const response: Promise<Response> = deleteGame(localStorage.getItem('game_name'));
			response.then((response) => {
				setGameState('join');
			});
		}
	}
</script>

<main>
	<h1>Unencrypto</h1>
	{#if game_state == 'join'}
		<Join {setGameState} />
	{:else if game_state == 'card'}
		<Card
			{setGameState}
			name={localStorage.getItem('name')}
			game_name={localStorage.getItem('game_name')}
			team={localStorage.getItem('team')}
			role={localStorage.getItem('role')}
		/>
	{/if}
	<div style="padding: 50px;"></div>
	{#if game_state != 'join'}
		<div>
			<Button text="Leave Game" onClick={onLeave} />
		</div>
		<div>
			<Button text="End Game" onClick={onEndGame} />
		</div>
	{/if}
</main>

<style>
	@import '../app.css';
</style>
