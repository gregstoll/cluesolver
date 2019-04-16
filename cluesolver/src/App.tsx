import React, { Component } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import "react-tabs/style/react-tabs.css";
import './App.css';

const SCRIPT_NAME = "clue.cgi";
const MIN_PLAYERS = 3;
const MAX_PLAYERS = 6;
const MIN_CARDS = 3;
const MAX_CARDS = 6;
const DEFAULT_CARDS = 3;
const TOTAL_CARDS_FOR_PLAYERS = 18;
const _INTERNAL_NAMES = [["ProfessorPlum", "ColonelMustard", "MrGreen", "MissScarlet", "MsWhite", "MrsPeacock"],
                    ["Knife", "Candlestick", "Revolver", "LeadPipe", "Rope", "Wrench"],
                    ["Hall", "Conservatory", "DiningRoom", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "BilliardRoom"]];
const _EXTERNAL_NAMES = [["Professor Plum", "Colonel Mustard", "Mr. Green", "Miss Scarlet", "Ms. White", "Mrs. Peacock"],
                    ["Knife", "Candlestick", "Revolver", "Lead Pipe", "Rope", "Wrench"],
                    ["Hall", "Conservatory", "Dining Room", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "Billiard Room"]];
function compareInternalCardsByCategory(card1: string, card2: string) {
  return categoryFromInternalCard(card1) - categoryFromInternalCard(card2);
}
function categoryFromInternalCard(card: string) {
  for (var i = 0; i < _INTERNAL_NAMES.length; ++i) {
      if (_INTERNAL_NAMES[i].indexOf(card) != -1) {
          return i;
      }
  }
  return 10;
}
const CARD_TYPE_NAMES = ["Suspects", "Weapons", "Rooms"];
interface CardName {
    card_type: number,
    index: number,
    internal: string,
    external: string
}
let CARD_NAMES : Array<Array<CardName>> = [];
for (let i = 0; i < _INTERNAL_NAMES.length; ++i) {
  CARD_NAMES.push([]);
  for (let j = 0; j < _INTERNAL_NAMES[i].length; ++j) {
      CARD_NAMES[i].push({'card_type': i, 'index': j, 'internal': _INTERNAL_NAMES[i][j], 'external': _EXTERNAL_NAMES[i][j]});
  }
}

enum CardState {
    Unknown = 0,
    OwnedByPlayer = 1,
    OwnedByCasefile = 2
}


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
