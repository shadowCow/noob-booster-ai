import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import reportWebVitals from '../reportWebVitals';
import { App } from './App';
import { createGameAnalysisServiceImpl } from './services/GameAnalysisService/GameAnalysisService.impl';

const appDependencies = {
  gameAnalysisService: createGameAnalysisServiceImpl(),
};

ReactDOM.render(
  <React.StrictMode>
    <App appDependencies={appDependencies}/>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
