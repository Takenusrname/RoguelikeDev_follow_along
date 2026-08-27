"""I'm just having fun because I just wanted main to be `game.start()`.
In reality this should just be `main.py` but when you want to have some fun
on a personal project who can really stop you.
"""
import copy
import tcod
import tcod.tileset

from engine import Engine
import entity_factories
from procgen import generate_dungeon
from settings import load_settings

def start() -> None:
    """Start the game"""
    settings = load_settings()
    screen_width = settings['GameSettings']['screen_width']
    screen_height = settings['GameSettings']['screen_height']
    font = settings['GameSettings']['font_file']
    row_col_size = settings['GameSettings']['row_col']

    map_width = settings['MapSettings']['map_width']
    map_height = settings['MapSettings']['map_height']

    max_room_size = settings['MapSettings']['room_max_size']
    min_room_size = settings['MapSettings']['room_min_size']
    max_room_amt = settings['MapSettings']['max_rooms']

    max_mon_per_room = settings['MapSettings']['max_monsters_per_room']

    tset = tcod.tileset.load_tilesheet(
        font, row_col_size, row_col_size, tcod.tileset.CHARMAP_CP437
        )

    player = copy.deepcopy(entity_factories.player)

    engine = Engine(player=player)

    engine.game_map = generate_dungeon(
        max_rooms=max_room_amt,
        room_min_size=min_room_size,
        room_max_size=max_room_size,
        map_width=map_width,
        map_height=map_height,
        max_monsters_per_room=max_mon_per_room,
        engine=engine
    )

    engine.update_fov()

    root_con = tcod.console.Console(screen_width, screen_height, order='F')

    with tcod.context.new(
        console=root_con, tileset=tset, title="RLDEV 2026 Tutorial"
        ) as ctx:

        while True:
            engine.render(root_con, ctx)

            engine.event_handler.handle_events()
