package com.gregstoll.cluesolver.client;

import com.google.gwt.core.client.GWT;
import com.google.gwt.user.client.ui.SimplePanel;
import com.google.gwt.user.client.ui.Image;
import com.google.gwt.user.client.ui.ImageBundle;
import com.google.gwt.user.client.ui.AbstractImagePrototype;

public class ClueStateWidget extends SimplePanel {
    public static final int STATE_UNKNOWN = 0;
    public static final int STATE_OWNED_BY_PLAYER = 1;
    public static final int STATE_OWNED_BY_CASEFILE = 2;

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

    public ClueStateWidget() {
        curState = STATE_UNKNOWN;
        setImage();
    }

    public void setImage() {
        Image i;
        switch (curState) {
            case STATE_UNKNOWN:
                i = images.unknown().createImage();
                i.setTitle("Unknown");
                break;
            case STATE_OWNED_BY_PLAYER:
                i = images.ownedByPlayer().createImage();
                i.setTitle("Owned by player ");
                // TODO - fix alt text
                break;
            default:
                // STATE_OWNED_BY_CASEFILE
                // TODO - assert?
            case STATE_OWNED_BY_CASEFILE:
                i = images.ownedByCasefile().createImage();
                i.setTitle("Solution!");
                break;
        }
        setWidget(i);
    }

}
