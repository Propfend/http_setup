import React from 'react';
import { Route, Router, Switch } from 'wouter';
import AppLayout from './AppLayout';
import Home from './Home';

export default function App() {
    return (
        <Router>
            <AppLayout>
              <Switch>
                <Route path="/">
                  <Home></Home>
                </Route>
              </Switch>
            </AppLayout>
        </Router>
    );
}
