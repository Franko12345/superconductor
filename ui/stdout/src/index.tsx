import 'bootstrap/dist/css/bootstrap.min.css';
import React from 'react';
import { createRoot } from 'react-dom/client';
import {createStore} from "redux";
import {initState, reducer, STUPID_INITIALIZER} from "./store";
import StdoutWindow from "./StdoutWindow";
import {Provider} from "react-redux";

// @ts-ignore
// const store = createStore(reducer, initState());
// store.dispatch({type:STUPID_INITIALIZER});
// ReactDOM.render(
//     <React.StrictMode>
//         <Provider store={store}>
//             <StdoutWindow webserverPort={1234} />
//         </Provider>
//     </React.StrictMode>,
//     document.getElementById("root")
// )

export function start(port: number) {
    // Why is preloaded state broken when this exact code functions in the main window?
    // god knows. Welcome to frontend
    // @ts-ignore
    const store = createStore(reducer, initState());
    store.dispatch({type:STUPID_INITIALIZER});

    const root = createRoot(document.getElementById("root")!);
    root.render(
        <React.StrictMode>
            <Provider store={store}>
                <StdoutWindow webserverPort={port} />
            </Provider>
        </React.StrictMode>
    );
}

// @ts-ignore
window.startapp = start;

