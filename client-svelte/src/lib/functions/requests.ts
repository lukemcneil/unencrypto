import type { Guess } from '$lib/datatypes/guess';
import type { PlayerData } from '$lib/datatypes/playerdata';

function getBaseServerPath(): string | null {
	return localStorage.getItem('base_server_path');
}

export async function putCreateGame(game_name: string, name: string, team: string, role: string) {
	const response: Response = await fetch(localStorage.getItem('base_server_path') + game_name, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			player: name,
			team: team,
			role: role
		})
	});
	return response;
}

export async function postJoinGame(game_name: string, name: string, team: string, role: string) {
	const request = await fetch(localStorage.getItem('base_server_path') + game_name, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			player: name,
			team: team,
			role: role
		})
	});
	return request;
}

export async function getGame(game_name: string | null) {
	const response: Response = await fetch(getBaseServerPath() + game_name, {
		method: 'GET',
		headers: { 'Content-Type': 'application/json' }
	});
	return response;
}

export async function postClues(game_name: string | null, playerData: PlayerData, clues: Array<string>) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/clues', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			clues: clues,
			player: playerData,
		})
	});
	return response;
}

export async function deletePlayerFromGame(game_name: string | null, name: string | null) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/exit', {
		method: 'DELETE',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			player: name
		})
	});
	return response;
}

export async function deleteGame(game_name: string | null) {
	const response: Response = await fetch(getBaseServerPath() + game_name, {
		method: 'DELETE',
		headers: { 'Content-Type': 'application/json' }
	});
	return response;
}

export async function postGuess(game_name: string | null, playerData: PlayerData, guess: Array<number>) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/guess_own_team', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			guess: guess,
			player: playerData,
		})
	});
	return response;
}

export async function postGuessOther(game_name: string | null, playerData: PlayerData, guess: Array<number>) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/guess_other_team', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			guess: guess,
			player: playerData,
		})
	});
	return response;
}

export async function getScore(game_name: string | null) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/score', {
		method: 'GET',
		headers: { 'Content-Type': 'application/json' }
	});
	return response;
}

export async function postChangeQuestion(game_name: string | null) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/change_question', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' }
	});
	return response;
}

export async function postChatGptQuestion(game_name: string | null, prompt: string) {
	const response: Response = await fetch(getBaseServerPath() + game_name + '/chat_gpt_question', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			prompt: prompt
		})
	});
	return response;
}
