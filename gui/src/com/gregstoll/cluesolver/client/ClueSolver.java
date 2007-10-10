package com.gregstoll.cluesolver.client;

import com.google.gwt.core.client.EntryPoint;
import com.google.gwt.http.client.RequestBuilder;
import com.google.gwt.http.client.URL;
import com.google.gwt.user.client.Window;
import com.google.gwt.user.client.ui.Button;
import com.google.gwt.user.client.ui.ClickListener;
import com.google.gwt.user.client.ui.FlowPanel;
import com.google.gwt.user.client.ui.Grid;
import com.google.gwt.user.client.ui.HorizontalPanel;
import com.google.gwt.user.client.ui.HTML;
import com.google.gwt.user.client.ui.Label;
import com.google.gwt.user.client.ui.ListBox;
import com.google.gwt.user.client.ui.PopupPanel;
import com.google.gwt.user.client.ui.RadioButton;
import com.google.gwt.user.client.ui.RootPanel;
import com.google.gwt.user.client.ui.TabPanel;
import com.google.gwt.user.client.ui.Tree;
import com.google.gwt.user.client.ui.TreeItem;
import com.google.gwt.user.client.ui.VerticalPanel;
import com.google.gwt.user.client.ui.Widget;
import com.google.gwt.json.client.JSONArray;
import com.google.gwt.json.client.JSONParser;
import com.google.gwt.json.client.JSONString;
import com.google.gwt.json.client.JSONObject;
import com.google.gwt.json.client.JSONValue;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Entry point classes define <code>onModuleLoad()</code>.
 */
public class ClueSolver implements EntryPoint {
  public static final String[][] internalNames = {{"ProfessorPlum", "ColonelMustard", "MrGreen", "MissScarlet", "MsWhite", "MrsPeacock"},
                                    {"Knife", "Candlestick", "Revolver", "LeadPipe", "Rope", "Wrench"},
                                    {"Hall", "Conservatory", "DiningRoom", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "BilliardRoom"}};
  public static final String[][] externalNames = {{"Professor Plum", "Colonel Mustard", "Mr. Green", "Miss Scarlet", "Ms. White", "Mrs. Peacock"},
                                    {"Knife", "Candlestick", "Revolver", "Lead Pipe", "Rope", "Wrench"},
                                    {"Hall", "Conservatory", "Dining Room", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "Billiard Room"}};

  public String[] playerNames = {"Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6"};
  public VerticalPanel namesPanel = null;
  public HashMap internalNameToClueStateWidgetMap = new HashMap();
  private ArrayList playerListBoxes = new ArrayList();
  public static final String scriptName = "clue.cgi";
  public String curSessionString = null;
  /*private static class TestPopup extends PopupPanel {
    public TestPopup(String s) {
        super(true);
        HTML contents = new HTML(s);
        contents.setWidth("128px");
        setWidget(contents);

        setStyleName("ks-popups-Popup");
    }
  }*/

  CgiResponseHandler newInfoHandler = new CgiResponseHandler() {
      public void onSuccess(String body) {
        JSONObject response = JSONParser.parse(body).isObject();
        double errorStatus = response.get("errorStatus").isNumber().getValue();
        if (errorStatus != 0.0) {
            Window.alert("Internal error - error returned from script - " + response.get("errorText").isString().toString());
        } else {
            curSessionString = response.get("session").isString().stringValue();
            JSONArray newInfos = response.get("newInfo").isArray();
            int numElements = newInfos.size();
            for (int i = 0; i < numElements; ++i) {
                JSONObject curInfo = newInfos.get(i).isObject();
                String card = curInfo.get("card").isString().stringValue();
                int status = (int) curInfo.get("status").isNumber().getValue();
                JSONArray ownerArray = curInfo.get("owner").isArray();
                int[] owners = new int[ownerArray.size()];
                for (int j = 0; j < owners.length; ++j) {
                    owners[j] = (int) ownerArray.get(j).isNumber().getValue();
                }
                getStateWidget(card).setState(status, owners);
            }
        }
      }
      public void onError(Throwable ex) {
        Window.alert("Internal error - unable to contact backend - " + ex.getMessage());
      }
  };

  /**
   * This is the entry point method.
   */
  public void onModuleLoad() {
    ClueStateWidget.solver = this;

    VerticalPanel playerInfoPanel = new VerticalPanel();
    playerInfoPanel.setHorizontalAlignment(VerticalPanel.ALIGN_LEFT);
    playerInfoPanel.setVerticalAlignment(HorizontalPanel.ALIGN_TOP);
    playerInfoPanel.add(new HTML("Number of players:"));
    FlowPanel radioPanel = new FlowPanel();
    for (int i = 2; i <= 6; ++i) {
        RadioButton cur = new RadioButton("numPlayers", new Integer(i).toString());
        final int iFinal = i;
        /*cur.addClickListener(new ClickListener() {
            public void onClick(Widget sender) {
                TestPopup tp = new TestPopup("You clicked button " + iFinal);
                int left = sender.getAbsoluteLeft() + 10;
                int top = sender.getAbsoluteTop() + 10;
                tp.setPopupPosition(left, top);
                tp.show();
            }
        });*/
        cur.addClickListener(new ClickListener() {
            public void onClick(Widget sender) {
                setNumberOfPlayers(iFinal);
            }
        });
        if (i == 6) {
            cur.setChecked(true);
        }
        radioPanel.add(cur);
    }
    playerInfoPanel.add(radioPanel);
    namesPanel = new VerticalPanel();
    namesPanel.setHorizontalAlignment(VerticalPanel.ALIGN_LEFT);
    namesPanel.setVerticalAlignment(HorizontalPanel.ALIGN_TOP);
    for (int i = 0; i < playerNames.length; ++i) {
        NameSuggestPanel nsp = new NameSuggestPanel(playerNames[i], i, this);
        namesPanel.add(nsp);
    }
    playerInfoPanel.add(namesPanel);

    HorizontalPanel gameInfoPanel = new HorizontalPanel();
    gameInfoPanel.setHorizontalAlignment(VerticalPanel.ALIGN_LEFT);
    gameInfoPanel.setVerticalAlignment(HorizontalPanel.ALIGN_TOP);

    Tree infoTree = new Tree();
    TreeItem suspectTree = new TreeItem("Suspects");
    for (int i = 0; i < internalNames[0].length; ++i) {
        suspectTree.addItem(new ClueStateWidget(internalNames[0][i], externalNames[0][i]));
    }
    TreeItem weaponTree = new TreeItem("Weapons");
    for (int i = 0; i < internalNames[1].length; ++i) {
        weaponTree.addItem(new ClueStateWidget(internalNames[1][i], externalNames[1][i]));
    }
    TreeItem roomTree = new TreeItem("Rooms");
    for (int i = 0; i < internalNames[2].length; ++i) {
        roomTree.addItem(new ClueStateWidget(internalNames[2][i], externalNames[2][i]));
    }
    infoTree.addItem(suspectTree);
    infoTree.addItem(weaponTree);
    infoTree.addItem(roomTree);
    suspectTree.setState(true);
    weaponTree.setState(true);
    roomTree.setState(true);
    gameInfoPanel.add(infoTree);

    VerticalPanel enterInfoPanel = new VerticalPanel();
    enterInfoPanel.add(new HTML("Enter new info:")); 
    TabPanel enterInfoTabs = new TabPanel();
    VerticalPanel whoOwnsCardPanel = new VerticalPanel();
    HorizontalPanel tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Card: "));
    final ListBox whichCardOwned = makeNewCardListBox(-1, false);
    tempPanel1.add(whichCardOwned);
    whoOwnsCardPanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Owned by: "));
    final ListBox ownerOwned = makeNewPlayerListBox(false, true);
    tempPanel1.add(ownerOwned);
    whoOwnsCardPanel.add(tempPanel1);
    Button whoOwnsSubmitButton = new Button("Add info", new ClickListener() {
        public void onClick(Widget sender) {
            CgiHelper.doRequest(RequestBuilder.POST, scriptName, "sess=" + curSessionString + "&action=whoOwns&owner=" + listBoxValue(ownerOwned) + "&card=" + listBoxValue(whichCardOwned), newInfoHandler);
        }
    });
    whoOwnsCardPanel.add(whoOwnsSubmitButton);
    enterInfoTabs.add(whoOwnsCardPanel, "Who owns a card");
    VerticalPanel suggestionMadePanel = new VerticalPanel();
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Made by: "));
    final ListBox suggestingPlayer = makeNewPlayerListBox(false, false);
    tempPanel1.add(suggestingPlayer);
    suggestionMadePanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Suspect: "));
    final ListBox card1 = makeNewCardListBox(0, false);
    tempPanel1.add(card1);
    suggestionMadePanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Weapon: "));
    final ListBox card2 = makeNewCardListBox(1, false);
    tempPanel1.add(card2);
    suggestionMadePanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Room: "));
    final ListBox card3 = makeNewCardListBox(2, false);
    tempPanel1.add(card3);
    suggestionMadePanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Refuted by: "));
    final ListBox refutingPlayer = makeNewPlayerListBox(true, false);
    tempPanel1.add(refutingPlayer);
    suggestionMadePanel.add(tempPanel1);
    tempPanel1 = new HorizontalPanel();
    tempPanel1.add(new HTML("Refuting card: "));
    final ListBox refutingCard = makeNewCardListBox(-1, true);
    tempPanel1.add(refutingCard);
    suggestionMadePanel.add(tempPanel1);
    Button suggestionSubmitButton = new Button("Add info", new ClickListener() {
        public void onClick(Widget sender) {
            CgiHelper.doRequest(RequestBuilder.POST, scriptName, "sess=" + curSessionString + "&action=suggestion&suggestingPlayer=" + listBoxValue(suggestingPlayer) + "&card1=" + listBoxValue(card1) + "&card2=" + listBoxValue(card2) + "&card3=" + listBoxValue(card3) + "&refutingPlayer=" + listBoxValue(refutingPlayer) + "&refutingCard=" + listBoxValue(refutingCard), newInfoHandler);
        }
    });
    suggestionMadePanel.add(suggestionSubmitButton);
    enterInfoTabs.add(suggestionMadePanel, "Suggestion made");

    enterInfoTabs.selectTab(0);
    enterInfoPanel.add(enterInfoTabs);
    gameInfoPanel.add(enterInfoPanel);

    TabPanel tabs = new TabPanel();
    tabs.add(playerInfoPanel, "Player Info");
    tabs.add(gameInfoPanel, "Game Info");
    tabs.selectTab(0);
    RootPanel.get().add(tabs);

    /*getStateWidget("ProfessorPlum").setState(ClueStateWidget.STATE_OWNED_BY_CASEFILE, -1);
    getStateWidget("ColonelMustard").setState(ClueStateWidget.STATE_OWNED_BY_PLAYER, 1);*/
    // Get the state of the game.
    CgiHelper.doRequest(RequestBuilder.POST, scriptName, "action=new&players=6", new CgiResponseHandler() {
        public void onSuccess(String body) {
            JSONObject response = JSONParser.parse(body).isObject();
            double errorStatus = response.get("errorStatus").isNumber().getValue();
            if (errorStatus != 0.0) {
                Window.alert("Internal error - error returned from script - " + response.get("errorText").isString().toString());
            } else {
                curSessionString = response.get("session").isString().stringValue();
            }
        }
        public void onError(Throwable ex) {
            Window.alert("Internal error - unable to contact backend for new session - " + ex.getMessage());
        }
    });
     
    // Assume that the host HTML has elements defined whose
    // IDs are "slot1", "slot2".  In a real app, you probably would not want
    // to hard-code IDs.  Instead, you could, for example, search for all 
    // elements with a particular CSS class and replace them with widgets.
    //
    //RootPanel.get("table").add(g);
    
  }

  public void setNumberOfPlayers(int numP) {
    // TODO - check here if we've done anything.  If not, fine.
    // If so, ask if they really want to start a new game.
    int curNumP = playerNames.length;
    String[] newPlayerNames = new String[numP];
    if (curNumP > numP) {
        for (int i = 0; i < numP; ++i) {
            newPlayerNames[i] = playerNames[i];
        }
        while (curNumP > numP) {
            namesPanel.remove(namesPanel.getWidgetCount() - 1);
            --curNumP;
        }
    } else if (curNumP < numP) {
        for (int i = 0; i < curNumP; ++i) {
            newPlayerNames[i] = playerNames[i];
        }
        while (curNumP < numP) {
            newPlayerNames[curNumP] = "Player " + new Integer(curNumP + 1).toString();
            namesPanel.add(new NameSuggestPanel(newPlayerNames[curNumP], curNumP, this));
            ++curNumP;
        }
    }
    playerNames = newPlayerNames;
  }

  public ClueStateWidget getStateWidget(String id) {
      return (ClueStateWidget) internalNameToClueStateWidgetMap.get(id);
      //return ((ClueStateWidget)RootPanel.get(id).getWidget(0));
  }

  public static String listBoxValue(ListBox lb) {
      return lb.getValue(lb.getSelectedIndex());
  }

  public ListBox makeNewCardListBox(int index, boolean includeNoneUnknown) {
      ListBox toReturn = new ListBox();
      if (includeNoneUnknown) {
          toReturn.addItem("None/Unknown", "None");
      }
      if (index == -1) {
        for (int i = 0; i < externalNames.length; ++i) {
            for (int j = 0; j < externalNames[i].length; ++j) {
                toReturn.addItem(externalNames[i][j], internalNames[i][j]);
            }
        }
      } else {
        for (int i = 0; i < externalNames[index].length; ++i) {
            toReturn.addItem(externalNames[index][i], internalNames[index][i]);
        }
      }
      return toReturn;
  }

  public ListBox makeNewPlayerListBox(boolean includeNone, boolean includeSolution) {
      ListBox toReturn = new ListBox();
      if (includeNone) {
          toReturn.addItem("None", "-1");
      }
      for (int i = 0; i < playerNames.length; ++i) {
          toReturn.addItem(playerNames[i], new Integer(i).toString());
      }
      if (includeSolution) {
          toReturn.addItem("Solution (case file)", new Integer(playerNames.length).toString());
      }
      playerListBoxes.add(toReturn);
      return toReturn;
  }

  public void changePlayerName(int index, String newName) {
      playerNames[index] = newName;
      for (int i = 0; i < playerListBoxes.size(); ++i) {
        ListBox listBox = ((ListBox) playerListBoxes.get(i));
        // See if we start with an extra item.
        int curIndex = index;
        if (listBox.getValue(0).equals("-1")) {
            ++curIndex;
        }
        listBox.setItemText(curIndex, newName);
      }
      // Update the tooltips on the images in the ClueStateWidgets
      Set stateWidgetKeys = internalNameToClueStateWidgetMap.entrySet();
      for (Iterator it = stateWidgetKeys.iterator(); it.hasNext();) {
          Map.Entry curEntry = (Map.Entry) it.next(); 
          ClueStateWidget curWidget = (ClueStateWidget) curEntry.getValue();
          curWidget.setImage();
      }
  }

}
