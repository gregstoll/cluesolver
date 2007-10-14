package com.gregstoll.cluesolver.client;

import com.google.gwt.core.client.GWT;
import com.google.gwt.user.client.Window;
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

    public int[] ownerIndices = {};
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

    public void setState(int state, int[] owners) {
        curState = state;
        if (owners != null) {
            ownerIndices = new int[owners.length];
            for (int i = 0; i < ownerIndices.length; ++i) {
                ownerIndices[i] = owners[i];
            }
        } else {
            ownerIndices = null;
        }
        setImage();
    }

    private String getOwnedByString() {
        if (ownerIndices.length == 0) {
            return "???";
        }
        StringBuffer buffer = new StringBuffer();
        for (int i = 0; i < ownerIndices.length; ++i) {
            String ownerName;
            int curIndex = ownerIndices[i];
            if (curIndex == solver.playerNames.size()) {
                ownerName = "(solution)";
            } else {
                ownerName = (String) solver.playerNames.get(curIndex);
            }
            buffer.append(ownerName);
            if (i < ownerIndices.length - 1) {
                buffer.append(" or ");
            }
        }
        return buffer.toString();
    }

    public void setImage() {
        switch (curState) {
            case STATE_UNKNOWN:
                images.unknown().applyTo(curImage);
                if (ownerIndices != null && ownerIndices.length > 0) {
                    curImage.setTitle("Owned by " + getOwnedByString());
                } else {
                    curImage.setTitle("Unknown");
                }
                break;
            case STATE_OWNED_BY_PLAYER:
                images.ownedByPlayer().applyTo(curImage);
                curImage.setTitle("Owned by " + getOwnedByString());
                break;
            default:
                // STATE_OWNED_BY_CASEFILE
            case STATE_OWNED_BY_CASEFILE:
                images.ownedByCasefile().applyTo(curImage);
                curImage.setTitle("Solution!");
                break;
        }
    }

}
