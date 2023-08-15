import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import reportWebVitals from './reportWebVitals';
import Wallet from './utils/near-wallet';

const CONTRACT_ID = process.env.REACT_APP_CONTRACT_NAME;

console.log(CONTRACT_ID)

const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ID })

wallet.startUp().then(
  isSignedIn => {
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(
      <React.StrictMode>
        <App wallet={wallet} isSignedIn={isSignedIn}/>
      </React.StrictMode>
    );
  }
)



// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
