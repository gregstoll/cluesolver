import React, { Component, ChangeEvent } from 'react';
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

interface NumberOfPlayerOptionProps {
    thisNumberOfPlayers: number,
    numberOfPlayers: number,
    allowChange: boolean,
    setNumberOfPlayers: (numberOfPlayers: number) => void
}

class NumberOfPlayerOption extends Component<NumberOfPlayerOptionProps, {}> {
    handleChange() {
        this.props.setNumberOfPlayers(this.props.thisNumberOfPlayers);
    }
    render() {
        var id = "numberOfPlayersInput" + this.props.thisNumberOfPlayers;
        return <span><input type="radio" name="numberOfPlayer" id={id} value="{this.props.thisNumberOfPlayers}" checked={this.props.thisNumberOfPlayers == this.props.numberOfPlayers} onChange={this.handleChange} disabled={!this.props.allowChange}/><label htmlFor={id}> {this.props.thisNumberOfPlayers}</label></span>;
    }
}

interface PlayerInfo {
    name: string,
    numberOfCards: number
}

interface NumberOfPlayersProps {
    playerInfos: Array<PlayerInfo>,
    allowChange: boolean,
    setNumberOfPlayers: (numberOfPlayers: number) => void
}

class NumberOfPlayers extends Component<NumberOfPlayersProps, {}> {
    render() {
        var options = [];
        for (var i = MIN_PLAYERS; i <= MAX_PLAYERS; ++i) {
            options.push(<NumberOfPlayerOption thisNumberOfPlayers={i} numberOfPlayers={this.props.playerInfos.length} key={i} setNumberOfPlayers={this.props.setNumberOfPlayers} allowChange={this.props.allowChange}/>);
        }
        return <div><span>Number of players:</span><form style={{display: 'inline'}}>{options}</form></div>;
    }
}

interface PlayerNumberOfCardsProps {
    allowChange: boolean,
    num: number,
    setNumberOfCards: (numberOfCards: number) => void
}

class PlayerNumberOfCards extends Component<PlayerNumberOfCardsProps, {}> {
    handleChange(e: ChangeEvent<HTMLSelectElement>) {
        this.props.setNumberOfCards(parseInt(e.target.value, 10));
    }
    render() {
        var options = [];
        for (var i = MIN_CARDS; i <= MAX_CARDS; i++) {
            options.push(<option value={i} key={i}>{i}</option>);
        }
        return <select onChange={this.handleChange} disabled={!this.props.allowChange} value={this.props.num}>{options}</select>;
    }
}

interface PlayerInfoComponentProps {
    index: number,
    info: PlayerInfo,
    allowChange: boolean,
    setNumberOfCards: (index: number, numberOfCards: number) => void,
    setPlayerName: (index: number, name: string) => void,
}

class PlayerInfoComponent extends Component<PlayerInfoComponentProps, {}> {
    setNumberOfCards(numberOfCards: number) {
        this.props.setNumberOfCards(this.props.index, numberOfCards);
    }
    setPlayerName(e: ChangeEvent<HTMLInputElement>) {
        this.props.setPlayerName(this.props.index, e.target.value);
    }
    render() {
        return <div>Name:<input type="text" value={this.props.info.name} onChange={this.setPlayerName} /> Number of cards:<PlayerNumberOfCards num={this.props.info.numberOfCards} setNumberOfCards={this.setNumberOfCards} allowChange={this.props.allowChange} /></div>;
    }
}

interface PlayerListProps {
    playerInfo: Array<PlayerInfo>,
    allowChange: boolean,
    setNumberOfCards: (index: number, numberOfCards: number) => void,
    setPlayerName: (index: number, name: string) => void,
}

class PlayerList extends Component<PlayerListProps, {}> {
    render() {
        let infos = [];
        for (let i = 0; i < this.props.playerInfo.length; ++i) {
            infos.push(<PlayerInfoComponent index={i} key={i} info={this.props.playerInfo[i]} setNumberOfCards={this.props.setNumberOfCards} setPlayerName={this.props.setPlayerName} allowChange={this.props.allowChange}/>);
        }
        return <div>{infos}</div>;
    }
}

interface NumberOfCardsValidatorProps {
    playerInfo: Array<PlayerInfo>
}

class NumberOfCardsValidator extends Component<NumberOfCardsValidatorProps, {}> {
    render() {
        // TODO - do this better?
        var totalNumberOfCards = this.props.playerInfo.reduce(function (previousValue, currentValue) {
            return previousValue + currentValue.numberOfCards;
        }, 0);
        var badNumberOfCardsElem = null;
        if (totalNumberOfCards != TOTAL_CARDS_FOR_PLAYERS) {
            badNumberOfCardsElem = <span className="warning">Total number of cards must total {TOTAL_CARDS_FOR_PLAYERS}! (not {totalNumberOfCards})</span>;
        }
        return <div>{badNumberOfCardsElem}</div>;
    }
}

interface GameSetupProps {
    newSession: () => void,
    setGameString: (gameString: string) => void,
    playerInfos: Array<PlayerInfo>,
    haveEnteredData: boolean,
    session: string,
    setNumberOfCards: (index: number, numberOfCards: number) => void,
    setPlayerName: (index: number, name: string) => void,
    setNumberOfPlayers: (numberOfPlayers: number) => void
}

interface GameSetupState {
    loadGameString: string
}

class GameSetup extends Component<GameSetupProps, GameSetupState> {
    constructor(props: GameSetupProps) {
        super(props);
        this.state = {
            loadGameString: ""
        }
    }
    handleRestartGame() {
        this.props.newSession();
    }
    handleLoadGameStringChange(e: ChangeEvent<HTMLInputElement>) {
        this.setState({loadGameString: e.target.value});
    }
    handleSetLoadGameString() {
        this.props.setGameString(this.state.loadGameString);
    }
    render() {
        return <div>
            <NumberOfPlayers playerInfos={this.props.playerInfos} setNumberOfPlayers={this.props.setNumberOfPlayers} allowChange={!this.props.haveEnteredData}/>
            <PlayerList playerInfo={this.props.playerInfos} setNumberOfCards={this.props.setNumberOfCards} setPlayerName={this.props.setPlayerName} allowChange={!this.props.haveEnteredData}/>
            <NumberOfCardsValidator playerInfo={this.props.playerInfos} />
            <button type="button" onClick={this.handleRestartGame} disabled={!this.props.haveEnteredData}>Restart game</button>
            <div>Current game state (for loading later): {this.props.session}</div>
            <div>Load game state: <input type="text" onChange={this.handleLoadGameStringChange} value={this.state.loadGameString}/></div>
            <button type="button" onClick={this.handleSetLoadGameString}>Load game</button>
          </div>;
    }
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
