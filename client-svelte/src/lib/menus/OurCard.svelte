<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { OneTeamRound } from '$lib/datatypes/one_team_round';
	import { PlayerData } from '$lib/datatypes/playerdata';
	import { postClues, postGuess } from '$lib/functions/requests';

	export let team: string;
	export let role: string;
	export let team_round: OneTeamRound;
	export let is_active_round: boolean;
	export let game_name: string;
	export let name: string;
	export let words: Array<String>;

	let clues: Array<string> = ['', '', ''];
	let guess: Array<number> = [1, 1, 1];

	function getState(team_round: OneTeamRound): string {
		if (!team_round.clues) {
			return 'clues';
		} else if (!team_round.own_team_guess) {
			return 'guess';
		} else {
			return 'done';
		}
	}

	$: state = getState(team_round);

	function onSubmitClues() {
		const response: Promise<Response> = postClues(
			game_name,
			new PlayerData(name, team, role),
			clues
		);
	}

	function onSubmitGuess() {
		const response: Promise<Response> = postGuess(
			game_name,
			new PlayerData(name, team, role),
			guess
		);
	}
</script>

{#if team_round}
	<table class="center">
		<tr class="{team}Card">
			<td>Clue</td>
			<td>Our Guess</td>
			<td>Code</td>
		</tr>
		{#each team_round.code as c, i}
			<tr>
				<td>
					{#if state == 'clues' && role == 'Encryptor'}
						<input bind:value={clues[i]} placeholder="clue" />
					{:else}
						{team_round.clues ? team_round.clues[i] : ''}
					{/if}
				</td>
				<td>
					{#if state == 'guess' && role == 'Decryptor'}
						<select bind:value={guess[i]}>
							{#each words as word, i}
								<option value={i + 1}>{i + 1} {word}</option>
							{/each}
						</select>
					{:else}
						{team_round.own_team_guess
							? team_round.own_team_guess[i].toString() +
								' ' +
								words[team_round.own_team_guess[i] - 1]
							: ''}
					{/if}
				</td>
				<td
					>{role == 'Encryptor' || state == 'done' || !is_active_round
						? c.toString() + ' ' + words[c - 1]
						: '?'}</td
				>
			</tr>
		{/each}
		{#if state == 'clues' && role == 'Encryptor'}
			<Button text={'Submit Clues'} onClick={onSubmitClues} />
		{/if}
		{#if state == 'guess' && role == 'Decryptor'}
			<Button text={'Submit Guess'} onClick={onSubmitGuess} />
		{/if}
	</table>
{/if}

<style>
	@import 'GuessCard.css';
</style>
