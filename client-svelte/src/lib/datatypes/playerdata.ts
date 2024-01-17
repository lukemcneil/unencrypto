export class PlayerData {
    player: string;
    team: string;
    role: string;

    constructor(player: string, team: string, role: string){
        this.player = player;
        this.team = team;
        this.role = role;
    }
}