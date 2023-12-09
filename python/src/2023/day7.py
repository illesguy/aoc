from inputs import get_input_for_day


class Hand:

    def __init__(self, cards, bid):
        self.cards = cards
        self.bid = bid

    def __gt__(self, other):
        if self.kind() > other.kind():
            return True
        if self.kind() < other.kind():
            return False
        return self._hand_is_higher(other)

    def kind(self):
        if self._five_of_a_kind():
            return 6
        if self._four_of_a_kind():
            return 5
        if self._full_house():
            return 4
        if self._three_of_a_kind():
            return 3
        if self._two_pairs():
            return 2
        if self._one_pair():
            return 1
        return 0

    def _five_of_a_kind(self):
        return len(set(self.cards)) == 1

    def _four_of_a_kind(self):
        first_char_count = self.cards.count(self.cards[0])
        return len(set(self.cards)) == 2 and (first_char_count == 1 or first_char_count == 4)

    def _full_house(self):
        first_char_count = self.cards.count(self.cards[0])
        return len(set(self.cards)) == 2 and (first_char_count == 2 or first_char_count == 3)

    def _three_of_a_kind(self):
        return len(set(self.cards)) == 3 and all([self.cards.count(c) == 3 or self.cards.count(c) == 1 for c in self.cards])

    def _two_pairs(self):
        return len(set(self.cards)) == 3 and all([self.cards.count(c) == 2 or self.cards.count(c) == 1 for c in self.cards])

    def _one_pair(self):
        return len(set(self.cards)) == 4

    def __repr__(self):
        return self.cards

    def _hand_is_higher(self, other):
        for s, o in zip(self.cards, other.cards):
            if Hand.cards_to_int(s) > Hand.cards_to_int(o):
                return True
            if Hand.cards_to_int(s) < Hand.cards_to_int(o):
                return False
        return False

    @staticmethod
    def cards_to_int(card):
        if card == 'A':
            return 14
        if card == 'K':
            return 13
        if card == 'Q':
            return 12
        if card == 'J':
            return 11
        if card == 'T':
            return 10
        return int(card)


class JokerHand(Hand):

    def _hand_is_higher(self, other):
        for s, o in zip(self.cards, other.cards):
            if JokerHand.cards_to_int(s) > JokerHand.cards_to_int(o):
                return True
            if JokerHand.cards_to_int(s) < JokerHand.cards_to_int(o):
                return False
        return False

    @staticmethod
    def cards_to_int(card):
        if card == 'A':
            return 14
        if card == 'K':
            return 13
        if card == 'Q':
            return 12
        if card == 'J':
            return 1
        if card == 'T':
            return 10
        return int(card)

    def _get_best_card(self):
        return max({c for c in self.cards if c != 'J'}, key=lambda c: (self.cards.count(c), JokerHand.cards_to_int(c)))

    def _five_of_a_kind(self):
        if self.cards == 'JJJJJ':
            return True
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        res = len(set(cards_to_use)) == 1
        # print("Five of a kind")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    def _four_of_a_kind(self):
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        first_char_count = cards_to_use.count(cards_to_use[0])
        res = len(set(cards_to_use)) == 2 and (first_char_count == 1 or first_char_count == 4)
        # print("Four of a kind")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    def _full_house(self):
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        first_char_count = cards_to_use.count(cards_to_use[0])
        res = len(set(cards_to_use)) == 2 and (first_char_count == 2 or first_char_count == 3)
        # print("Full house")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    def _three_of_a_kind(self):
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        res = len(set(cards_to_use)) == 3 and all([cards_to_use.count(c) == 3 or cards_to_use.count(c) == 1 for c in cards_to_use])
        # print("Three of a kind")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    def _two_pairs(self):
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        res = len(set(cards_to_use)) == 3 and all([cards_to_use.count(c) == 2 or cards_to_use.count(c) == 1 for c in cards_to_use])
        # print("Two pairs")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    def _one_pair(self):
        best_card = self._get_best_card()
        cards_to_use = self.cards.replace('J', best_card)
        res = len(set(cards_to_use)) == 4
        # print("One pair")
        # print("Original cards",  self.cards)
        # print("Best card", best_card)
        # print("Replaced cards", cards_to_use)
        # print(res)
        # print("----")
        return res

    @classmethod
    def from_hand(cls, hand):
        return cls(hand.cards, hand.bid)


def parse_input(line):
    cards, bid = line.split()
    return Hand(cards, int(bid))


def part1(inp):
    return sum([h.bid * (i + 1) for i, h in enumerate(sorted(inp))])


def part2(inp):
    return part1([JokerHand.from_hand(h) for h in inp])


inp = get_input_for_day(2023, 7, parse_input)
print("Input lines read:", len(inp))
print("part 1:", part1(inp))
print("part 2:", part2(inp))
