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
		} else if (!other_team_round.other_team_guess && other_team_round.clues) {
			return 'guess_other';
		} else if (!other_team_round.other_team_guess && !other_team_round.clues) {
			return 'waiting';
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
	<table class="center">
		<tr class="{other_team}Card">
			<td>Clue</td>
			<td>Our Guess</td>
			<td>Code</td>
		</tr>
		{#each other_team_round.code as c, i}
			<tr>
				<td>{other_team_round.clues ? other_team_round.clues[i] : ''}</td>
				<td>
					{#if state == 'guess_other'}
						<select bind:value={guess[i]}>
							{#each [1, 2, 3, 4] as o}
								<option value={o}>{o}</option>
							{/each}
						</select>
					{:else}
						{other_team_round.other_team_guess ? other_team_round.other_team_guess[i] : ''}
					{/if}
				</td>
				<td>{!is_active_round || state == 'done' ? c : '?'}</td>
			</tr>
		{/each}
		{#if state == 'guess_other'}
			<Button text={'Submit Guess'} onClick={onSubmitGuessOther} />
		{/if}
	</table>
	{#if state == 'done' && is_active_round}
		<h3>Waiting for other team</h3>
	{/if}
{/if}

<style>
	@import 'GuessCard.css';
</style>
