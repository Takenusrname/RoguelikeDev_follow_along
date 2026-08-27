import json

def load_settings() -> dict:
    """reads"""

    settings_file = open("settings.json", mode="r", encoding="utf-8")
    loaded_settings = json.load(settings_file)

    return loaded_settings
