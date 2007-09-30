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
        self.clueEngine.validateCard(card)
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
                if (card in self.hasCards):
                    # We have this card, so this clause is done.
                    self.possibleCards.remove(clause)
                elif (card in self.notHasCards):
                    clause.remove(card)
                    if (len(clause) == 1):
                        # We have this card!
                        self.hasCards.add(clause[0])
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

    # TODO - accusations as well
    # TODO - simulations as well?
    def suggest(self, suggestingPlayer, card1, card2, card3, refutingPlayer, cardShown):
        self.validateCard(card1)
        self.validateCard(card2)
        self.validateCard(card3)
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
        self.validateCard(card)
        for i in range(self.numPlayers+1):
            if (card in self.players[i].hasCards):
                return i
        return -1

    def playerHasCard(self, playerIndex, card):
        self.validateCard(card)
        if (card in self.players[playerIndex].hasCards):
            return True
        elif (card in self.players[playerIndex].notHasCards):
            return False
        else:
            # Don't know
            return -1
    
    def validateCard(self, card):
        for cardtype in self.cards:
            if card in self.cards[cardtype]:
                return
        raise "ERROR - unrecognized card %s" % card

    def checkSolution(self, card):
        noOneHasCard = True
        someoneHasCard = False
        # - Check also for all cards except one in a category are
        # accounted for.
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
        # Now see if we've deduced a solution.
        for cardtype in self.cards:
            allCards = self.cards[cardtype][:]
            solutionCard = None
            isSolution = True
            for testCard in allCards:
                # See if anyone has this card
                cardOwned = False
                for i in range(self.numPlayers):
                    if (testCard in self.players[i].hasCards):
                        # someone has it, mark it as such
                        cardOwned = True
                if (cardOwned == False):
                    # If there's another possibility, we don't know which is
                    # right.
                    if (solutionCard != None):
                        solutionCard = None
                        isSolution = False
                    else:
                        solutionCard = testCard
            if (isSolution):
                # There's only one possibility, so this must be it!
                if (solutionCard not in self.players[self.numPlayers].hasCards):
                    self.players[self.numPlayers].hasCards.add(solutionCard)

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
        self.assertEqual(ce.playerHasCard(1, 'ProfessorPlum'), False)
        self.assertEqual(ce.playerHasCard(1, 'ColonelMustard'), -1)
        self.assertEqual(ce.playerHasCard(0, 'ColonelMustard'), -1)
        self.assertEqual(ce.playerHasCard(0, 'ProfessorPlum'), False)

    def testPossibleCards1(self):
        ce = ClueEngine()
        self.assertEqual(len(ce.players[3].possibleCards), 0)
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, None)
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], ['ProfessorPlum', 'Knife', 'Hall'])
        ce.infoOnCard(3, 'Hall', True)
        self.assertEqual(ce.whoHasCard('Hall'), 3)
        self.assertEqual(ce.playerHasCard(3, 'Hall'), True)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testPossibleCards2(self):
        ce = ClueEngine()
        self.assertEqual(len(ce.players[3].possibleCards), 0)
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, None)
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], ['ProfessorPlum', 'Knife', 'Hall'])
        ce.infoOnCard(3, 'Hall', False)
        self.assertEqual(ce.playerHasCard(3, 'Hall'), False)
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], ['ProfessorPlum', 'Knife'])
        ce.infoOnCard(3, 'ProfessorPlum', False)
        self.assertEqual(ce.playerHasCard(3, 'ProfessorPlum'), False)
        self.assertEqual(ce.whoHasCard('Knife'), 3)
        self.assertEqual(ce.playerHasCard(3, 'Knife'), True)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testAllCardsAccountedFor(self):
        ce = ClueEngine()
        ce.infoOnCard(0, 'ColonelMustard', True)
        self.assertEqual(ce.playerHasCard(0, 'ColonelMustard'), True)
        ce.infoOnCard(1, 'MrGreen', True)
        ce.infoOnCard(2, 'MissScarlet', True)
        ce.infoOnCard(3, 'MsWhite', True)
        ce.infoOnCard(4, 'MrsPeacock', True)
        self.assertEqual(ce.playerHasCard(6, 'ProfessorPlum'), True)

def main():
    ce = ClueEngine()

if (__name__ == '__main__'):
    testRunner = unittest.TextTestRunner()
    testSuite = unittest.TestLoader().loadTestsFromTestCase(TestCaseClueEngine)
    testRunner.run(testSuite)
    main()
