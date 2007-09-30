package com.gregstoll.cluesolver.client;

import com.google.gwt.core.client.EntryPoint;
import com.google.gwt.user.client.ui.Button;
import com.google.gwt.user.client.ui.ClickListener;
import com.google.gwt.user.client.ui.Grid;
import com.google.gwt.user.client.ui.Label;
import com.google.gwt.user.client.ui.RootPanel;
import com.google.gwt.user.client.ui.Widget;

/**
 * Entry point classes define <code>onModuleLoad()</code>.
 */
public class ClueSolver implements EntryPoint {

  public static int[] numCards = {6, 6, 9};
  public static String[][] internalNames = {{"ProfessorPlum", "ColonelMustard", "MrGreen", "MissScarlet", "MsWhite", "MrsPeacock"},
                                    {"Knife", "Candlestick", "Revolver", "LeadPipe", "Rope", "Wrench"},
                                    {"Hall", "Conservatory", "DiningRoom", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "BilliardRoom"}};
  public static String[][] externalNames = {{"Professor Plum", "Colonel Mustard", "Mr. Green", "Miss Scarlet", "Ms. White", "Mrs. Peacock"},
                                    {"Knife", "Candlestick", "Revolver", "Lead Pipe", "Rope", "Wrench"},
                                    {"Hall", "Conservatory", "Dining Room", "Kitchen", "Study", "Library", "Ballroom", "Lounge", "Billiard Room"}};
  /**
   * This is the entry point method.
   */
  public void onModuleLoad() {
    final Button button = new Button("Click me");
    final Label label = new Label();

    button.addClickListener(new ClickListener() {
      public void onClick(Widget sender) {
        if (label.getText().equals(""))
          label.setText("Hello World!");
        else
          label.setText("");
      }
    });
    for (int i = 0; i < 6; ++i) {
        RootPanel.get("staticSuspect" + (i + 1)).add(new Label(externalNames[0][i]));
        RootPanel.get("suspect" + (i + 1)).add(new ClueStateWidget());
    }
    for (int i = 0; i < 6; ++i) {
        RootPanel.get("staticWeapon" + (i + 1)).add(new Label(externalNames[1][i]));
        RootPanel.get("weapon" + (i + 1)).add(new ClueStateWidget());
    }
    for (int i = 0; i < 9; ++i) {
        RootPanel.get("staticRoom" + (i + 1)).add(new Label(externalNames[2][i]));
        RootPanel.get("room" + (i + 1)).add(new ClueStateWidget());
    }
    
    // Assume that the host HTML has elements defined whose
    // IDs are "slot1", "slot2".  In a real app, you probably would not want
    // to hard-code IDs.  Instead, you could, for example, search for all 
    // elements with a particular CSS class and replace them with widgets.
    //
    //RootPanel.get("table").add(g);
  }
}
