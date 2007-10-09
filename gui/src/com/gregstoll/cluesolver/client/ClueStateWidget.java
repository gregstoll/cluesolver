package com.gregstoll.cluesolver.client;

import com.google.gwt.core.client.GWT;
import com.google.gwt.user.client.ui.HorizontalPanel;
import com.google.gwt.user.client.ui.HTML;
import com.google.gwt.user.client.ui.Image;
import com.google.gwt.user.client.ui.ImageBundle;
import com.google.gwt.user.client.ui.AbstractImagePrototype;
import com.google.gwt.user.client.ui.Image;

public class ClueStateWidget extends HorizontalPanel {
    public static final int STATE_UNKNOWN = 0;
    public static final int STATE_OWNED_BY_PLAYER = 1;
    public static final int STATE_OWNED_BY_CASEFILE = 2;
    public static ClueSolver solver = null;
    public String privateName = null;
    public String publicName = null;

    public int ownerIndex = -1;
    public int curState = STATE_UNKNOWN;

    public interface Images extends ImageBundle {
        /**
         * @gwt.resource accept.png
         */
        AbstractImagePrototype ownedByCasefile();

        /**
         * @gwt.resource cancel.png
         */
        AbstractImagePrototype ownedByPlayer();

        /**
         * @gwt.resource help.png
         */
        AbstractImagePrototype unknown();
    }

    private Images images = (Images) GWT.create(Images.class);
    private Image curImage = new Image();

    public ClueStateWidget(String privateName, String publicName) {
        this.privateName = privateName;
        this.publicName = publicName;
        curState = STATE_UNKNOWN;
        HTML name = new HTML(publicName);
        name.setWidth("125px");
        add(name);
        add(curImage);
        setImage();
        solver.internalNameToClueStateWidgetMap.put(privateName, this);
    }

    public void setState(int state, int owner) {
        curState = state;
        if (curState == STATE_OWNED_BY_PLAYER) {
            ownerIndex = owner;
        }
        setImage();
    }

    private void setImage() {
        switch (curState) {
            case STATE_UNKNOWN:
                images.unknown().applyTo(curImage);
                curImage.setTitle("Unknown");
                break;
            case STATE_OWNED_BY_PLAYER:
                images.ownedByPlayer().applyTo(curImage);
                // TODO - check for invalid here
                // TODO - update this when name changes - ugh
                curImage.setTitle("Owned by " + solver.playerNames[ownerIndex]);
                break;
            default:
                // STATE_OWNED_BY_CASEFILE
                // TODO - assert?
            case STATE_OWNED_BY_CASEFILE:
                images.ownedByCasefile().applyTo(curImage);
                curImage.setTitle("Solution!");
                break;
        }
    }

}
