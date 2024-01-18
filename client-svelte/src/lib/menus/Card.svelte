<script lang="ts">
	import { onMount } from 'svelte';
	import { getGame } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import OurCard from './OurCard.svelte';
	import TheirCard from './TheirCard.svelte';
	import type { PlayerData } from '$lib/datatypes/playerdata';
	import type { Game } from '$lib/datatypes/game';
	import type { OneTeamRound } from '$lib/datatypes/one_team_round';
	import { json } from '@sveltejs/kit';
	import type { Round } from '$lib/datatypes/round';

	let round_state: string | null;

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	export let team: string | null;
	export let role: string | null;

	let round_count: number;
	let players: Array<PlayerData> = [];
	let cards: Array<String> = [];
	let game: Game;
	let our_round: OneTeamRound;
	let their_round: OneTeamRound;

	function setRoundState(new_state: string) {
		localStorage.setItem('round_state', new_state);
		round_state = new_state;
	}

	function updateRoundState() {
		if (our_round.clues != null && their_round.clues != null) {
			setRoundState('guess');
		} else {
			setRoundState('clues');
		}
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				game = data;
				round_count = game.rounds.length - 1;

				players = [];
				for (var prop in game.players) {
					players = [...players, data.players[prop] as PlayerData];
				}

				if (team == 'Red') {
					cards = data.red_words;
					our_round = data.rounds[round_count].red_round;
					their_round = data.rounds[round_count].blue_round;
				} else {
					cards = data.blue_words;
					our_round = data.rounds[round_count].blue_round;
					their_round = data.rounds[round_count].red_round;
				}

				updateRoundState();
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
		setRoundState('clues');
	});

	function getOurRound(round: Round): OneTeamRound {
		if (team == 'Red') {
			return round.red_round;
		} else {
			return round.blue_round;
		}
	}

	function getTheirRound(round: Round): OneTeamRound {
		if (team == 'Blue') {
			return round.red_round;
		} else {
			return round.blue_round;
		}
	}
</script>

<main>
	<div>
		{#if game}
			{#each game.rounds as round, i}
				<h2>Round {i}</h2>
				<OurCard
					team={team || ''}
					role={role || ''}
					team_round={getOurRound(round)}
					is_active_round={i == game.rounds.length - 1}
					game_name={game_name || ''}
					name={name || ''}
				/>
				<TheirCard
					team={team || ''}
					role={role || ''}
					team_round={getOurRound(round)}
					other_team_round={getTheirRound(round)}
					is_active_round={i == game.rounds.length - 1}
					game_name={game_name || ''}
					name={name || ''}
				/>
			{/each}
		{/if}
	</div>
	<h2>Words</h2>
	<div>
		{#each cards as card}
			{card} &emsp;
		{/each}
	</div>
	<h2>Players</h2>
	{#each players as player}
		<div class={player.team}>
			{player.player}: {player.role}
		</div>
	{/each}
</main>

<style>
	@import '../../app.css';
	@import 'GuessCard.css';
	.Red {
		color: red;
	}
	.Blue {
		color: cornflowerblue;
	}
</style>
