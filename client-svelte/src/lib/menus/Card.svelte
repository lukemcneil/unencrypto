<script lang="ts">
	import { onMount } from 'svelte';
	import { getGame, getScore } from '$lib/functions/requests';
	import { sleep } from '$lib/functions/helper';
	import OurCard from './OurCard.svelte';
	import TheirCard from './TheirCard.svelte';
	import type { PlayerData } from '$lib/datatypes/playerdata';
	import { Game } from '$lib/datatypes/game';
	import type { OneTeamRound } from '$lib/datatypes/one_team_round';
	import { json } from '@sveltejs/kit';
	import type { Round } from '$lib/datatypes/round';
	import type { Score } from '$lib/datatypes/score';

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
	let scores: Score;
	let our_words: Map<number, Array<String>> = new Map([
		[1, []],
		[2, []],
		[3, []],
		[4, []]
	]);
	let their_words: Map<number, Array<String>> = new Map([
		[1, []],
		[2, []],
		[3, []],
		[4, []]
	]);

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

	function getOtherTeam(my_team: string | null) {
		if (my_team == 'Red') {
			return 'Blue';
		} else {
			return 'Red';
		}
	}

	async function readGame() {
		getGame(game_name).then((response) => {
			if (response.ok) {
				response.json().then((data) => {
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

					our_words = new Map([
						[1, []],
						[2, []],
						[3, []],
						[4, []]
					]);
					their_words = new Map([
						[1, []],
						[2, []],
						[3, []],
						[4, []]
					]);
					game.rounds.forEach((round: Round) => {
						let our: OneTeamRound;
						let their: OneTeamRound;
						if (team == 'Red') {
							our = round.red_round;
							their = round.blue_round;
						} else {
							our = round.blue_round;
							their = round.red_round;
						}
						if (our.clues && our.own_team_guess) {
							our.code.forEach((x: number, i: number) => {
								let clue = our.clues?.at(i);
								if (clue) {
									our_words.get(x)?.push(clue);
								}
							});
						}

						if (their.clues && their.other_team_guess) {
							their.code.forEach((x: number, i: number) => {
								let clue = their.clues?.at(i);
								if (clue) {
									their_words.get(x)?.push(clue);
								}
							});
						}
					});

					updateRoundState();
				});
			}
		});

		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				scores = data;
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
				<h3>Round {i + 1}</h3>
				<div class="center">
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
				</div>
			{/each}
		{/if}
	</div>

	<h3>Words</h3>
	<div>
		{#if game}
			<div>
				<table class="center">
					<tr class="{team}Card">
						{#each cards as card}
							<td>{card}</td>
						{/each}
					</tr>
					<tr>
						{#each [1, 2, 3, 4] as count}
							<td>
								{#if our_words.get(count)}
									{#each our_words.get(count) as word}
										{word} <br />
									{/each}
								{/if}
							</td>
						{/each}
					</tr>
				</table>
			</div>
			<div>
				<table class="center">
					<tr class="{getOtherTeam(team)}Card">
						{#each cards as card, i}
							<td>{i + 1}</td>
						{/each}
					</tr>
					<tr>
						{#each [1, 2, 3, 4] as count}
							<td>
								{#if their_words.get(count)}
									{#each their_words.get(count) as word}
										{word} <br />
									{/each}
								{/if}
							</td>
						{/each}
					</tr>
				</table>
			</div>
		{/if}
	</div>

	<h3>Scores</h3>
	{#if game && scores}
		<table class="center">
			<tr>
				<td><b>Interceptions</b></td>
				<td><b>Miscommunications</b></td>
			</tr>
			<tr class="RedCard">
				<td>{scores.red_interceptions}</td>
				<td>{scores.red_miscommunications}</td>
			</tr>
			<tr class="BlueCard">
				<td>{scores.blue_interceptions}</td>
				<td>{scores.blue_miscommunications}</td>
			</tr>
		</table>
	{/if}

	<h3>Players</h3>
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
