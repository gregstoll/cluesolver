#!/usr/bin/python

import elementtree.ElementTree as ET
import cgi, sys
import clueengine

def error(str):
    print "Content-type: text/xml\n\n"
    print "<data><errorStatus>1</errorStatus><errorText>%s</errorText></data>" % str
    sys.exit(0)

def success(str):
    print "Content-type: text/xml\n\n"
    print "<data><errorStatus>0</errorStatus>%s</data>" % str
    sys.exit(0)

form = cgi.FieldStorage()
#success(form)
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
    success('<session>%s</session>' % engine.writeToString())
if (action == 'whoOwns'):
    # See who owns what.
    if (not form.has_key('owner') or not form.has_key('card')):
        error("Internal error: action=whoOwns, missing owner or card!")
    owner = int(form.getfirst('owner'))
    card = form.getfirst('card')
    engine.infoOnCard(owner, card, True)
    success('<newInfo><card>%s</card><status>1</status><owner>%d</owner></newInfo><session>%s</session>' % (card, owner, engine.writeToString()))
