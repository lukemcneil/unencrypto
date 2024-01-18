<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { PlayerData } from '$lib/datatypes/playerdata';
	import { postClues } from '$lib/functions/requests';

	export let setRoundState: (new_state: string) => void;
	export let name: string;
	export let game_name: string;
	export let team: string;
	export let role: string;

	export let red_words: Array<string> = [];
	export let red_correct: Array<number> = [];

	export let blue_words: Array<string> = [];
	export let blue_correct: Array<number> = [];

	function onSubmitClues() {
		let my_clues: Array<string>;
		if (team == 'Red') {
			my_clues = red_words;
		} else {
			my_clues = blue_words;
		}
		const response: Promise<Response> = postClues(
			game_name,
			new PlayerData(name, team, role),
			my_clues
		);
		response.then((response) => {
			if (response.ok) {
				setRoundState('guess');
			}
		});
	}
</script>

{#each [0, 1, 2] as i}
	{#if team == 'Red'}
		{#if role == 'Encryptor'}
			<tr>
				<td><InputField bind:value={red_words[i]} text={'enter word here'} /></td>
				<td></td>
				<td>{red_correct[i]}</td>
				<td></td>
				<td></td>
				<td></td>
			</tr>
		{:else}
			<tr>
				<td></td>
				<td></td>
				<td></td>
				<td></td>
				<td></td>
				<td></td>
			</tr>
		{/if}
	{:else if role == 'Encryptor'}
		<tr>
			<td></td>
			<td></td>
			<td></td>
			<td><InputField bind:value={blue_words[i]} text={'enter word here'} /></td>
			<td></td>
			<td>{blue_correct[i]}</td>
		</tr>
	{:else}
		<tr>
			<td></td>
			<td></td>
			<td></td>
			<td></td>
			<td></td>
			<td></td>
		</tr>
	{/if}
{/each}

{#if role == 'Encryptor'}
	<Button text={'Submit Clues'} onClick={onSubmitClues} />
{/if}

<style>
	@import 'GuessCard.css';
</style>
