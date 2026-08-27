"""Takes inputs and turns them into actions"""
from __future__ import annotations
from typing import Optional, TYPE_CHECKING
import tcod.event
from actions import Action, BumpAction, EscapeAction

if TYPE_CHECKING:
    from engine import Engine


class EventHandler():
    """Class for Handling Events"""
    def __init__(self, engine: Engine):
        self.engine = engine

    def handle_events(self) -> None:
        for event in tcod.event.wait():
            action = self.on_event(event)

            if action is None:
                continue

            action.perform()
            self.engine.handle_enemy_turns()
            self.engine.update_fov()

    def on_event(self, event: tcod.event.Event) -> Optional[Action]:
        """Matches the event to an Action"""
        action: Optional[Action] = None

        player = self.engine.player

        match event:
            # Escape Actions
            case tcod.event.WindowEvent(type='WindowClose'):
                action = EscapeAction(player)

            case tcod.event.KeyDown(sym=tcod.event.KeySym.ESCAPE):
                action = EscapeAction(player)

            # Movement Actions
            case tcod.event.KeyDown(sym=tcod.event.KeySym.UP):
                action = BumpAction(player, dx=0,dy=-1)

            case tcod.event.KeyDown(sym=tcod.event.KeySym.DOWN):
                action = BumpAction(player, dx=0, dy=1)

            case tcod.event.KeyDown(sym=tcod.event.KeySym.LEFT):
                action = BumpAction(player, dx=-1, dy=0)

            case tcod.event.KeyDown(sym=tcod.event.KeySym.RIGHT):
                action = BumpAction(player, dx=1, dy=0)

        return action
