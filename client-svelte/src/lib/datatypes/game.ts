import { Player } from "./player";
import type { PlayerData } from "./playerdata";
import { Round } from "./round";

export class Game {
    players: Map<string, PlayerData> = new Map();
    red_words: Array<string> = [];
    blue_words: Array<string> = [];
    rounds: Array<Round> = [];

    // constructor(names: Array<string>, rounds: Array<object>) {
    //     this.players = new Array();
    //     names.forEach(name => {
    //        this.players.push(new Player(name)); 
    //     });

    //     this.rounds = new Array();
    //     rounds.forEach(round => {
    //         this.rounds.push(new Round(round.question))
    //     })
    // }
}