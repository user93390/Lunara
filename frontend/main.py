"""
An App to show the current time.
"""

from textual.app import App
from screen.login import LoginScreen

class LunaraApp(App):
    def on_mount(self) -> None:
        self.push_screen(LoginScreen())


if __name__ == "__main__":
    app = LunaraApp()
    app.run()
