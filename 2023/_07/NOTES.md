# NOTES

## For sorting of Hand<T>
Instead of working with own cmp function applied in .sort_by(|a, b| a.cmp(&b)), one could implement the Ord trait to allow .sort().
For that, Eq, PartialEq, PartialOrd must be derived and in specific cases PartialEq and PartialOrd must be implemented
     otherwise: When derived on structs, it will produce a lexicographic ordering based 
     on the top-to-bottom declaration order of the struct’s members

```
impl <T: CardTrait<T>> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_ordering = self.typ.cmp(&other.typ);
        if hand_ordering.is_eq() {
            for idx in 0..POKER_HAND_SIZE {
                let o = self.cards[idx].cmp(&other.cards[idx]);
                if o != Ordering::Equal { return o; }
            }
            println!("completely equal hands found: {:?}, {:?}", self.cards, other.cards);
            Ordering::Equal // should not happen actually
        } else {
            hand_ordering
        }
    }
}
```

Careful, you can also derive Ord, but this will make again the sorting kinda dubious.