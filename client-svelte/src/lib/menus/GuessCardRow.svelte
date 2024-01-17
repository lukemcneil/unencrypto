<script lang="ts">
	export let red_words: Array<String> = [];
	export let red_guesses: Array<number> = [];
	export let red_correct: Array<number> = [];

	export let blue_words: Array<String> = [];
	export let blue_guesses: Array<number> = [];
	export let blue_correct: Array<number> = [];
</script>

{#each [0, 1, 2] as i}
	<tr>
		<td>{red_words[i]}</td>
		<td>{red_guesses[i]}</td>
		<td>{red_correct[i]}</td>
		<td>{blue_words[i]}</td>
		<td>{blue_guesses[i]}</td>
		<td>{blue_correct[i]}</td>
	</tr>
{/each}
