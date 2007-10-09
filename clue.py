#!/usr/bin/python

import elementtree.ElementTree as ET
import cgi, sys
import clueengine

def error(str):
    print "Content-type: application/json\n\n"
    print '{"errorStatus": 1, "errorText": "%s"}' % str
    sys.exit(0)

def success(str):
    print "Content-type: application/json\n\n"
    print '{"errorStatus": 0, %s}' % str
    sys.exit(0)

form = cgi.FieldStorage()
action = None
if (form.has_key('action')):
    action = form.getfirst('action')
else:
    error("Internal error - No action specified!")
# Valid actions are 'new', 'whoOwns', 'suggestion', 'moreInfo' ('accusation' in the future?)
if (action != 'new' and action != 'whoOwns' and action != 'suggestion' and action != 'moreInfo'):
    error("Internal error - invalid action '%s'!" % action)
if (action != 'new' and (not form.has_key('sess'))):
    error("Internal error - missing sess!")
if (action != 'new'):
    (engine, str) = clueengine.ClueEngine.loadFromString(form.getfirst('sess'))
    if (str != ''):
        error("Internal error - invalid session string '%s'!" % form.getfirst('sess'))
else:
    if (not form.has_key('players')):
        error("Internal error - action new without players!")
    engine = clueengine.ClueEngine(int(form.getfirst('players')))
if (action == 'new'):
    # This is all we have to do.
    success('"session": "%s"' % engine.writeToString())
if (action == 'whoOwns'):
    # See who owns what.
    if (not form.has_key('owner') or not form.has_key('card')):
        error("Internal error: action=whoOwns, missing owner or card!")
    owner = int(form.getfirst('owner'))
    card = form.getfirst('card')
    engine.infoOnCard(owner, card, True)
    # status = 1 means owned by player
    status = 1
    if (owner == engine.numPlayers):
        # status = 1 means owned by case file
        status = 2
    success('"newInfo": [{"card": "%s", "status": %d, "owner": %d}], "session": "%s"' % (card, status, owner, engine.writeToString()))
