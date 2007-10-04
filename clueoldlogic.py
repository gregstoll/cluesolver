#!/usr/bin/python

from logic import * #expr, conjuncts, disjuncts, to_cnf, unique, removeall, unify, NaryExpr, subst, FALSE, pretty
from utils import removeall
import logic

def fol_resolution(KB, alpha):
    # TODO - use set of support as alpha
    set_of_support = conjuncts(to_cnf(~alpha))
    clauses = KB.clauses
    new = set()
    while True:
        print "TODO top of loop: clauses is %s" % clauses
        print "TODO top of loop: set_of_support is %s" % set_of_support
        s = len(set_of_support)
        n = len(clauses)
        pairs = [(set_of_support[i], clauses[j])
                 for i in range(s) for j in range(n)]
        for (ci, cj) in pairs:
            resolvents = fol_resolve(ci, cj)
            if FALSE in resolvents: 
                #print "FALSE in resolvents from clauses: %s, %s" % (ci, cj)
                return True
            #if (resolvents != []):
            #    print "TODO - resolvents is %s" % resolvents
            new = new.union(set(resolvents))
        if new.issubset(set(clauses).union(set(set_of_support))): return False
        for c in new:
            if c not in clauses and c not in set_of_support: set_of_support.append(c)
        print "looping"

def fol_resolve(ci, cj):
    """Return all clauses that can be obtained by resolving clauses ci and cj.
    >>> for res in pl_resolve(to_cnf(A|B|C), to_cnf(~B|~C|F)):
    ...    ppset(disjuncts(res))
    set([A, C, F, ~C])
    set([A, B, F, ~B])
    """
    #print "TODO - fol_resolve: %s, %s" % (ci, cj)
    clauses = []
    for di in disjuncts(ci):
        for dj in disjuncts(cj):
            #print "TODO - di, dj: %s, %s" % (di, dj)
            dnew = None
            if di == ~dj or ~di == dj:
                dnew = unique(removeall(di, disjuncts(ci)) + 
                              removeall(dj, disjuncts(cj)))
            else:
                notDj = ~dj
                if (notDj.op == '~' and notDj.args[0].op == '~'):
                    notDj = notDj.args[0].args[0]
                unifySubst = unify(di, notDj, {})
                if (unifySubst != None):
                    #print "TODO - unifySubst is %s" % unifySubst
                    s = subst(unifySubst, di)
                    t = disjuncts(subst(unifySubst, ci))
                    #print "TODO - subst(unifySubst, di) is %s" % s
                    #print "TODO - disjuncts(subst(unifySubst, ci)) is %s" % t
                    #print "TODO - removeall(subst(unifySubst,di),disjuncts(subst(unifySubst,ci))) is %s" % removeall(s, t)
                    #print "TODO - subst(unifySubst, dj) is %s" % subst(unifySubst, dj)
                    #print "TODO - disjuncts(subst(unifySubst, cj)) is %s" % disjuncts(subst(unifySubst, cj))
                    dnew = unique(removeall(subst(unifySubst, di), disjuncts(subst(unifySubst, ci))) + 
                              removeall(subst(unifySubst, dj), disjuncts(subst(unifySubst, cj))))
                    #print "TODO -  unified! dnew is %s" % dnew
            if (dnew != None):
                clauses.append(NaryExpr('|', *dnew))
    #if len(clauses) > 0:
        #print "TODO - fol_resolve %s, %s is %s" % (ci, cj, clauses)
    return clauses


def main():
    cards = [['ProfessorPlum', 'MissScarlet', 'MissWhite'],
             ['Ballroom', 'Conservatory'],
             ['Knife', 'Wrench']]
    players = ['P1', 'P2']
    sentences = []
#    'Player(P1)',
#                 'Player(P2)',
#                 'Player(NoOne)',
#                 'Player(x) ==> (Eq(x,P1) | Eq(x,P2) | Eq(x,NoOne))',
#                 'Card(c) ==> (Player(O(c)) & HasCard(O(c), c))',
#                 '(HasCard(p,c) & HasCard(q,c)) ==> Eq(p,q)',
#                 'Eq(x,x)',
#                 'Eq(x,y) ==> Eq(y,x)',
#                 '(Eq(x,y) & Eq(y,z)) ==> Eq(x,z)',
#                 'Eq(x,y) ==> Eq(O(x), O(y))',
#                 '(Eq(w,y) & Eq(x,z)) ==> (HasCard(w,x) <=> HasCard(y,z))',
#                 'Eq(x,y) ==> (Player(x) <=> Player(y))']
    #sentences =

    for group in cards:
        # Someone and only one person owns each card.
        for card in group:
            sentence = ""
            for player in players:
                sentence += 'HasCard(' + player + ',' + card + ') | '
            sentence += 'HasCard(NoOne,' + card + ')'
            sentences.append(sentence)
            for player in (players + ['NoOne']):
                sentence = 'HasCard('+player+','+card+') ==> ('
                clauses = []
                for otherplayer in (players + ['NoOne']):
                    if (otherplayer != player):
                        clauses.append('~HasCard('+otherplayer+','+card+')')
                sentence += ' & '.join(clauses) + ')'
                sentences.append(sentence)
        # In every category, NoOne owns one and only one card
        clauses = []
        for card in group:
            clauses.append('HasCard(NoOne,'+card+')')
        sentence = ' | '.join(clauses)
        sentences.append(sentence)
        for card in group:
            sentence = 'HasCard(NoOne,'+card+') ==> ('
            clauses = []
            for othercard in group:
                if (othercard != card):
                    clauses.append('~HasCard(NoOne,'+othercard+')')
            sentence += ' & '.join(clauses) + ')'
            sentences.append(sentence)

            

    testSentences = ['HasCard(P1,ProfessorPlum)',
                     'HasCard(P2,MissScarlet)']
#                     '~Eq(O(ProfessorPlum), P1)',
#                     '~Eq(O(ProfessorPlum), P2)']

#    kb = logic.FolKB()
    kb = logic.PropKB()
    for sentence in sentences:
        kb.tell(expr(sentence))
    for sentence in testSentences:
        kb.tell(expr(sentence))
    #print fol_resolution(kb, expr('HasCard(P1,ProfessorPlum)'))
    #print fol_resolution(kb, expr('Eq(P1,ProfessorPlum)'))
    #print fol_resolve(expr('~Eq(y,ProfessorPlum) | ~Eq(P1, y)'), expr('Eq(x,x)'))
#    print fol_resolve(standardize_apart(expr('~Eq(y,ProfessorPlum) | ~Eq(P1, y)')), standardize_apart(expr('Eq(x,x)')))
    #kb = logic.PropKB()
    #kb.tell(expr('F(x,y)|G(y)'))
    #print fol_resolution(kb, expr('F(A,B)'))
    #kb = logic.PropKB()
    #kb.tell(expr('P'))
    #print fol_resolution(kb, expr('P'))
    #print pl_resolution(kb, expr('HasCard(NoOne,ProfessorPlum)'))
    #print pl_resolution(kb, expr('HasCard(NoOne,MissScarlet)'))
    #print pl_resolution(kb, expr('HasCard(NoOne,MissWhite)'))
if (__name__ == '__main__'):
    main()
