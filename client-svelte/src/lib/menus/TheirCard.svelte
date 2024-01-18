<script lang="ts">
	import Button from '$lib/Button.svelte';
	import type { OneTeamRound } from '$lib/datatypes/one_team_round';
	import { PlayerData } from '$lib/datatypes/playerdata';
	import { postGuessOther } from '$lib/functions/requests';

	export let team: string;
	export let role: string;
	export let team_round: OneTeamRound;
	export let other_team_round: OneTeamRound;
	export let is_active_round: boolean;
	export let game_name: string;
	export let name: string;

	let guess: Array<number> = [1, 1, 1];

	$: other_team = team == 'Red' ? 'Blue' : 'Red';

	function getState(team_round: OneTeamRound): string {
		if (!team_round.clues) {
			return 'clues';
		} else if (!team_round.own_team_guess) {
			return 'guess';
		} else if (!other_team_round.other_team_guess) {
			return 'guess_other';
		} else {
			return 'done';
		}
	}

	$: state = getState(team_round);

	function onSubmitGuessOther() {
		const response: Promise<Response> = postGuessOther(
			game_name,
			new PlayerData(name, team, role),
			guess
		);
	}
</script>

{#if other_team_round}
	<table class="{other_team}Card">
		<tr>
			<td>Clue</td>
			<td>Our Guess</td>
			<td>Code</td>
		</tr>
		{#each other_team_round.code as c, i}
			<tr>
				<td>{other_team_round.clues ? other_team_round.clues[i] : ''}</td>
				<td>
					{#if state == 'guess_other'}
						<input type="number" bind:value={guess[i]} placeholder="guess" min="2" max="4" />
					{:else}
						{other_team_round.other_team_guess ? other_team_round.other_team_guess[i] : ''}
					{/if}
				</td>
				<td>{!is_active_round ? c : '?'}</td>
			</tr>
		{/each}
		{#if state == 'guess_other'}
			<Button text={'Submit Guess'} onClick={onSubmitGuessOther} />
		{/if}
	</table>
{/if}

<style>
	@import 'GuessCard.css';
</style>
