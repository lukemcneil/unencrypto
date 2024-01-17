<script lang="ts">
	import InputField from '$lib/InputField.svelte';

	export let team: string | null;
	export let role: string | null;

	export let red_words: Array<string> = [];
	export let red_guesses: Array<string> = [];
	export let red_correct: Array<number> = [];

	export let blue_words: Array<string> = [];
	export let blue_guesses: Array<string> = [];
	export let blue_correct: Array<number> = [];
</script>

{#each [0, 1, 2] as i}
	{#if team == 'Red'}
		{#if role == 'Encryptor'}
			<tr>
				<td>{red_words[i]}</td>
				<td></td>
				<td>{red_correct[i]}</td>
				<td>{blue_words[i]}</td>
				<td></td>
				<td></td>
			</tr>
		{:else}
			<tr>
				<td>{red_words[i]}</td>
				<td><InputField bind:value={red_guesses[i]} text={'enter here'} /></td>
				<td></td>
				<td>{blue_words[i]}</td>
				<td></td>
				<td></td>
			</tr>
		{/if}
	{:else if role == 'Encryptor'}
		<tr>
			<td>{red_words[i]}</td>
			<td></td>
			<td></td>
			<td>{blue_words[i]}</td>
			<td></td>
			<td>{blue_correct[i]}</td>
		</tr>
	{:else}
		<tr>
			<td>{red_words[i]}</td>
			<td></td>
			<td></td>
			<td>{blue_words[i]}</td>
			<td><InputField bind:value={blue_guesses[i]} text={'enter here'} /></td>
			<td></td>
		</tr>
	{/if}
{/each}
