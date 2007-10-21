#!/usr/bin/python

import unittest, sets, copy

class PlayerData:
    def __init__(self, clueengine, numCards, isSolutionPlayer=False):
        # A set of cards that the player is known to have
        self.hasCards = sets.Set()
        # A set of cards that the player is known not to have
        self.notHasCards = sets.Set()
        # A list of clauses.  Each clause is a set of cards, one of which
        # the player is known to have.
        self.possibleCards = []
        self.clueEngine = clueengine
        self.isSolutionPlayer = isSolutionPlayer
        self.numCards = numCards

    def __repr__(self):
        return "PD(%d,%s,%s,%s)" % (self.numCards, self.hasCards, self.notHasCards, self.possibleCards)

    def infoOnCard(self, card, hasCard, updateClueEngine=True):
        changedCards = set()
        self.clueEngine.validateCard(card)
        if (hasCard):
            self.hasCards.add(card)
        else:
            self.notHasCards.add(card)
        changedCards.add(card)
        changedCards.update(self.examineClauses(card))
        if (updateClueEngine):
            changedCards.update(self.clueEngine.checkSolution(card))
        if (hasCard and self.isSolutionPlayer):
            # We know we have no other cards in this category.
            for cardType in self.clueEngine.cards:
                if (card in self.clueEngine.cards[cardType]):
                    for otherCard in self.clueEngine.cards[cardType]:
                        if (otherCard != card):
                            changedCards.update(self.infoOnCard(otherCard, False))
        return changedCards

    def hasOneOfCards(self, cards):
        changedCards = set()
        newClause = [] 
        clauseHelpful = True
        for card in cards:
            if (card in self.hasCards):
                # We already know player has one of these cards, so this
                # clause is worthless.
                newClause = []
                clauseHelpful = False
            elif (card in self.notHasCards):
                # We know player doesn't have this card, so don't add this card
                # to the new clause.
                pass
            else:
                # Don't know - add it to the clause.
                newClause.append(card)
        if (clauseHelpful and len(newClause) > 0):
            if (len(newClause) == 1):
                # We have learned player has this card!
                changedCards.update(self.infoOnCard(newClause[0], True))
            else:
                self.possibleCards.append(set(newClause))
            changedCards.update(self.examineClauses(None))
        return changedCards

    def eliminateExtraneousClauses(self):
        for i in range(len(self.possibleCards)):
            for j in range(i+1, len(self.possibleCards)):
                clause1 = self.possibleCards[i]
                clause2 = self.possibleCards[j]
                if (clause1.issubset(clause2)):
                    # clause 2 is extraneous
                    self.possibleCards = self.possibleCards[:j] + self.possibleCards[j+1:]
                    # The easiest way to check without messing up the loop is
                    # to start over, although it's kinda slow.  But I don't
                    # expect there to be tons of extraneous clauses.
                    return self.eliminateExtraneousClauses()
                elif (clause1.issuperset(clause2)):
                    # clause 1 is extraneous
                    self.possibleCards = self.possibleCards[:i] + self.possibleCards[i+1:]
                    # See above
                    return self.eliminateExtraneousClauses()

    def transposeClauses(self, possibleCards):
        cardClauses = {}
        for i in range(len(possibleCards)):
            clause = possibleCards[i]
            for card in clause:
                if card in cardClauses:
                    cardClauses[card].add(i)
                else:
                    cardClauses[card] = set([i])
        return cardClauses

    def removeCardFromClauses(self, clauses, card):
        newClauses = []
        for clause in clauses:
            # Copy so we don't destroy the original one
            newClause = set(list(clause))
            if (card in newClause):
                newClause.remove(card)
            newClauses.append(newClause)
        return newClauses

    def removeClauses(self, clauses, indicesToRemove):
        newClauses = []
        for i in range(len(clauses)):
            if (i not in indicesToRemove):
                newClauses.append(clauses[i])
        return newClauses
   
    def canSatisfy(self, clauses, numUnaccountedFor):
        if len(clauses) == 0:
            return True
        if numUnaccountedFor == 0:
            return False
        # See if there's any way we can satisfy these.
        # Try one card at a time.
        cardClauses = self.transposeClauses(clauses)
        for testCard in cardClauses:
            # First, remove all clauses containing this card.
            newClauses = self.removeClauses(clauses, cardClauses[testCard])
            # See if it's possible to satisfy the rest of the clauses with one fewer card.
            isPossible = self.canSatisfy(newClauses, numUnaccountedFor - 1)
            if (isPossible):
                return True
        return False

    def examineClauses(self, card):
        # Eliminate clauses.
        self.eliminateExtraneousClauses()
        changedCards = set()
        if (card != None):
            possibleCardsCopy = copy.copy(self.possibleCards)
            for clause in possibleCardsCopy:
                if (card in clause):
                    if (card in self.hasCards):
                        # We have this card, so this clause is done.
                        self.possibleCards.remove(clause)
                    elif (card in self.notHasCards):
                        # Remove this card from the clause
                        clause.remove(card)
                        if (len(clause) == 1):
                            # We have this card!
                            self.hasCards.add(list(clause)[0])
                            self.possibleCards.remove(clause)
                            changedCards.add(list(clause)[0])
        if (self.numCards != -1):
            if (self.numCards == len(self.hasCards)):
                # All cards are accounted for.
                for cardType in self.clueEngine.cards:
                    for card in self.clueEngine.cards[cardType]:
                        if (card not in self.hasCards and card not in self.notHasCards):
                            changedCards.update(self.infoOnCard(card, False))
            elif (len(self.hasCards) + len(self.possibleCards) > self.numCards):
                # We may be able to figure out something.
                numUnaccountedFor = self.numCards - len(self.hasCards)
                cardClauses = self.transposeClauses(self.possibleCards)
                for testCard in cardClauses:
                    # See if we could have this card, by contradiction.
                    # Assume we don't have this card.  Remove it from
                    # all clauses.
                    newClauses = self.removeCardFromClauses(self.possibleCards, testCard)
                    # If there are any empty clauses we have a contradiction already.
                    if (set() in newClauses):
                        isPossible = False
                    else:
                        # See if it's possible to satisfy the rest of the clauses with one fewer card.
                        isPossible = self.canSatisfy(newClauses, numUnaccountedFor)
                    if (not isPossible):
                        # We found a contradiction if we don't have this card,
                        # so we must have this card.
                        changedCards.update(self.infoOnCard(testCard, True))
                        return changedCards
        return changedCards

class ClueEngine:
    cards = {}
    cards['suspect'] = ['ProfessorPlum', 'ColonelMustard', 'MrGreen', 'MissScarlet', 'MsWhite', 'MrsPeacock']
    cards['weapon'] = ['Knife', 'Candlestick', 'Revolver', 'LeadPipe', 'Rope', 'Wrench']
    cards['room'] = ['Hall', 'Conservatory', 'DiningRoom', 'Kitchen', 'Study', 'Library', 'Ballroom', 'Lounge', 'BilliardRoom']
    def __init__(self, players=6):
        self.numPlayers = players
        self.players = [PlayerData(self, ClueEngine.getNumberOfCards(i, self.numPlayers), (i == self.numPlayers)) for i in range(self.numPlayers+1)]

    def __repr__(self):
        return "Engine:\n" + "\n".join([repr(x) for x in self.players])

    def __str__(self):
        return repr(self)

    @classmethod
    def getNumberOfCards(cls, playerIndex, numPlayers):
        if (playerIndex == numPlayers):
            # The case file always has exactly 3 cards, for what it's worth.
            return 3
        # There are 18 cards among the players.
        numCards = 18 / numPlayers # Integer division
        leftovers = 18 % numPlayers
        # Assume the earlier players get the extra cards
        if (playerIndex < leftovers):
            numCards += 1
        return numCards

    @classmethod
    def cardFromChar(cls, char):
        idx = ord(char) - ord('A')
        if idx < len(cls.cards['suspect']):
            return cls.cards['suspect'][idx]
        idx -= len(cls.cards['suspect'])
        if idx < len(cls.cards['weapon']):
            return cls.cards['weapon'][idx]
        idx -= len(cls.cards['weapon'])
        if idx < len(cls.cards['room']):
            return cls.cards['room'][idx]
        # invalid character
        return ''

    @classmethod
    def charFromCard(cls, card):
        idx = 0
        if (card in cls.cards['suspect']):
            idx += cls.cards['suspect'].index(card)
            return chr(idx + ord('A'))
        idx += len(cls.cards['suspect'])
        if (card in cls.cards['weapon']):
            idx += cls.cards['weapon'].index(card)
            return chr(idx + ord('A'))
        idx += len(cls.cards['weapon'])
        if (card in cls.cards['room']):
            idx += cls.cards['room'].index(card)
            return chr(idx + ord('A'))
        # invalid card
        return ''

    @classmethod
    def loadFromString(cls, str):
        numPlayers = int(str[0])
        str = str[1:]
        ce = ClueEngine(numPlayers)
        for i in range(numPlayers+1):
            str = cls.loadPlayerFromString(str, i, ce)
        return (ce, str)

    def writeToString(self):
        str = ''
        str += '%d' % self.numPlayers
        for i in range(self.numPlayers+1):
            str += self.writePlayerToString(i)
        return str

    @classmethod
    def loadPlayerFromString(cls, str, idx, ce):
        numCards = int(str[0])
        if (numCards == 0):
            numCards = -1
        ce.players[idx].numCards = numCards
        str = str[1:]
        # Load the list of cards this player has
        while (str[0] != '-'):
            ce.infoOnCard(idx, cls.cardFromChar(str[0]), True)
            str = str[1:]
        str = str[1:]
        # Load the list of cards this player doesn't have
        while (str[0] != '-' and str[0] != '.'):
            ce.infoOnCard(idx, cls.cardFromChar(str[0]), False)
            str = str[1:]
        # Load the list of clauses as long as it's not done
        while str[0] != '.':
            str = str[1:]
            clause = []
            while str[0] != '-' and str[0] != '.':
                clause.append(cls.cardFromChar(str[0]))
                str = str[1:]
            if (len(clause) > 0):
                ce.players[idx].hasOneOfCards(clause)
        str = str[1:]
        return str

    def writePlayerToString(self, idx):
        numCardsToWrite = self.players[idx].numCards
        # Always write one digit for simplicity
        if (numCardsToWrite == -1):
            numCardsToWrite = 0
        str = '%d' % numCardsToWrite
        for card in self.players[idx].hasCards:
            str += ClueEngine.charFromCard(card)
        str += '-'
        for card in self.players[idx].notHasCards:
            str += ClueEngine.charFromCard(card)
        if (len(self.players[idx].possibleCards) == 0):
            str += '.'
            return str
        str += '-'
        for possibleCardGroup in self.players[idx].possibleCards:
            for card in possibleCardGroup:
                str += ClueEngine.charFromCard(card)
            str += '-'
        # But we want a . at the end, not -
        str = str[:-1]
        str += '.'
        return str

    def infoOnCard(self, playerIndex, card, hasCard):
        return self.players[playerIndex].infoOnCard(card, hasCard)

    # TODO - accusations as well
    # TODO - simulations as well?
    def suggest(self, suggestingPlayer, card1, card2, card3, refutingPlayer, cardShown):
        changedCards = set()
        self.validateCard(card1)
        self.validateCard(card2)
        self.validateCard(card3)
        curPlayer = suggestingPlayer + 1
        if (curPlayer == self.numPlayers):
            curPlayer = 0
        while True:
            if (refutingPlayer == curPlayer):
                if (cardShown != None):
                    changedCards.update(self.players[curPlayer].infoOnCard(cardShown, True))
                else:
                    changedCards.update(self.players[curPlayer].hasOneOfCards([card1, card2, card3]))
                return changedCards
            elif (suggestingPlayer == curPlayer):
                # No one can refute this.  We're done.
                return changedCards
            else:
                changedCards.update(self.players[curPlayer].infoOnCard(card1, False))
                changedCards.update(self.players[curPlayer].infoOnCard(card2, False))
                changedCards.update(self.players[curPlayer].infoOnCard(card3, False))
            curPlayer += 1
            if (curPlayer == self.numPlayers):
                curPlayer = 0

    def whoHasCard(self, card):
        self.validateCard(card)
        possibleOwners = []
        for i in range(self.numPlayers+1):
            if (card in self.players[i].hasCards):
                return [i]
            elif (card not in self.players[i].notHasCards):
                possibleOwners.append(i)
        return possibleOwners

    def playerHasCard(self, playerIndex, card):
        self.validateCard(card)
        if (card in self.players[playerIndex].hasCards):
            return True
        elif (card in self.players[playerIndex].notHasCards):
            return False
        else:
            # Don't know
            return -1

    def getSimulationData(self):
        # TODO
        pass

    @classmethod
    def validateCard(cls, card):
        for cardtype in cls.cards:
            if card in cls.cards[cardtype]:
                return
        raise "ERROR - unrecognized card %s" % card

    def checkSolution(self, card):
        changedCards = set()
        someoneHasCard = False
        numWhoDontHaveCard = 0
        playerWhoMightHaveCard = -1
        # - Check also for all cards except one in a category are
        # accounted for.
        for i in range(self.numPlayers + 1):
            if (card in self.players[i].hasCards):
                # Someone has the card, so the solution is not this
                someoneHasCard = True
            elif (card in self.players[i].notHasCards):
                numWhoDontHaveCard += 1
            else:
                playerWhoMightHaveCard = i
        if ((not someoneHasCard) and (numWhoDontHaveCard == self.numPlayers)):
            # Every player except one doesn't have this card, so we know the player has it.
            changedCards.update(self.players[playerWhoMightHaveCard].infoOnCard(card, True, updateClueEngine=False))
        elif (someoneHasCard):
            # Someone has this card, so no one else does. (including solution)
            for i in range(self.numPlayers + 1):
                if (card not in self.players[i].hasCards):
                    changedCards.update(self.players[i].infoOnCard(card, False, updateClueEngine=False))
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
                    changedCards.add(solutionCard)
        return changedCards

class TestCaseClueEngine(unittest.TestCase):
    def makeSet(*args):
        a = set()
        for arg in args[1:]:
            a.add(arg)
        return a
        
    def setUp(self):
        pass

    def testSimpleSuggest(self):
        ce = ClueEngine()
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, 'Knife')
        self.assertEqual(cc, self.makeSet('ProfessorPlum', 'Knife', 'Hall'))
        self.assert_('Knife' in ce.players[3].hasCards)
        self.assertEqual(len(ce.players[3].notHasCards), 0)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testSuggestNoRefute(self):
        ce = ClueEngine()
        cc = ce.suggest(1, 'ProfessorPlum', 'Knife', 'Hall', None, None)
        self.assertEqual(cc, self.makeSet('ProfessorPlum', 'Knife', 'Hall'))
        cc = ce.infoOnCard(1, 'ProfessorPlum', False)
        self.assertEqual(cc, self.makeSet(*ce.cards['suspect']))
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
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, None)
        self.assertEqual(ce.whoHasCard('ProfessorPlum'), [0,3,4,5,6])
        self.assertEqual(cc, self.makeSet('ProfessorPlum', 'Knife', 'Hall'))
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], set(['ProfessorPlum', 'Knife', 'Hall']))
        cc = ce.infoOnCard(3, 'Hall', True)
        self.assertEqual(cc, self.makeSet('Hall'))
        self.assertEqual(ce.whoHasCard('Hall'), [3])
        self.assertEqual(ce.playerHasCard(3, 'Hall'), True)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testPossibleCards2(self):
        ce = ClueEngine()
        self.assertEqual(len(ce.players[3].possibleCards), 0)
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 3, None)
        self.assertEqual(cc, self.makeSet('ProfessorPlum', 'Knife', 'Hall'))
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], set(['ProfessorPlum', 'Knife', 'Hall']))
        cc = ce.infoOnCard(3, 'Hall', False)
        self.assertEqual(cc, self.makeSet('Hall'))
        self.assertEqual(ce.playerHasCard(3, 'Hall'), False)
        self.assertEqual(len(ce.players[3].possibleCards), 1)
        self.assertEqual(ce.players[3].possibleCards[0], set(['ProfessorPlum', 'Knife']))
        cc = ce.infoOnCard(3, 'ProfessorPlum', False)
        self.assertEqual(cc, self.makeSet('ProfessorPlum', 'Knife'))
        self.assertEqual(ce.playerHasCard(3, 'ProfessorPlum'), False)
        self.assertEqual(ce.whoHasCard('Knife'), [3])
        self.assertEqual(ce.playerHasCard(3, 'Knife'), True)
        self.assertEqual(len(ce.players[3].possibleCards), 0)

    def testAllCardsAccountedFor(self):
        ce = ClueEngine()
        cc = ce.infoOnCard(0, 'ColonelMustard', True)
        self.assertEqual(cc, self.makeSet('ColonelMustard'))
        self.assertEqual(ce.playerHasCard(0, 'ColonelMustard'), True)
        cc = ce.infoOnCard(1, 'MrGreen', True)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        cc = ce.infoOnCard(2, 'MissScarlet', True)
        self.assertEqual(cc, self.makeSet('MissScarlet'))
        cc = ce.infoOnCard(3, 'MsWhite', True)
        self.assertEqual(cc, self.makeSet('MsWhite'))
        cc = ce.infoOnCard(4, 'MrsPeacock', True)
        self.assertEqual(cc, self.makeSet('MrsPeacock', 'ProfessorPlum'))
        self.assertEqual(ce.playerHasCard(6, 'ProfessorPlum'), True)

    def testSingleCardAccountedForNotSolution(self):
        ce = ClueEngine()
        cc = ce.infoOnCard(6, 'ColonelMustard', True)
        self.assertEqual(cc, self.makeSet(*ce.cards['suspect']))
        self.assertEqual(ce.playerHasCard(6, 'ColonelMustard'), True)
        cc = ce.infoOnCard(0, 'MrGreen', False)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        cc = ce.infoOnCard(1, 'MrGreen', False)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        cc = ce.infoOnCard(2, 'MrGreen', False)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        cc = ce.infoOnCard(3, 'MrGreen', False)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        cc = ce.infoOnCard(4, 'MrGreen', False)
        self.assertEqual(cc, self.makeSet('MrGreen'))
        self.assertEqual(ce.playerHasCard(5, 'MrGreen'), True)

    def testNumberCardLimit(self):
        ce = ClueEngine()
        ce.infoOnCard(0, 'MrGreen', True)
        ce.infoOnCard(0, 'Knife', True)
        ce.infoOnCard(0, 'Wrench', True)
        self.assertEqual(len(ce.players[0].hasCards), 3)
        self.assertEqual(len(ce.players[0].notHasCards), 18)
        self.assertEqual(len(ce.players[0].possibleCards), 0)

    def testNumberCardDeduction(self):
        ce = ClueEngine()
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Knife', 'Hall']))
        cc = ce.suggest(0, 'ProfessorPlum', 'Revolver', 'Lounge', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Revolver', 'Lounge']))
        cc = ce.suggest(0, 'ProfessorPlum', 'Candlestick', 'BilliardRoom', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Candlestick', 'BilliardRoom']))
        cc = ce.suggest(0, 'ProfessorPlum', 'Rope', 'Kitchen', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Rope', 'Kitchen']))
        self.assertEqual(ce.whoHasCard('ProfessorPlum'), [2])
        ce = ClueEngine()
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 2, None)
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Lounge', 2, None)
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'BilliardRoom', 2, None)
        self.assertEqual(len(ce.players[2].possibleCards), 3)
        ce.suggest(0, 'ProfessorPlum', 'Knife', 'Kitchen', 2, None)
        cc = ce.infoOnCard(2, 'ProfessorPlum', False)
        self.assertEqual(cc, set(['Knife', 'ProfessorPlum']))
        self.assertEqual(ce.whoHasCard('Knife'), [2])
        self.assertEqual(len(ce.players[2].possibleCards), 0)

    def testEliminateExtraneousClauses(self):
        ce = ClueEngine()
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Hall', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Knife', 'Hall']))
        cc = ce.infoOnCard(2, 'Hall', False)
        self.assertEqual(cc, set(['Hall']))
        self.assertEqual(len(ce.players[2].possibleCards), 1)
        self.assertEqual(ce.players[2].possibleCards[0], set(['ProfessorPlum', 'Knife']))
        cc = ce.suggest(0, 'ProfessorPlum', 'Knife', 'Lounge', 2, None)
        self.assertEqual(cc, set(['ProfessorPlum', 'Knife', 'Lounge']))
        self.assertEqual(len(ce.players[2].possibleCards), 1)
        self.assertEqual(ce.players[2].possibleCards[0], set(['ProfessorPlum', 'Knife']))
 
    def testCardFromChar(self):
        self.assertEqual(ClueEngine.cardFromChar('A'), 'ProfessorPlum')
        self.assertEqual(ClueEngine.cardFromChar('B'), 'ColonelMustard')
        self.assertEqual(ClueEngine.cardFromChar('F'), 'MrsPeacock')
        self.assertEqual(ClueEngine.cardFromChar('G'), 'Knife')
        self.assertEqual(ClueEngine.cardFromChar('L'), 'Wrench')
        self.assertEqual(ClueEngine.cardFromChar('M'), 'Hall')
        self.assertEqual(ClueEngine.cardFromChar('U'), 'BilliardRoom')
        self.assertEqual(ClueEngine.cardFromChar('V'), '')
        for i in range(ord('A'), ord('V')):
            ClueEngine.validateCard(ClueEngine.cardFromChar(chr(i)))

    def testCharFromCard(self):
        self.assertEqual(ClueEngine.charFromCard('ProfessorPlum'), 'A')
        self.assertEqual(ClueEngine.charFromCard('ColonelMustard'), 'B')
        self.assertEqual(ClueEngine.charFromCard('MrsPeacock'), 'F')
        self.assertEqual(ClueEngine.charFromCard('Knife'), 'G')
        self.assertEqual(ClueEngine.charFromCard('Wrench'), 'L')
        self.assertEqual(ClueEngine.charFromCard('Hall'), 'M')
        self.assertEqual(ClueEngine.charFromCard('BilliardRoom'), 'U')
        self.assertEqual(ClueEngine.charFromCard('InvalidCard'), '')
        for i in range(ord('A'), ord('V')):
            self.assertEqual(chr(i), ClueEngine.charFromCard(ClueEngine.cardFromChar(chr(i))))


    def testLoadFromString(self):
        (ce, str) = ClueEngine.loadFromString('29AH-BCD-KL-MN.9-AH.3-.')
        self.assertEqual(str, '')
        (ce, str) = ClueEngine.loadFromString('29A-.9-.3-.')
        self.assertEqual(str, '')
        self.assertEqual(len(ce.players[0].hasCards), 1)
        self.assert_(ce.playerHasCard(0, 'ProfessorPlum'))
        self.assertEqual(len(ce.players[0].notHasCards), 0)
        self.assertEqual(len(ce.players[1].hasCards), 0)
        self.assertEqual(len(ce.players[1].notHasCards), 1)
        self.assertEqual(ce.playerHasCard(1, 'ProfessorPlum'), False)
        self.assertEqual(len(ce.players[2].hasCards), 0)
        self.assertEqual(len(ce.players[2].notHasCards), 1)
        self.assertEqual(ce.playerHasCard(2, 'ProfessorPlum'), False)
        (ce, str) = ClueEngine.loadFromString('29A-B.9L-C.3U-.')
        self.assertEqual(str, '')
        self.assert_(ce.playerHasCard(0, 'ProfessorPlum'))
        self.assertEqual(ce.playerHasCard(0, 'ColonelMustard'), False)
        self.assert_(ce.playerHasCard(1, 'Wrench'))
        self.assertEqual(ce.playerHasCard(1, 'MrGreen'), False)
        self.assert_(ce.playerHasCard(2, 'BilliardRoom'))
        (ce, str) = ClueEngine.loadFromString('29-.9A-B-CDE-FGH.3U-.')
        self.assertEqual(str, '')
        self.assert_(ce.playerHasCard(1, 'ProfessorPlum'))
        self.assertEqual(ce.playerHasCard(1, 'ColonelMustard'), False)
        self.assertEqual(len(ce.players[1].possibleCards), 2)
        self.assertEqual(ce.players[1].possibleCards[0], set([ClueEngine.cardFromChar('C'), ClueEngine.cardFromChar('D'), ClueEngine.cardFromChar('E')]))
        self.assertEqual(ce.players[1].possibleCards[1], set([ClueEngine.cardFromChar('F'), ClueEngine.cardFromChar('G'), ClueEngine.cardFromChar('H')]))
       
    def testWriteToString(self):
        str = '29AH-BCD-KL-NM.9-AH.3-AH.'
        self.assertEqual(str, ClueEngine.loadFromString(str)[0].writeToString())
        str = '29-AU.9A-BU-ECD-FH.3U-ANSQRTOMP.'
        self.assertEqual(str, ClueEngine.loadFromString(str)[0].writeToString())

def main():
    ce = ClueEngine()

if (__name__ == '__main__'):
    testRunner = unittest.TextTestRunner()
    testSuite = unittest.TestLoader().loadTestsFromTestCase(TestCaseClueEngine)
    testRunner.run(testSuite)
    main()
