import React, { Component } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import "react-tabs/style/react-tabs.css";
import './App.css';

class App extends Component {
  render() {
    return (
        <div>
            <Tabs>
                <TabList>
                    <Tab>Game setup</Tab>
                    <Tab>Game info</Tab>
                    <Tab>Undo and history</Tab>
                    <Tab>Simulation</Tab>
                </TabList>
                <TabPanel>
                    <div>Setup</div>
                </TabPanel>
                <TabPanel>
                    <div>Info</div>
                </TabPanel>
                <TabPanel>
                    <div>History</div>
                </TabPanel>
                <TabPanel>
                    <div>Simulation</div>
                </TabPanel>
            </Tabs>
        </div>
      //<div className="App">
      //  <header className="App-header">
      //    <p>
      //      Edit <code>src/App.tsx</code> and save to reload.
      //    </p>
      //    <a
      //      className="App-link"
      //      href="https://reactjs.org"
      //      target="_blank"
      //      rel="noopener noreferrer"
      //    >
      //      Learn React
      //    </a>
      //  </header>
      //</div>
    );
  }
}

export default App;
