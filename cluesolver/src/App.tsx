import React, { Component, ChangeEvent } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import "react-tabs/style/react-tabs.css";
import './App.css';
import { isNullOrUndefined, isNull } from 'util';

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
function compareCardIndexByCategory(card1: CardIndex, card2: CardIndex) {
    return card1.card_type - card2.card_type;
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
//TODO - refactor this into a method that takes a CardIndex or something
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

interface SpecificCardInfoProps {
    //TODO - rename to cardInfo
    info: CardInfo,
    playerInfos: Array<PlayerInfo>
}

class SpecificCardInfo extends React.Component<SpecificCardInfoProps, {}> {
    getImgSrc = () => {
        let name : string;
        switch (this.props.info.state) {
            case CardState.OwnedByPlayer:
                name = 'cancel.png';
                break;
            case CardState.OwnedByCasefile:
                name = 'accept.png';
                break;
            case CardState.Unknown:
            default:
                name = 'help.png';
                break;
        }
        return 'images/' + name;
    }
    getOwnedByString = () => {
        if (this.props.info.owners && !(this.props.info.owners.length  > 0)) {
            return "???";
        }
        let s = "";
        for (let i = 0; i < this.props.info.owners.length; ++i) {
            let curIndex = this.props.info.owners[i];
            if (curIndex == this.props.playerInfos.length) {
                s += "(solution)";
            }
            else {
                s += this.props.playerInfos[curIndex].name;
            }
            if (i < this.props.info.owners.length - 1) {
                s += " or ";
            }
        }
        return s;
    }
    getAltText = () => {
        switch (this.props.info.state) {
            case CardState.OwnedByPlayer:
              return "Owned by " + this.getOwnedByString();
              break;
            case CardState.OwnedByCasefile:
              return "Solution!";
              break;
            case CardState.Unknown:
            default:
              if (this.props.info.owners && this.props.info.owners.length > 0) {
                  return "Owned by " + this.getOwnedByString();
              } else {
                  return "Unknown";
              }
              break;
        }
    }
    render = () => {
        return <tr>
            <td style={{ paddingLeft: 20, textAlign: 'left' }}>{CARD_NAMES[this.props.info.card_type][this.props.info.index].external}</td>
            <td><img src={this.getImgSrc()} alt={this.getAltText()} title={this.getAltText()} /></td>
        </tr>;
    }
}

interface GameInfoProps {
    playerInfos: Array<PlayerInfo>,
    cardInfos: Array<Array<CardInfo>>,
    session: string | null,
    isConsistent: boolean,
    clauseInfos: ClauseInfoMap,
    updateInfoFromJson: (json: any, haveEnteredData: boolean) => void,
    addToHistory: (historyEvent: HistoryEvent) => void,
    sendClueRequest: (data: string, successCallback: (responseJson : any) => void, failureCallback: (message : string) => void, skipWorking?: boolean) => void
}

class GameInfo extends React.Component<GameInfoProps, {}> {
    render = () => {
        let infoRows = [];
        for (let i = 0; i < CARD_TYPE_NAMES.length; ++i) {
            infoRows.push(<tr key={i}><th style={{textAlign: 'left'}}>{CARD_TYPE_NAMES[i]}</th><th></th></tr>);
            for (let j = 0; j < this.props.cardInfos[i].length; ++j) {
                infoRows.push(<SpecificCardInfo key={i + ' ' + j} info={this.props.cardInfos[i][j]} playerInfos={this.props.playerInfos}/>);
            }
        }
        return <div><div style={{float: 'left'}}><div className="warning">{!this.props.isConsistent && "Game is no longer consistent!"}</div><table><tbody>{infoRows}</tbody></table></div><div style={{float: 'left'}}>Enter new info:
          <Tabs>
              <TabList>
                  <Tab>Who owns a card</Tab>
                  <Tab>Suggestion made</Tab>
              </TabList>
              <TabPanel>
                  <WhoOwnsACard playerInfos={this.props.playerInfos} session={this.props.session} updateInfoFromJson={this.props.updateInfoFromJson} addToHistory={this.props.addToHistory} sendClueRequest={this.props.sendClueRequest} />
              </TabPanel>
              <TabPanel>
                  <SuggestACard playerInfos={this.props.playerInfos} session={this.props.session} updateInfoFromJson={this.props.updateInfoFromJson} addToHistory={this.props.addToHistory} sendClueRequest={this.props.sendClueRequest} />
              </TabPanel>
          </Tabs>
        </div>
        <div style={{clear: 'both'}}/><ClauseInfo clauseInfos={this.props.clauseInfos} playerInfos={this.props.playerInfos} /></div>;
    }
}

interface HistoryProps {
    doUndo: () => void,
    history: Array<HistoryEntry>,
    playerInfos: Array<PlayerInfo>,
}

class History extends React.Component<HistoryProps, {}> {
    doUndo = () => {
        this.props.doUndo();
    }
    render = () => {
        let entries = [];
        for (let i = 0; i < this.props.history.length; ++i)
        {
            let event = this.props.history[i].event;
            let description = '';
            switch (event.history_type) {
                case "suggestion":
                    description = this.props.playerInfos[event.suggester_index].name + " suggested " + CARD_NAMES[0][event.suspect_index].external + ", " + CARD_NAMES[1][event.weapon_index].external + ", " + CARD_NAMES[2][event.room_index].external + " ";
                    if (event.refuter_index == -1) {
                        description += " - no one refuted";
                    }
                    else {
                        description += " - refuted by " + this.props.playerInfos[event.refuter_index].name + " with card ";
                        //TODO - check for none
                        if (isNull(event.refuted_card_index) || (event.refuted_card_index.card_type == -1 && event.refuted_card_index.index == -1)) {
                            description += "Unknown";
                        }
                        else {
                            description += CARD_NAMES[event.refuted_card_index.card_type][event.refuted_card_index.index].external;
                        }
                    }
                    break;
                case "whoOwns":
                    let player = "Solution (case file)";
                    if (event.player_index < this.props.playerInfos.length) {
                        player = this.props.playerInfos[event.player_index].name;
                    }
                    description = CARD_NAMES[event.card_index.card_type][event.card_index.index].external + " owned by " + player;
                    break;
            }
            entries.push(<li key={i}>{description}</li>);
        }

        return <div>
          <ul>{entries}</ul>
          <div><button type="button" disabled={entries.length == 0} onClick={this.doUndo}>Undo latest information</button></div>
        </div>;
    }
}

type Clause = Array<CardIndex>;
//ClauseInfo is a map of player index to an array of array of cards
type ClauseInfoMap = Map<number, Array<Clause>>;

interface ClauseInfoProps {
    playerInfos: Array<PlayerInfo>,
    clauseInfos: ClauseInfoMap
}

class ClauseInfo extends React.Component<ClauseInfoProps, {}> {
    render = () => {
        let rows = [];
        let playerIndices: Array<number> = Array.from(this.props.clauseInfos.keys());
        for (let playerIndex of playerIndices) {
            // make a copy since we're modifying it
            let info = this.props.clauseInfos.get(playerIndex)!.slice();
            for (let i = 0; i < info.length; ++i) {
                let s = this.props.playerInfos[playerIndex].name + " has ";
                let names = info[i].sort(compareCardIndexByCategory).map((x: CardIndex) => CARD_NAMES[x.card_type][x.index].external);
                s += names.join(' or ');
                rows.push(<tr key={playerIndex + ' ' + i}><td>{s}</td></tr>);
            }
        }
        return <table><tbody>{rows}</tbody></table>;
    }
}

interface WhoOwnsACardProps {
    playerInfos: Array<PlayerInfo>,
    session: string | null,
    updateInfoFromJson: (json: any, haveEnteredData: boolean) => void,
    addToHistory: (historyEvent: HistoryEvent) => void,
    sendClueRequest: (data: string, successCallback: (responseJson : any) => void, failureCallback: (message : string) => void, skipWorking?: boolean) => void
}
interface WhoOwnsACardState {
    cardIndex: CardIndex,
    playerIndex: number
}

class WhoOwnsACard extends React.Component<WhoOwnsACardProps, WhoOwnsACardState> {
    constructor(props: WhoOwnsACardProps) {
        super(props);
        this.state = { cardIndex: { card_type: 0, index: 0 }, playerIndex: 0 };
    }
    setCardIndex = (newCardIndex: CardIndex) => {
        this.setState({cardIndex: newCardIndex});
    }
    setPlayerIndex = (newPlayerIndex: number) => {
        this.setState({playerIndex: newPlayerIndex});
    }
    sendWhoOwnsACard = () => {
        let data = "sess=" + this.props.session + "&action=whoOwns&owner=" + this.state.playerIndex + "&card=" + CARD_NAMES[this.state.cardIndex.card_type][this.state.cardIndex.index].internal;
        let description: HistoryWhoOwns = {history_type: 'whoOwns', player_index: this.state.playerIndex, card_index: this.state.cardIndex};
        let that = this;
        this.props.sendClueRequest(data, (json: any) => {
            that.props.addToHistory(description);
            that.props.updateInfoFromJson(json, true);
        }, (message: string) => {
            alert('Error: ' + message);
        });
    }
    render = () => {
        return <div>
            <div><CardSelector cardType={-1} cardIndex={this.state.cardIndex} setCardIndex={this.setCardIndex}/></div>
            <div><PlayerSelector label="Owned by" includeSolution={true} includeNone={false} playerInfos={this.props.playerInfos} playerIndex={this.state.playerIndex} setPlayerIndex={this.setPlayerIndex}/></div>
            <button type="button" onClick={this.sendWhoOwnsACard}>Add info</button>
        </div>;
    }
}

interface SuggestACardState {
    suggestingPlayerIndex: number,
    refutingPlayerIndex: number,
    // First entry of this is the index of the suspect card
    // Second entry of this is the index of the weapon card
    // Third entry of this is the index of the room card
    cardIndices: Array<number>,
    refutingCardIndex: CardIndex
}
class SuggestACard extends React.Component<WhoOwnsACardProps, SuggestACardState> {
    constructor(props: WhoOwnsACardProps) {
        super(props);
        this.state = { suggestingPlayerIndex: 0, refutingPlayerIndex: -1, cardIndices: [0, 0, 0], refutingCardIndex: { card_type: -1, index: -1 } };
    }
    setSuggestingPlayerIndex = (playerIndex: number) => {
        this.setState({suggestingPlayerIndex: playerIndex});
    }
    setRefutingPlayerIndex = (playerIndex: number) => {
        this.setState({refutingPlayerIndex: playerIndex});
    }
    setRefutingCardIndex = (cardIndex: CardIndex) => {
        this.setState({refutingCardIndex: cardIndex});
    }
    setCardIndex = (cardIndex: CardIndex) => {
        // TODO - need to copy?
        let newCardIndices = this.state.cardIndices;
        newCardIndices[cardIndex.card_type] = cardIndex.index;
        this.setState({cardIndices: newCardIndices});
    }
    sendSuggestion = () => {
        let data = "sess=" + this.props.session + "&action=suggestion&suggestingPlayer=" + this.state.suggestingPlayerIndex;
        for (let i = 0; i < CARD_TYPE_NAMES.length; ++i) {
            data += "&card" + (i+1) + "=" + CARD_NAMES[i][this.state.cardIndices[i]].internal;
        }
        data += "&refutingPlayer=" + this.state.refutingPlayerIndex + "&refutingCard=";
        //TODO - none check
        if (this.state.refutingCardIndex.card_type == -1 && this.state.refutingCardIndex.index == -1) {
            data += "None";
        }
        else {
            data += CARD_NAMES[this.state.refutingCardIndex.card_type][this.state.refutingCardIndex.index].internal;
        }
        const description : HistorySuggestion = {
            history_type: "suggestion",
            suggester_index: this.state.suggestingPlayerIndex,
            suspect_index: this.state.cardIndices[0],
            weapon_index: this.state.cardIndices[1],
            room_index: this.state.cardIndices[2],
            refuter_index: this.state.refutingPlayerIndex,
            refuted_card_index: this.state.refutingCardIndex
        };
        let that = this;
        this.props.sendClueRequest(data, function(json) {
            that.props.addToHistory(description);
            that.props.updateInfoFromJson(json, true);
        }, function(errorText) {
            alert('Error: ' + errorText);
        });
    }
    render = () => {
        // TODO - hide suggesting player in refuting player box?
        return <div>
            <div><PlayerSelector label="Made by" includeSolution={false} playerInfos={this.props.playerInfos} playerIndex={this.state.suggestingPlayerIndex} setPlayerIndex={this.setSuggestingPlayerIndex} /></div>
            <div><CardSelector cardType={0} cardIndex={{ card_type: 0, index: this.state.cardIndices[0] }} setCardIndex={this.setCardIndex} /></div>
            <div><CardSelector cardType={1} cardIndex={{ card_type: 1, index: this.state.cardIndices[1] }} setCardIndex={this.setCardIndex}/></div>
            <div><CardSelector cardType={2} cardIndex={{ card_type: 2, index: this.state.cardIndices[2] }} setCardIndex={this.setCardIndex}/></div>
            <div><PlayerSelector label="Refuted by" includeSolution={false} includeNone={true} playerInfos={this.props.playerInfos} playerIndex={this.state.refutingPlayerIndex} setPlayerIndex={this.setRefutingPlayerIndex}/></div>
            <div><RefutingCardSelector cardIndex={this.state.refutingCardIndex} cardIndices={this.state.cardIndices} setCardIndex={this.setRefutingCardIndex} /></div>
            <button type="button" onClick={this.sendSuggestion}>Add info</button>
        </div>;
    }
}

interface CardSelectorProps {
    cardType: number,
    cardIndex: CardIndex,
    setCardIndex: (newCardIndex: CardIndex) => void
}

class CardSelector extends React.Component<CardSelectorProps, {}> {
    handleChange = (e: ChangeEvent<HTMLSelectElement>) => {
        let cardIndexParts: Array<number> = e.target.value.split(" ").map((x: string) => parseInt(x, 10));
        this.props.setCardIndex({ card_type: cardIndexParts[0], index: cardIndexParts[1] });
    }
    render = () => {
        let options = [];
        let allCardTypes = this.props.cardType == -1;
        let iMin = allCardTypes ? 0 : this.props.cardType;
        let iMax = allCardTypes ? CARD_TYPE_NAMES.length : (this.props.cardType + 1);
        let label = allCardTypes ? "Card" : CARD_TYPE_NAMES[this.props.cardType];
        for (let i = iMin; i < iMax; ++i) {
            for (let j = 0; j < CARD_NAMES[i].length; ++j) {
                options.push(<option value={i + ' ' + j} key={i + ' ' + j}>{CARD_NAMES[i][j].external}</option>);
            }
        }
        return <span>{label}: <select value={this.props.cardIndex.card_type + ' ' + this.props.cardIndex.index} onChange={this.handleChange}>{options}</select></span>;
    }
}

interface RefutingCardSelectorProps {
    cardIndex: CardIndex,
    // First entry of this is the index of the suspect card
    // Second entry of this is the index of the weapon card
    // Third entry of this is the index of the room card
    cardIndices: Array<number>,
    setCardIndex: (newCardIndex: CardIndex) => void
}

class RefutingCardSelector extends React.Component<RefutingCardSelectorProps, {}> {
    handleChange = (e: ChangeEvent<HTMLSelectElement>) => {
        let cardIndexParts: Array<number> = e.target.value.split(" ").map((x: string) => parseInt(x, 10));
        this.props.setCardIndex({ card_type: cardIndexParts[0], index: cardIndexParts[1] });
    }
    render = () => {
        let options = [];
        let invalidSelectedCard = true;
        //TODO - use a constant for this "-1 -1" stuff (and similar CardIndex)
        options.push(<option value="-1 -1" key="-1">None/Unknown</option>);
        let selectedCardIndexString = this.props.cardIndex.card_type + ' ' + this.props.cardIndex.index;
        if (selectedCardIndexString == "-1 -1") {
            invalidSelectedCard = false;
        }
        for (let i = 0; i < this.props.cardIndices.length; ++i) {
            let cardInfo = CARD_NAMES[i][this.props.cardIndices[i]];
            //TODO - should really have a method for this
            let cardIndexString = cardInfo.card_index.card_type + ' ' + cardInfo.card_index.index;
            if (selectedCardIndexString == cardIndexString) {
                invalidSelectedCard = false;
            }
            options.push(<option value={cardIndexString} key={cardInfo.card_index.card_type}>{cardInfo.external}</option>);
        }
        if (invalidSelectedCard) {
            // this happens if there's a selected refuting card, and then
            // the choices change so this isn't an option anymore
            this.props.setCardIndex({ card_type: -1, index: -1 });
        }
        return <span>Refuting card: <select value={selectedCardIndexString} onChange={this.handleChange}>{options}</select></span>;
    }
}

interface PlayerSelectorProps {
    label: string,
    includeSolution: boolean,
    includeNone?: boolean,
    playerInfos: Array<PlayerInfo>,
    playerIndex: number,
    setPlayerIndex: (playerIndex: number) => void
}

class PlayerSelector extends React.Component<PlayerSelectorProps, {}> {
    handleChange = (e: ChangeEvent<HTMLSelectElement>) => {
        this.props.setPlayerIndex(parseInt(e.target.value));
    }
    render = () => {
        let options = [];
        if (this.props.includeNone) {
            options.push(<option value={-1} key={-1}>None</option>);
        }
        for (let i = 0; i < this.props.playerInfos.length; ++i) {
            options.push(<option value={i} key={i}>{this.props.playerInfos[i].name}</option>);
        }
        if (this.props.includeSolution) {
            options.push(<option value={this.props.playerInfos.length} key={this.props.playerInfos.length}>Solution (case file)</option>);
        }
        return <span>{this.props.label}: <select value={this.props.playerIndex} onChange={this.handleChange}>{options}</select></span>
    }
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

interface SimulationProps {
    playerInfos: Array<PlayerInfo>,
    setDoingSimulation: (doingSimulation: boolean) => void,
    sendClueRequest: (data: string, successCallback: (responseJson : any) => void, failureCallback: (message : string) => void, skipWorking?: boolean) => void,
    session: string | null,
    cardIndexFromInternalName : (name: string) => CardIndex,
    setNumberOfSimulations: (simulations: number) => void,
    setSimulationData: (simulationData: SimulationData) => void,
    simData: SimulationData,
    numberOfSimulations: number,
    doingSimulation: boolean,
    isConsistent: boolean
}

class Simulation extends React.Component<SimulationProps, {}> {
    doSimulate = () => {
        this.props.setDoingSimulation(true);
        let that = this;
        this.props.sendClueRequest('sess=' + this.props.session + '&action=simulate', function (json) {
            let simData = new Map<string, Array<number>>();
            let totalNumberOfSims = 0;
            for (let jsonKey in json.simData) {
                let key = jsonKey as string;
                let data : Array<number> = json.simData[key];
                totalNumberOfSims = data.reduce((previousValue: number, currentValue: number) => previousValue + currentValue, 0);
                if (totalNumberOfSims > 0) {
                    simData.set(key, data.map((x: number) => x / totalNumberOfSims));
                } else {
                    simData.set(key, data.map((x: number) => 0.0));
                }
            }
            that.props.setDoingSimulation(false);
            that.props.setNumberOfSimulations(totalNumberOfSims);
            that.props.setSimulationData(simData);
        }, function(errorText) {
            alert('Error: ' + errorText);
            that.props.setDoingSimulation(false);
        },
        true);
    }
    interpolateBetween = (percent: number, colorArray: Array<Array<number>>): any => {
        let quartiles = colorArray.length - 1;
        let quartileSpan = 100 / quartiles;
        let quartile = Math.floor((percent / 100) * quartiles);
        if (quartile >= quartiles) {
            quartile = quartiles - 1;
        }
        let r = colorArray[quartile][0] + (colorArray[quartile+1][0] - colorArray[quartile][0]) * ((percent - quartile * quartileSpan) / quartileSpan);
        let g = colorArray[quartile][1] + (colorArray[quartile+1][1] - colorArray[quartile][1]) * ((percent - quartile * quartileSpan) / quartileSpan);
        let b = colorArray[quartile][2] + (colorArray[quartile+1][2] - colorArray[quartile][2]) * ((percent - quartile * quartileSpan) / quartileSpan);
        return {backgroundColor: 'rgb(' + Math.round(r) + ', ' + Math.round(g) + ', ' + Math.round(b) + ')', textAlign: 'right'};
    }
    getColorProp = (percent: number) => {
        // Interpolation between
        // (3, 146, 207) blue
        // (123, 192, 67) green
        //// (253, 244, 152) yellow
        // (243, 119, 54) orange
        // (238, 64, 53) red
        // http://www.color-hex.com/color-palette/807
        return this.interpolateBetween(percent,
            [[3, 146, 207],
             [123, 192, 67],
             //[253, 244, 152],
             [243, 119, 54],
             [238, 64, 53]
             ]);
    }
    render = () => {
        let infoRows = [];
        let playerHeaderEntries = [];
        for (let i = 0; i < this.props.playerInfos.length; ++i) {
            playerHeaderEntries.push(<th className="playerHeader" key={"player" + i}>{this.props.playerInfos[i].name}</th>);
        }
        playerHeaderEntries.push(<th className="playerHeader" key="solution">Solution</th>);
        for (let i = 0; i < CARD_TYPE_NAMES.length; ++i) {
            infoRows.push(<tr key={i}><th style={{textAlign: 'left'}}>{CARD_TYPE_NAMES[i]}</th>{i == 0 ? playerHeaderEntries : undefined}</tr>);
            for (let j = 0; j < CARD_NAMES[i].length; ++j) {
                let cells = [<td key="cardName">{CARD_NAMES[i][j].external}</td>];
                let dataArray = this.props.simData.get(CARD_NAMES[i][j].internal);
                for (let k = 0; k < this.props.playerInfos.length; ++k) {
                    if (dataArray != undefined && dataArray[k] !== undefined) {
                        let v = (Math.round(dataArray[k] * 1000) / 10);
                        let c = this.getColorProp(v);
                        let vString = Number(v).toFixed(1) + '%';
                        cells.push(<td style={c} key={"player" + k}>{vString}</td>);
                    }
                    else {
                        cells.push(<td key={"player" + k}></td>);
                    }
                }
                let sc = {};
                if (dataArray != undefined && dataArray[this.props.playerInfos.length] !== undefined) {
                    let percent = (Math.round(dataArray[this.props.playerInfos.length] * 1000) / 10);
                    sc = this.getColorProp(percent);
                    let percentString = Number(percent).toFixed(1) + '%';
                    cells.push(<td style={sc} key="solution">{percentString}</td>);
                }
                else {
                    cells.push(<td style={sc} key="solution"></td>);
                }
                infoRows.push(<tr key={i + ' ' + j}>{cells}</tr>);
            }
        }
        let working = undefined;
        if (this.props.doingSimulation) {
            working = <img src="images/loading.gif"/>;
        }
        let simulationSpan = undefined;
        if (this.props.numberOfSimulations > -1) {
            if (this.props.numberOfSimulations > 0) {
                simulationSpan = <span style={{marginLeft: 20, fontStyle: "italic"}}>Simulations done: {this.props.numberOfSimulations}</span>;
            } else {
                simulationSpan = <span className="warning" style={{marginLeft: 20, fontStyle: "italic"}}>No simulations done!</span>;
            }
        }
        return <div>
          <div className="warning">{!this.props.isConsistent && "Game is no longer consistent!"}</div>
          <div><button type="button" onClick={this.doSimulate} disabled={this.props.doingSimulation}>Simulate</button>{working}{simulationSpan}</div>
          <div style={{display: 'flex'}}>
              <table><tbody>{infoRows}</tbody></table>
              <div style={{display: 'flex', verticalAlign: 'top', flexDirection: 'row', height: 300}}><img src="images/rainbow.png" style={{height: 300, width: 50}}/><div style={{display: 'flex', flexDirection: 'column', height: 300}}><div style={{alignSelf: 'flex-start'}}>0%</div><div style={{flex: 1}}/><div style={{alignSelf: 'flex-end'}}>100%</div></div></div>
          </div>
      </div>;
    }
}

// Map from internal card name to probabilities (between 0-1)
type SimulationData = Map<string, Array<number>>;

interface AppState {
    playerInfos: Array<PlayerInfo>,
    cardInfos: Array<Array<CardInfo>>,
    isConsistent: boolean,
    haveEnteredData: boolean,
    clauseInfos: Map<number, Array<Array<CardIndex>>>,
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
            clauseInfos: new Map<number, Array<Array<CardIndex>>>(),
            history: [],
            working: false,
            simData: new Map<string, Array<number>>(),
            doingSimulation: false,
            numberOfSimulations: -1,
            session: null
        };
    }
    setSimulationData = (data: SimulationData) => {
        this.setState({simData: data});
    }
    setNumberOfSimulations = (n: number) => {
        this.setState({numberOfSimulations: n});
    }
    setDoingSimulation = (isDoingSimulation: boolean) => {
        this.setState({doingSimulation: isDoingSimulation});
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
        let clauseInfos = new Map<number, Array<Array<CardIndex>>>();
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
                clauseInfos.set(playerIndexNumber, newClauses);
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
            clauseInfos: clauseInfos,
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

    addToHistory = (historyEvent: HistoryEvent) => {
        //TODO - assert this is non-null?
        let session = this.state.session!;
        this.setState(function(previousState, currentProps) {
            let history = previousState.history;
            history.push({ event: historyEvent, session: session });
            return {'history': history};
        });
    }

    doUndo = () => {
        if (this.state.history.length > 0) {
            let that = this;
            this.updateCardInfo(this.state.history[this.state.history.length - 1].session, this.state.history.length > 1, function () {
                that.setState({'history': that.state.history.slice(0, that.state.history.length - 1)});
            });
        }
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
                        <GameInfo
                            playerInfos={this.state.playerInfos}
                            cardInfos={this.state.cardInfos}
                            session={this.state.session}
                            isConsistent={this.state.isConsistent}
                            clauseInfos={this.state.clauseInfos}
                            updateInfoFromJson={this.updateInfoFromJson}
                            addToHistory={this.addToHistory}
                            sendClueRequest={this.sendClueRequest} />
                    </TabPanel>
                    <TabPanel>
                        <History
                            playerInfos={this.state.playerInfos}
                            history={this.state.history}
                            doUndo={this.doUndo} />
                    </TabPanel>
                    <TabPanel>
                        <Simulation
                            playerInfos={this.state.playerInfos}
                            session={this.state.session}
                            isConsistent={this.state.isConsistent}
                            sendClueRequest={this.sendClueRequest}
                            cardIndexFromInternalName={this.cardIndexFromInternalName}
                            simData={this.state.simData}
                            numberOfSimulations={this.state.numberOfSimulations}
                            doingSimulation={this.state.doingSimulation}
                            setSimulationData={this.setSimulationData}
                            setNumberOfSimulations={this.setNumberOfSimulations}
                            setDoingSimulation={this.setDoingSimulation} />
                    </TabPanel>
                </Tabs>
            </div>
        );
    }
}

export default App;
