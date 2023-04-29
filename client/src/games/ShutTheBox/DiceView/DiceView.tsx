import React from 'react';


import {GameState, D6, nextD6} from '../GameState';

export type DiceViewProps = {
    gameState: GameState,
    setDie1: (pips: D6) => void,
    setDie2: (pips: D6) => void,
}

export function DiceView(props: DiceViewProps): JSX.Element {

    return <div className="dice">
        <DieView die={props.gameState.d1}
            setDie={props.setDie1}
        />
        <DieView die={props.gameState.d2}
            setDie={props.setDie2}
        />
    </div>
}

type DieViewProps = {
    die: D6,
    setDie: (pips: D6) => void,
}
function DieView(props: DieViewProps): JSX.Element {
    return <div className="die"
        onClick={() => props.setDie(nextD6(props.die))}
    >
        <span className="die_value">
            <PipsView die={props.die} />
        </span>
    </div>
}

type PipsViewProps = {
    die: D6,
}
function PipsView(props: PipsViewProps): JSX.Element {
    const PipView = getPipView(props.die);

    return <svg viewBox="0 0 100 100">
        <PipView r={8} />
    </svg>
}

function getPipView(die: D6): (props: PipViewProps) => JSX.Element {
    switch (die) {
        case D6.One: return OnePipView;
        case D6.Two: return TwoPipView;
        case D6.Three: return ThreePipView;
        case D6.Four: return FourPipView;
        case D6.Five: return FivePipView;
        case D6.Six: return SixPipView;
        default:
            const _exhaustiveCheck: never = die;
            return _exhaustiveCheck;
    }
}

type PipViewProps = {
    r: number,
}
function OnePipView(props: PipViewProps): JSX.Element {
    return <circle cx="50" cy="50" r={props.r.toString()}/>
}

function TwoPipView(props: PipViewProps): JSX.Element {
    return <>
        <circle cx="30" cy="30" r={props.r.toString()}/>
        <circle cx="70" cy="70" r={props.r.toString()}/>
    </>
}

function ThreePipView(props: PipViewProps): JSX.Element {
    return <>
        <circle cx="30" cy="30" r={props.r.toString()}/>
        <circle cx="50" cy="50" r={props.r.toString()}/>
        <circle cx="70" cy="70" r={props.r.toString()}/>
    </>
}

function FourPipView(props: PipViewProps): JSX.Element {
    return <>
        <circle cx="30" cy="30" r={props.r.toString()}/>
        <circle cx="30" cy="70" r={props.r.toString()}/>
        <circle cx="70" cy="30" r={props.r.toString()}/>
        <circle cx="70" cy="70" r={props.r.toString()}/>
    </>
}

function FivePipView(props: PipViewProps): JSX.Element {
    return <>
        <circle cx="30" cy="30" r={props.r.toString()}/>
        <circle cx="30" cy="70" r={props.r.toString()}/>
        <circle cx="50" cy="50" r={props.r.toString()}/>
        <circle cx="70" cy="30" r={props.r.toString()}/>
        <circle cx="70" cy="70" r={props.r.toString()}/>
    </>
}

function SixPipView(props: PipViewProps): JSX.Element {
    return <>
        <circle cx="30" cy="30" r={props.r.toString()}/>
        <circle cx="30" cy="50" r={props.r.toString()}/>
        <circle cx="30" cy="70" r={props.r.toString()}/>
        <circle cx="70" cy="30" r={props.r.toString()}/>
        <circle cx="70" cy="50" r={props.r.toString()}/>
        <circle cx="70" cy="70" r={props.r.toString()}/>
    </>
}