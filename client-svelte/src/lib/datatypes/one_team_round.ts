import { Player } from "./player"
import { Answer } from "./answer"
import { Guess } from "./guess"

export class OneTeamRound {
    code: Array<number> = [];
    clues: Array<String> | null = [];
    own_team_guess: Array<number> | null = [];
    other_team_guess: Array<number> | null = [];
}