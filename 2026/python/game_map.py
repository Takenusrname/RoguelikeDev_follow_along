"""Game Map"""
from __future__ import annotations

from typing import Iterable, Optional, TYPE_CHECKING
import numpy as np
from tcod.console import Console

import tile_types

if TYPE_CHECKING:
    from engine import Engine
    from entity import Entity


class GameMap:
    """Game Map Class that handles initialization and generation and rendering"""
    def __init__(self, engine: Engine, width: int, height: int, entities: Iterable[Entity] = ()):
        self.engine = engine
        self.width = width
        self.entities = set(entities)
        self.height = height
        self.tiles = np.full(
            (width, height), fill_value=tile_types.wall, order="F"
        )

        self.visible = np.full(
            (width, height), fill_value=False, order="F"
        )
        self.explored = np.full(
            (width, height), fill_value=False, order="F"
        )
        self.mask = np.full(
            (width, height), fill_value=0, order="F"
        )

    def get_blocking_entity_at_location(self, location_x: int, location_y:int) -> Optional[Entity]:
        for entity in self.entities:
            if entity.blocks_movement and entity.x == location_x and entity.y == location_y:
                return entity

        return None

    def in_bounds(self, x: int, y: int) -> bool:
        """Returns true if x and y are inside the bounds of this map."""
        return 0 <= x < self.width and 0 <= y < self.height

    def render(self, console: Console) -> None:
        """Renders the map.

        If a tile is in the `visible` array, then draw it with the `light` colors.
        If it isn't, but it's in the `explored` array, then draw it witht the `dark` colors.
        Otherwise, the default is `SHROUD`.
        """
        console.rgb[0:self.width, 0:self.height] = np.select(
            condlist=[self.visible, self.explored],
            choicelist=[self.tiles["light"], self.tiles["dark"]],
            default=tile_types.SHROUD
        )

        for entity in self.entities:
            if self.visible[entity.x, entity.y]:
                console.print(entity.x, entity.y, entity.char, fg=entity.fg_col, bg=entity.bg_col)

def wall_mask(game_map: GameMap):
    tile_mask = game_map.mask
    wall = game_map.tiles == tile_types.wall
#    if wall.all():
    if game_map.in_bounds is True and wall:
        for y in range(0, game_map.width):
            for x in range(0, game_map.height):
                inbounds = 0 <=x < game_map.width - 1 and 0 <= y < game_map.height-1
                c_wall = wall[x, y]
                if inbounds and c_wall:
                    if game_map.tiles[x, y - 1] == wall:
                        tile_mask += 1
                    if game_map.tiles[x, y + 1] == wall:
                        tile_mask += 2
                    if game_map.tiles[x - 1, y] == wall:
                        tile_mask += 4
                    if game_map.tiles[x + 1, y] == wall:
                        tile_mask += 8

"""
    match tile_mask:
        case 0: game_map.tiles["dark"]["ch"] = ord("○")
        case 1: game_map.tiles["dark"]["ch"] = ord("╨")
        case 2: game_map.tiles["dark"]["ch"] = ord("╥")
        case 3: game_map.tiles["dark"]["ch"] = ord("║")
        case 4: game_map.tiles["dark"]["ch"] = ord("╡")
        case 5: game_map.tiles["dark"]["ch"] = ord("╝")
        case 6: game_map.tiles["dark"]["ch"] = ord("╗")
        case 7: game_map.tiles["dark"]["ch"] = ord("╣")
        case 8: game_map.tiles["dark"]["ch"] = ord("╞")
        case 9: game_map.tiles["dark"]["ch"] = ord("╚")
        case 10: game_map.tiles["dark"]["ch"] = ord("╔")
        case 11: game_map.tiles["dark"]["ch"] = ord("╠")
        case 12: game_map.tiles["dark"]["ch"] = ord("═")
"""