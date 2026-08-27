from entity import Entity

player = Entity(
    char="@",
    fg_col=(0,0,0),
    bg_col=(95, 205, 228),
    name="Player",
    blocks_movement=True
)

orc = Entity(char="o", fg_col=(63, 127, 63), bg_col=(34, 32, 52), name="Orc", blocks_movement=True)
troll = Entity(
    char="T",
    fg_col=(0, 127, 63),
    bg_col=(34, 32, 52),
    name="Troll",
    blocks_movement=True
)
