#!/usr/bin/python

import unittest, sets

class PlayerData:
    def __init__(self, clueengine):
        # A set of cards that the player is known to have
        self.hasCards = sets.Set()
        # A set of cards that the player is known not to have
        self.notHasCards = sets.Set()
        # A list of clauses.  Each clauses is a list of cards, one of which
        # the player is known to have.
        self.possibleCards = []
        self.clueEngine = clueengine

    def __repr__(self):
        return "PD(%s,%s,%s)" % (self.hasCards, self.notHasCards, self.possibleCards)

    def infoOnCard(self, card, hasCard, updateClueEngine=True):
        if (hasCard):
            self.hasCards.add(card)
        else:
            self.notHasCards.add(card)
        self.examineClauses(card)
        if (updateClueEngine):
            self.clueEngine.checkSolution(card)

    def hasOneOfCards(self, cards):
        newClause = cards
        for card in newClause:
            if (card in self.hasCards):
                # We already know player has one of these cards, so this
                # clause is worthless.
                newClause = []
            elif (card in self.notHasCards):
                # We know player doesn't have this card, so remove this card
                # from the clause
                newClause.remove(card)
        if (len(newClause) > 0):
            if (len(newClause) == 1):
                # We have learned player has this card!
                self.infoOnCard(newClause[0], True)
            else:
                self.possibleCards.append(newClause)

    def examineClauses(self, card):
        for clause in self.possibleCards:
            if (card in clause):
                clause.remove(card)
                if (len(clause) == 1):
                    # We have this card!
                    self.hasCards.add(card)
                    self.possibleCards.remove(clause)

class ClueEngine:
    cards = {}
    cards['suspect'] = ['ProfessorPlum', 'ColonelMustard', 'MrGreen', 'MissScarlet', 'MsWhite', 'MrsPeacock']
    cards['weapon'] = ['Knife', 'Candlestick', 'Revolver', 'LeadPipe', 'Rope', 'Wrench']
    cards['room'] = ['Hall', 'Conservatory', 'DiningRoom', 'Kitchen', 'Study', 'Library', 'Ballroom', 'Lounge', 'BilliardRoom']
    def __init__(self, players=6):
        self.numPlayers = players
        self.players = [PlayerData(self) for i in range(self.numPlayers+1)]

    def infoOnCard(self, playerIndex, card, hasCard):
        self.players[playerIndex].infoOnCard(card, hasCard)

    def suggest(self, suggestingPlayer, card1, card2, card3, refutingPlayer, cardShown):
        curPlayer = suggestingPlayer + 1
        if (curPlayer == self.numPlayers):
            curPlayer = 0
        while True:
            if (refutingPlayer == curPlayer):
                if (cardShown != None):
                    self.players[curPlayer].infoOnCard(cardShown, True)
                else:
                    self.players[curPlayer].hasOneOfCards([card1, card2, card3])
                return
            elif (suggestingPlayer == curPlayer):
                # No one can refute this.  We're done.
                return
            else:
                self.players[curPlayer].infoOnCard(card1, False)
                self.players[curPlayer].infoOnCard(card2, False)
                self.players[curPlayer].infoOnCard(card3, False)
            curPlayer += 1
            if (curPlayer == self.numPlayers):
                curPlayer = 0

    def whoHasCard(self, card):
        for i in range(self.numPlayers+1):
            if (card in self.players[i].hasCards):
                return i
        return -1

    def checkSolution(self, card):
        noOneHasCard = True
        someoneHasCard = False
        for i in range(self.numPlayers):
            if (card in self.players[i].hasCards):
                # Someone has the card, so the solution is not this
                self.players[self.numPlayers].infoOnCard(card, False, updateClueEngine=False)
                someoneHasCard = True
                noOneHasCard = False
            elif (card in self.players[i].notHasCards):
                pass
            else:
                noOneHasCard = False
        if (noOneHasCard):
            # Solution - no one has this card!
            self.players[self.numPlayers].infoOnCard(card, True, updateClueEngine=False)
            # update notHasCard for everything else in this category
            for cardType in self.cards:
                if (card in self.cards[cardType]):
                    for otherCard in self.cards[cardType]:
                        if (otherCard != card):
                            self.players[self.numPlayers].infoOnCard(otherCard, False)
        elif (someoneHasCard):
            # Someone has this card, so no one else does. (including solution)
            for i in range(self.numPlayers + 1):
                if (card not in self.players[i].hasCards):
                    self.players[i].infoOnCard(card, False, updateClueEngine=False)
            

class TestCaseClueEngine(unittest.TestCase):
    def setUp(self):
        pass

    def testSimpleSuggest(self):
        ce = ClueEngine()
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, 'Knife')
        self.assert_('Knife' in ce.players[3].hasCards)
        self.assertEqual(len(ce.players[3].notHasCards), 0)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testSuggestNoRefute(self):
        ce = ClueEngine()
        ce.suggest(1, 'ProfessorPlum', 'Knife', 'Hall', None, None)
        ce.infoOnCard(1, 'ProfessorPlum', False)
        self.assert_('ProfessorPlum' in ce.players[ce.numPlayers].hasCards)
        self.assert_('ColonelMustard' in ce.players[ce.numPlayers].notHasCards)
        self.assert_('Knife' not in ce.players[ce.numPlayers].hasCards)
        self.assert_('Hall' not in ce.players[ce.numPlayers].hasCards)
        self.assert_('ProfessorPlum' in ce.players[1].notHasCards)
        self.assert_('Knife' not in ce.players[1].notHasCards)
        self.assert_('Knife' not in ce.players[1].hasCards)
        self.assert_('Knife' in ce.players[2].notHasCards)
        self.assert_('Knife' in ce.players[0].notHasCards)

def main():
    ce = ClueEngine()

if (__name__ == '__main__'):
    testRunner = unittest.TextTestRunner()
    testSuite = unittest.TestLoader().loadTestsFromTestCase(TestCaseClueEngine)
    testRunner.run(testSuite)
    main()
