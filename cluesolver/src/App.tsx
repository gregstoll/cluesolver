import React, { Component, ChangeEvent } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import "react-tabs/style/react-tabs.css";
import './App.css';
import { isNullOrUndefined } from 'util';

//TODO - for dev only
const SCRIPT_NAME = "https://gregstoll.dyndns.org/cluesolver/clue.cgi";
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
  for (let i = 0; i < _INTERNAL_NAMES.length; ++i) {
      if (_INTERNAL_NAMES[i].indexOf(card) != -1) {
          return i;
      }
  }
  return 10;
}
interface CardIndex {
    card_type: number,
    index: number
}
//TODO - make this an enum
const CARD_TYPE_NAMES = ["Suspects", "Weapons", "Rooms"];
//TODO - use CardIndex
interface CardName {
    card_index: CardIndex,
    internal: string,
    external: string
}
let CARD_NAMES : Array<Array<CardName>> = [];
for (let i = 0; i < _INTERNAL_NAMES.length; ++i) {
  CARD_NAMES.push([]);
  for (let j = 0; j < _INTERNAL_NAMES[i].length; ++j) {
      let card_index: CardIndex = { card_type: i, index: j };
      CARD_NAMES[i].push({card_index: card_index, internal: _INTERNAL_NAMES[i][j], external: _EXTERNAL_NAMES[i][j]});
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
    handleChange = () => {
        this.props.setNumberOfPlayers(this.props.thisNumberOfPlayers);
    }
    render = () => {
        let id = "numberOfPlayersInput" + this.props.thisNumberOfPlayers;
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
    render = () => {
        let options = [];
        for (let i = MIN_PLAYERS; i <= MAX_PLAYERS; ++i) {
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
    handleChange = (e: ChangeEvent<HTMLSelectElement>) => {
        this.props.setNumberOfCards(parseInt(e.target.value, 10));
    }
    render = () => {
        let options = [];
        for (let i = MIN_CARDS; i <= MAX_CARDS; i++) {
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
    setNumberOfCards = (numberOfCards: number) => {
        this.props.setNumberOfCards(this.props.index, numberOfCards);
    }
    setPlayerName = (e: ChangeEvent<HTMLInputElement>) => {
        this.props.setPlayerName(this.props.index, e.target.value);
    }
    render = () => {
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
    render = () => {
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
    render = () => {
        // TODO - do this better?
        let totalNumberOfCards = this.props.playerInfo.reduce(function (previousValue, currentValue) {
            return previousValue + currentValue.numberOfCards;
        }, 0);
        let badNumberOfCardsElem = null;
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
    session: string | null,
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
    handleRestartGame = () => {
        this.props.newSession();
    }
    handleLoadGameStringChange = (e: ChangeEvent<HTMLInputElement>) => {
        this.setState({loadGameString: e.target.value});
    }
    handleSetLoadGameString = () => {
        this.props.setGameString(this.state.loadGameString);
    }
    render = () => {
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

interface CardInfo {
    card_type: number,
    index: number,
    state: CardState,
    owners: Array<number>
}

interface HistorySuggestion {
    history_type: "suggestion",
    suggester_index: number,
    suspect_index: number,
    weapon_index: number,
    room_index: number,
    refuter_index: number,
    refuted_card_index: CardIndex | null
}

interface HistoryWhoOwns {
    history_type: "whoOwns",
    player_index: number,
    card_index: CardIndex
}

type HistoryEvent = HistorySuggestion | HistoryWhoOwns;
interface HistoryEntry {
    event: HistoryEvent,
    //TODO -describe
    session: string,
}

type SimulationData = Map<CardIndex, Array<number>>;

interface AppState {
    playerInfos: Array<PlayerInfo>,
    cardInfos: Array<Array<CardInfo>>,
    isConsistent: boolean,
    haveEnteredData: boolean,
    clauseInfo: Map<number, Array<Array<CardIndex>>>,
    history: Array<HistoryEntry>,
    working: boolean,
    simData: SimulationData,
    doingSimulation: boolean,
    numberOfSimulations: number,
    session: string | null
}

class App extends Component<{}, AppState> {
    constructor(props: {}) {
        // TODO - is this super() call right?
        super(props);
        // TODO - parse query hash from window.location.hash?
        let playerInfos: Array<PlayerInfo> = [];
        for (let i = 1; i <= MAX_PLAYERS; ++i) {
            playerInfos.push({ name: 'Player ' + i, numberOfCards: DEFAULT_CARDS});
        }
        let cardInfos : Array<Array<CardInfo>> = [];
        for (let i = 0; i < CARD_NAMES.length; ++i) {
            cardInfos.push([]);
            for (let j = 0; j < CARD_NAMES[i].length; ++j) {
                cardInfos[i].push({'card_type': i, 'index': j, 'state': CardState.Unknown, 'owners': []});
            }
        }

        this.state = {
            playerInfos: playerInfos,
            cardInfos: cardInfos,
            isConsistent: true,
            haveEnteredData: false,
            clauseInfo: new Map<number, Array<Array<CardIndex>>>(),
            history: [],
            working: false,
            simData: new Map<CardIndex, Array<number>>(),
            doingSimulation: false,
            numberOfSimulations: -1,
            session: null
        };
    }
    //TODO could be async
    sendClueRequest = (data: string, successCallback: (responseJson : any) => void, failureCallback: (message : string) => void, skipWorking?: boolean) => {
        if (!skipWorking) {
            this.setState({working: true});
        }
        let url = SCRIPT_NAME;
        if (!isNullOrUndefined(data) && data != "") {
            url += "?" + data;
        }
        let promise = fetch(url);
        promise
            .then(response => {
                if (!response.ok) {
                    failureCallback("Error doing request: " + response.statusText);
                    return;
                }
                response.json().then(data => {
                    successCallback(data);
                })
                .catch(error => {
                    failureCallback("Error getting JSON from request: " + error.toString())
                });
            })
            .catch(error => {
                failureCallback(error.message);
            })
            .finally(() => {
                if (!skipWorking) {
                    this.setState({working: false});
                }
            });
    }
    internalSetNumberOfPlayers = (previousPlayerInfo: Array<PlayerInfo>, numberOfPlayers: number) => {
        let playerInfo = previousPlayerInfo.slice(0, previousPlayerInfo.length);
        if (playerInfo.length == numberOfPlayers) {
            return playerInfo;
        }
        if (playerInfo.length > numberOfPlayers) {
            playerInfo = playerInfo.slice(0, numberOfPlayers);
        }
        else {
            while (playerInfo.length < numberOfPlayers) {
                // use DEFAULT_CARDS here, will fix afterwards
                playerInfo.push({ name: 'Player ' + (playerInfo.length + 1), numberOfCards: DEFAULT_CARDS });
            }
        }
        const baseCards = Math.floor(TOTAL_CARDS_FOR_PLAYERS / numberOfPlayers);
        const numWhoGetExtra = (TOTAL_CARDS_FOR_PLAYERS - baseCards * numberOfPlayers) % numberOfPlayers;
        for (let i = 0; i < playerInfo.length; ++i) {
            let numCards = baseCards;
            if (i < numWhoGetExtra) {
                numCards++;
            }
            playerInfo[i].numberOfCards = numCards;
        }
        return playerInfo;
    }

    cardIndexFromInternalName = (name: string) : CardIndex => {
        // TODO - optimize this
        for (let i = 0; i < CARD_NAMES.length; ++i) {
            for (let j = 0; j < CARD_NAMES[i].length; ++j) {
                if (CARD_NAMES[i][j].internal === name) {
                    return { card_type: i, index: j };
                }
            }
        }
        return { card_type: -1, index: -1 };
    }

    updateInfoFromJson = (json: any, haveEnteredData: boolean) => {
        let playerInfos = this.state.playerInfos;
        if (json.numPlayers) {
            playerInfos = this.internalSetNumberOfPlayers(this.state.playerInfos, json.numPlayers);
            for (let i = 0; i < json.numPlayers; ++i) {
                playerInfos[i].numberOfCards = json.numCards[i];
            }
        }
        let totalCards = playerInfos.reduce(function (previousValue: number, currentValue) {
            return previousValue + currentValue.numberOfCards;
        }, 0);

        let jsonClauseInfo = json.clauseInfo;
        let clauseInfo = new Map<number, Array<Array<CardIndex>>>();
        if (jsonClauseInfo) {
            for (let playerIndex in jsonClauseInfo) {
                //TODO - is this casting right?
                let playerIndexNumber: number = (playerIndex as unknown) as number;
                let newClauses : Array<Array<CardIndex>>= [];
                for (let i = 0; i < jsonClauseInfo[playerIndex].length; ++i) {
                    let clause : Array<CardIndex> = [];
                    for (let j = 0; j < jsonClauseInfo[playerIndex][i].length; ++j) {
                        clause.push(this.cardIndexFromInternalName(jsonClauseInfo[playerIndex][i][j]));
                    }
                    newClauses.push(clause);
                }
                clauseInfo.set(playerIndexNumber, newClauses);
            }
        }
        let newInfo = json.newInfo;
        //let newCardInfo = $.extend(true, {}, this.state.cardInfo);
        let newCardInfos = this.state.cardInfos;
        for (let i = 0; i < newInfo.length; ++i) {
            let cardIndex = this.cardIndexFromInternalName(newInfo[i].card);
            newCardInfos[cardIndex.card_type][cardIndex.index].state = newInfo[i].status;
            newCardInfos[cardIndex.card_type][cardIndex.index].owners = newInfo[i].owner;
        }
        this.setState({
            cardInfos: newCardInfos,
            playerInfos: playerInfos,
            session: json.session,
            clauseInfo: clauseInfo,
            isConsistent: json.isConsistent && totalCards == TOTAL_CARDS_FOR_PLAYERS,
            haveEnteredData: haveEnteredData
        });
    }

    updateCardInfo = (session: string, haveEnteredData: boolean, callback?: () => void) => {
        const data = "sess=" + session + "&action=fullInfo";
        let myApp = this;
        this.sendClueRequest(data, function(json) {
            if (callback) {
                callback();
            }
            myApp.updateInfoFromJson(json, haveEnteredData);
        }, function(errorText) {
            alert('Error: ' + errorText);
        });
    }

    newSession = () => {
        let data = "action=new&players=" + this.state.playerInfos.length;
        for (let i = 0; i < this.state.playerInfos.length; ++i) {
            data += "&numCards" + i + "=" + this.state.playerInfos[i].numberOfCards;
        }
        let myApp = this;
        this.sendClueRequest(data, function(json) {
            myApp.setState({'session': json.session, 'history': []});
            myApp.updateCardInfo(json.session, false);
        }, function(errorText) {
            alert('Error: ' + errorText);
        });
    }
    setNumberOfCards = (playerIndex: number, numberOfCards: number) => {
        let that = this;
        this.setState((previousState, currentProps) => {
            let playerInfos = previousState.playerInfos.slice(0, previousState.playerInfos.length);
            playerInfos[playerIndex].numberOfCards = numberOfCards;
            return { playerInfos: playerInfos };
        }, () => {
            // TODO - should be using a componentDidUpdate() callback instead? see https://reactjs.org/docs/react-component.html#setstate
            that.newSession();
        });
    }

    setNumberOfPlayers = (numberOfPlayers: number) => {
        let that = this;
        this.setState((previousState, currentProps) => {
            return {playerInfos: this.internalSetNumberOfPlayers(previousState.playerInfos, numberOfPlayers)};
        }, () => {
            // TODO - should be using a componentDidUpdate() callback instead? see https://reactjs.org/docs/react-component.html#setstate
            that.newSession();
        });
    }
    setPlayerName = (playerIndex: number, playerName: string) => {
        this.setState((previousState, currentProps) => {
            let playerInfos = previousState.playerInfos.slice(0, previousState.playerInfos.length);
            playerInfos[playerIndex].name = playerName;
            return {playerInfos: playerInfos};
        });
    }
    setGameString = (s: string) => {
        this.setState({session: s});
        this.updateCardInfo(s, true);
    }

    componentDidMount = () => {
        this.newSession();
    }

    render = () => {
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
                        <GameSetup
                            playerInfos={this.state.playerInfos}
                            setNumberOfPlayers={this.setNumberOfPlayers}
                            setNumberOfCards={this.setNumberOfCards}
                            setPlayerName={this.setPlayerName}
                            session={this.state.session}
                            haveEnteredData={this.state.haveEnteredData}
                            setGameString={this.setGameString}
                            newSession={this.newSession} />
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
