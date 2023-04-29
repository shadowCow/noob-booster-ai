import React from 'react';
import { Action, GameState } from '../GameState';
import { GameAnalysisService } from '../services/GameAnalysisService/GameAnalysisService';


export type GameAnalysisViewProps = {
    gameState: GameState,
    gameAnalysisService: GameAnalysisService,
}
export function GameAnalysisView(props: GameAnalysisViewProps): JSX.Element {
    const [bestAction, setBestAction] = React.useState<Action | undefined>(undefined);

    React.useEffect(() => {
        props.gameAnalysisService.findBestAction(props.gameState)
            .then(maybeAction => {
                setBestAction(maybeAction);
            })
            .catch(err => {
                setBestAction(undefined);
                console.error(err);
            })
    }, [props.gameState]);

    return <div className="game_analysis_view">
        <h3>Analysis</h3>
        <BestActionView bestAction={bestAction} />
    </div>
}

type BestActionViewProps = {
    bestAction: Action | undefined,
}
function BestActionView(props: BestActionViewProps): JSX.Element {
    const content = props.bestAction
        ? <ActionView action={props.bestAction} />
        : "No legal actions";

    return <div>
        {content}
    </div>
}

type ActionViewProps = {
    action: Action,
}
function ActionView(props: ActionViewProps): JSX.Element {
    return <div>
        {`Close ${props.action.join(', ')}`}
    </div>
}
