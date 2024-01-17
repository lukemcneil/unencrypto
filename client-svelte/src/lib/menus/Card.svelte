<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { Player } from '$lib/datatypes/player';
	import { onMount } from 'svelte';
	import { getGame } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import GuessCardGuess from './GuessCardGuess.svelte';
	import GuessCardClue from './GuessCardClue.svelte';

	let round_state: string | null;

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	export let team: string | null;
	export let role: string | null;

	let round_count: number;
	let cards: Array<String> = [];
	let red_code: Array<number> = [];
	let blue_code: Array<number> = [];

	function setRoundState(new_state: string) {
		localStorage.setItem('round_state', new_state);
		round_state = new_state;
	}
	
	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				console.log(data);
				round_count = data.rounds.length - 1;
				if (team == 'Red') {
					cards = data.red_words;
				} else {
					cards = data.blue_words;
				}
				red_code = data.rounds[round_count].red_round.code;
				blue_code = data.rounds[round_count].blue_round.code;
			});
	}

	let get_game_interval_ms: number = 1000;
	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'card') {
			readGame();
			await sleep(get_game_interval_ms);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});
</script>

<main>
	<h2>
		Round: {round_count}
	</h2>
	<div>
		{#each cards as card}
			{card}
		{/each}
	</div>
	<div>
		<table class='center'>
			<GuessCardClue {setRoundState} {name} {game_name} {team} {role} red_correct={red_code} blue_correct={blue_code} />
		</table>
	</div>
</main>

<style>
	@import '../../app.css';
	@import 'GuessCard.css';
</style>
