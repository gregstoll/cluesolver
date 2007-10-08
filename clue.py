#!/usr/bin/python

import cgi
import clueengine

form = cgi.FieldStorage()
if (form.has_key('sess')):
    (engine, str) = clueengine.ClueEngine.loadFromString(form.getfirst('sess'))
else:
    engine = clueengine.ClueEngine()

