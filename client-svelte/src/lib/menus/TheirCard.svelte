<script lang="ts">
	import type { OneTeamRound } from '$lib/datatypes/one_team_round';

	export let team: string;
	export let role: string;
	export let team_round: OneTeamRound;
	export let is_active_round: boolean;

	let clues: Array<string> = ['', '', ''];

	$: other_team = team == 'Red' ? 'Blue' : 'Red';
</script>

{#if team_round}
	<table class="{other_team}Card">
		<tr>
			<td>Clue</td>
			<td>Our Guess</td>
			<td>Code</td>
		</tr>
		{#each team_round.code as c, i}
			<tr>
				<td>{team_round.clues ? team_round.clues[i] : ''}</td>
				<td>{team_round.other_team_guess ? team_round.other_team_guess[i] : ''}</td>
				<td>{c}</td>
			</tr>
		{/each}
	</table>
{/if}

<style>
	@import 'GuessCard.css';
</style>
