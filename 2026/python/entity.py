"""Entity class that all game objects are made from"""
from __future__ import annotations

import copy
from typing import Optional, Tuple, TypeVar, TYPE_CHECKING
if TYPE_CHECKING:
    from game_map import GameMap

T = TypeVar("T", bound="Entity")


class Entity:
    """A generic object to represent players, enemies, items, etc.

    `x` and `y` positions on the map.

    `char` is the glyph that represents the entity.

    `fg_col` and `bg_col` are foreground and background colors of the entity
    """
    game_map: GameMap
    def __init__(
            self,
            game_map: Optional[GameMap] = None,
            x: int = 0,
            y: int = 0,
            char: str = "?",
            fg_col: Tuple[int, int, int] = (255,255,255),
            bg_col: Tuple[int, int, int] = (0, 0, 0),
            name: str = "<Unnamed>",
            blocks_movement: bool = False,
    ):
        self.x = x
        self.y = y
        self.char = char
        self.fg_col = fg_col
        self.bg_col = bg_col
        self.name = name
        self.blocks_movement = blocks_movement

        if game_map:
            self.game_map = game_map
            game_map.entities.add(self)

    def spawn(self: T, game_map: GameMap, x: int, y: int) -> T:
        """Spawns a copy of this instance at the given location."""
        clone = copy.deepcopy(self)
        clone.x = x
        clone.y = y
        clone.game_map = game_map
        game_map.entities.add(clone)
        return clone

    def place(self, x: int, y: int, game_map: Optional[GameMap] = None) -> None:
        self.x = x
        self.y = y
        if game_map:
            if hasattr(self, "gamemap"):
                self.game_map.entities.remove(self)
            self.game_map = game_map
            game_map.entities.add(self)

    def move(self, dx: int, dy: int) -> None:
        """Moves the `Entity` in the direction of `dx` and `dy`"""
        self.x += dx
        self.y += dy
