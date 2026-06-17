import 'bootstrap/dist/css/bootstrap.min.css';
import React from 'react';
import { createRoot } from 'react-dom/client';
import {Message, UpdateGSM} from "./ipc";
import {Provider} from "react-redux";
import {initState, rootReducer} from "./store";
import {createStore} from "redux";
import DriverStation from "./DriverStation";


// Uncomment this for dev server, comment again for rust integration
// @ts-ignore
// const store = createStore(rootReducer, initState());
//
// ReactDOM.render(
//     <React.StrictMode>
//         <Provider store={store}>
//             <DriverStation webserverPort={1234} />
//         </Provider>
//     </React.StrictMode>,
//     document.getElementById('root')
// );

export function start(port: number) {
    // @ts-ignore
    const store = createStore(rootReducer, initState());

    const root = createRoot(document.getElementById('root')!);
    root.render(
        <React.StrictMode>
            <Provider store={store}>
                <DriverStation webserverPort={port} />
            </Provider>
        </React.StrictMode>
    );
}

// @ts-ignore
window.startapp = start;

