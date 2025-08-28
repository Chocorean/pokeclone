# Description of fighting mechanisms

## Creatures

Creatures can be inspired from anything. They are made from a `base` (animal, concept, etc), and then declined by `elements` (1 for each?). They have several physical attributes, which grant them attacks (eg having claws implies "scratch" attack).

### Stats

Each creature have base stats, defined by its `base`: it will be the main source of variation for stat. The list being:

- hp
- attack
- defense
- speed

Each elemental variation brings a specific change:

|-----|---|---|
|Element|Stat|Change|
|-|-|-|
|Fire|Speed|+5%|
||Defense|-5%|
|Water|Dodge|+5%|
||Attack|-5%|
|Air|Attack|+5%|
||Accuracy|-5%|
|Earth|Defense|+5%|
||Speed|-5%|

Also strenghs and weaknesses:

|Mutiplier|Air|Earth|Fire|Water|
|-|-|-|-|-|
|Air|-|-|0.5x|2x|
|Earth|-|-|2x|0.5x|
|Fire|2x|0.5x|-|-|
|Water|0.5x|2x|-|-|


### Physical attacks

Depends on body parts and attributes. Have no accuracy (always hit), might be less efficient against other body parts (imagine kicklee trying to kick an ekans). Base 15% crit.

- `kick`: 

#### Body

|Body part|Moves|Strong against|Weak Against|Dmg|Effect|Target|
|-|-|-|-|-|-|-|
|claws|scratch||+wool|Y||All (A)|
|ears|listen|||N|+dodge|Self (S)|
|legs (2)|kick||-legs|Y||All but self (ABS)|
|legs (4)|charge|||Y||ABS|
|legs (any)|sprint|||N|+speed,-attack|S|
|teeth|bite|||Y||A|
|tongue|clean|||N|-debuff|A|
||lick|-wool + -hair|+hair, +wool|Y|-accuracy|ABS|

#### Elemental attacks

For now let's keep it simple. In the future, would be nice to unlock only a few attacks depending on the physical characteristics as well. An example would be fire only unlocks an attack if one has a tail, wings, or scales.

Air: gust,
Earth: earthquake,
Fire: spit flames,
Water: ice spikes.

### Evolving TBD

They can evolve under certain conditions? Transitions-between-sprites idea. stats tho

## Fights

Trainer fights are 3v3. Our team is on the left, opponent on the right. We select an action for each creature, then depending on stats, attacks and modifiers, each creature plays.

Wild encounters are 1v1.

At the beginning of any fight, the player is being asked to choose up to three creatures among the 5 they carry.

### Actions

For now let's say "Attack", which makes one choose the target first, then choose between body parts (eventually showing the modifiers). The other one is Magic, which offers to choose the target, showing the modifiers.
