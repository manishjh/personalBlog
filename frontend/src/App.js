import './App.css';
import React from 'react';
import { ReactDOM } from 'react-dom';
import { Provider } from 'react-redux';

import AppRouter from './routers/AppRouter';
import store from './store/configureStore';

const App = () => (
  <Provider store={store}>
    <AppRouter />
  </Provider>
);

export default App;
