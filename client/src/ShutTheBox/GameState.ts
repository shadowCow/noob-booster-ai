export enum D6 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

export type GameState = {
    d1: D6,
    d2: D6,
    tiles: Array<Tile>,
}

export function createGameState(): GameState {
    const tiles: GameState['tiles'] = [
        createTile(1),
        createTile(2),
        createTile(3),
        createTile(4),
        createTile(5),
        createTile(6),
        createTile(7),
        createTile(8),
        createTile(9),
    ];
    return {
        d1: D6.One,
        d2: D6.Two,
        tiles,
    }
}


export function pipsFor(die: D6): number {
    switch (die) {
        case D6.One: return 1;
        case D6.Two: return 2;
        case D6.Three: return 3;
        case D6.Four: return 4;
        case D6.Five: return 5;
        case D6.Six: return 6;
        default:
            const _exhaustiveCheck: never = die;
            return _exhaustiveCheck;
    }
}

export function nextD6(die: D6): D6 {
    switch (die) {
        case D6.One: return D6.Two;
        case D6.Two: return D6.Three;
        case D6.Three: return D6.Four;
        case D6.Four: return D6.Five;
        case D6.Five: return D6.Six;
        case D6.Six: return D6.One;
        default:
            const _exhaustiveCheck: never = die;
            return _exhaustiveCheck;
    }
}

export type Tile = {
    value: number,
    isOpen: boolean,
}

function createTile(value: number): Tile {
    return {
        value,
        isOpen: true,
    }
}

export type Action = Array<number>;